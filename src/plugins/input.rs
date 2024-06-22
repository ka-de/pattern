use bevy::{ prelude::*, utils::HashMap };
use bevy_rapier2d::dynamics::Velocity;

use input_manager::{ plugin::InputManagerSystem, action_state::ActionData, prelude::* };
use std::time::Instant;

use crate::{
    components::{ climbing::Climber, swimming::Swimmer, ground::GroundDetection },
    entities::player::Player,
    plugins::{ dialogueview::not_in_dialogue, gamestate::GameState },
};

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
    Move,
    Jump,
    Interact,
}

pub(crate) type InputMap = input_manager::prelude::InputMap<Action>;
pub(crate) type ActionState = input_manager::prelude::ActionState<Action>;

pub(crate) fn make_action_map() -> InputMap {
    let dual_axis_pad = VirtualDPad::wasd()
        // You can configure a processing pipeline to handle axis-like user inputs.
        //
        // This step adds a circular deadzone that normalizes input values
        // by clamping their magnitude to a maximum of 1.0,
        // excluding those with a magnitude less than 0.1,
        // and scaling other values linearly in between.
        .with_circle_deadzone(0.1)
        // Followed by appending Y-axis inversion for the next processing step.
        .inverted_y()
        // Or reset the pipeline, leaving no any processing applied.
        .reset_processing_pipeline();

    InputMap::new([
        (Action::Jump, KeyCode::Space),
        (Action::Interact, KeyCode::KeyE),
    ]).with(Action::Move, dual_axis_pad)
}

// Velocity in px/s for full gamepad range
const AXIS_GAIN: f32 = 200.0;

#[derive(Copy, Clone, Debug)]
pub(crate) struct ActionTimer {
    pub count: u32,
    pub last_pressed: Instant,
}

#[derive(Component, Default, Clone)]
pub(crate) struct ActionTimers(pub HashMap<Action, ActionTimer>);

pub(crate) fn movement(
    mut query: Query<
        (
            &ActionState,
            &mut ActionTimers,
            &mut Velocity,
            &mut Climber,
            &mut Swimmer,
            &GroundDetection,
        ),
        With<Player>
    >,
    time: Res<Time<Real>>
) {
    for (
        action_state,
        mut timers,
        mut velocity,
        mut climber,
        mut swimmer,
        ground_detection,
    ) in &mut query {
        if
            let Some(ActionData { axis_pair: Some(axis_pair), state, .. }) =
                action_state.action_data(&Action::Move)
        {
            if climber.intersecting_climbables.is_empty() {
                climber.climbing = false;
            } else if state.just_pressed() && axis_pair.y().abs() > 0.0 {
                climber.climbing = true;
            }

            swimmer.swimming = !swimmer.intersecting_swimmables.is_empty();
            let axis_gain = if swimmer.swimming { AXIS_GAIN * 0.5 } else { AXIS_GAIN };

            velocity.linvel.x = axis_pair.x() * axis_gain;
            if climber.climbing {
                velocity.linvel.y = axis_pair.y() * axis_gain;
            } else if swimmer.swimming {
                velocity.linvel.y = (axis_pair.y() + 0.5) * axis_gain;
            }
        }

        if
            action_state.just_pressed(&Action::Jump) &&
            (ground_detection.on_ground || climber.climbing)
        {
            velocity.linvel.y = 500.0;
            climber.climbing = false;
        }

        for action in action_state.get_pressed() {
            let now = time.last_update().unwrap_or(Instant::now());
            timers.0
                .entry(action)
                .and_modify(|state| {
                    state.count += 1;
                    state.last_pressed = now;
                })
                .or_insert_with(|| ActionTimer {
                    count: 0,
                    last_pressed: now,
                });
        }
    }
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        use crate::components;

        // Plugins
        app.add_plugins((
            // Action
            InputManagerPlugin::<Action>::default(),
            // Slot
            InputManagerPlugin::<Slot>::default(),
            // Ability
            InputManagerPlugin::<Ability>::default(),
        ))

            // PreUpdate
            .add_systems(PreUpdate, (
                // This system coordinates the state of our two actions
                copy_action_state.after(InputManagerSystem::ManualControl),
                // ⚠️ NOTE: These systems run during PreUpdate.
                //
                // If you have systems that care about inputs and actions that also run during this stage,
                // you must define an ordering between your systems or behavior will be very erratic.
                // The stable system sets for these systems are available under InputManagerSystem enum.
                movement
                    .run_if(not_in_dialogue.and_then(in_state(GameState::Playing)))
                    .after(components::ground::update_on_ground)
                    .after(components::climbing::detect_climb_range)
                    .after(components::swimming::detect_swim_range),
            ));
    }
}

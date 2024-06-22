use bevy::{
    app::{ App, Plugin, PreUpdate, Update },
    core::Name,
    ecs::{
        bundle::Bundle,
        component::Component,
        schedule::{ common_conditions::in_state, Condition, IntoSystemConfigs as _ },
        system::{ Query, Res },
    },
    input::{ gamepad::GamepadButtonType, keyboard::KeyCode, mouse::MouseButton },
    log::info,
    reflect::Reflect,
    time::{ Real, Time },
    utils::default,
};
use bevy_rapier2d::dynamics::Velocity;
use input_manager::{
    input_map::InputMap,
    action_state::{ ActionData, ActionState },
    axislike::VirtualDPad,
    input_processing::WithDualAxisProcessingPipelineExt,
    plugin::{ InputManagerPlugin, InputManagerSystem },
};
pub use input_manager::Actionlike;

use std::collections::HashMap;
use std::time::Instant;

use crate::components::{ climbing::Climber, ground::GroundDetection, swimming::Swimmer };

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
    Move,
    Jump,
    Interact,
    Ability(u8),
}

impl Action {
    const PRIMARY_ACTION: Self = Self::Ability(0);
    const SECONDARY_ACTION: Self = Self::Ability(1);
}

#[derive(Bundle, Clone)]
pub struct InputBundle<Ability: Actionlike> {
    input_map: InputMap<Action>,
    action_state: ActionState<Action>,
    // We do not need an InputMap<Ability> component,
    // as abilities are never triggered directly from inputs.
    ability_action_state: ActionState<Ability>,
    ability_slot_map: AbilitySlotMap<Ability>,
    action_timers: ActionTimers,
}

/// This struct stores which ability corresponds to which slot for a particular player
#[derive(Component, Debug, Clone)]
pub struct AbilitySlotMap<Ability> {
    map: HashMap<u8, Ability>,
}

// Fix wrong trait bounds on default
impl<A> Default for AbilitySlotMap<A> {
    fn default() -> Self {
        Self { map: Default::default() }
    }
}
impl<A: Actionlike> Default for InputBundle<A> {
    fn default() -> Self {
        Self {
            input_map: default(),
            action_state: default(),
            ability_action_state: default(),
            ability_slot_map: default(),
            action_timers: default(),
        }
    }
}

pub(crate) fn make_action_map<Ability: Actionlike>(
    abilities: HashMap<u8, Ability>
) -> InputBundle<Ability> {
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

    let input_map = InputMap::new([
        (Action::Jump, KeyCode::Space),
        (Action::Interact, KeyCode::KeyE),
        (Action::PRIMARY_ACTION, KeyCode::KeyQ),
        (Action::SECONDARY_ACTION, KeyCode::KeyW),
        (Action::Ability(2), KeyCode::KeyR),
        (Action::Ability(3), KeyCode::KeyT),
        (Action::Ability(4), KeyCode::KeyZ),
        (Action::Ability(5), KeyCode::KeyX),
        (Action::Ability(6), KeyCode::KeyC),
        (Action::Ability(7), KeyCode::KeyV),
    ])
        .with(Action::Move, dual_axis_pad)
        .with(Action::Interact, GamepadButtonType::RightTrigger2)
        .with(Action::Jump, MouseButton::Left)
        .with(Action::Jump, GamepadButtonType::LeftTrigger)
        .with(Action::PRIMARY_ACTION, MouseButton::Right)
        .with(Action::PRIMARY_ACTION, GamepadButtonType::RightTrigger)
        .with(Action::SECONDARY_ACTION, GamepadButtonType::LeftTrigger2)
        .with(Action::Ability(2), GamepadButtonType::East) // PS: Circle, Xbox: B
        .with(Action::Ability(3), GamepadButtonType::North) // PS: Triangle, Xbox: Y
        .with(Action::Ability(4), GamepadButtonType::West) // PS: Square, Xbox: X
        .with(Action::Ability(5), GamepadButtonType::South) // PS: Cross, Xbox: A
        .with(Action::Ability(6), GamepadButtonType::C)
        .with(Action::Ability(7), GamepadButtonType::Z);

    InputBundle {
        input_map,
        ability_slot_map: AbilitySlotMap { map: abilities },
        ..default()
    }
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

/// System that handle input and movement
pub(crate) fn movement(
    mut query: Query<
        (
            &ActionState<Action>,
            &mut ActionTimers,
            &mut Velocity,
            &mut Climber,
            &mut Swimmer,
            &GroundDetection,
        )
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

fn copy_ability_action_state<Ability: Actionlike + Copy + Clone + std::fmt::Debug>(
    mut query: Query<
        (&mut ActionState<Action>, &mut ActionState<Ability>, &AbilitySlotMap<Ability>)
    >
) {
    for (mut action_state, mut ability_state, ability_slot_map) in query.iter_mut() {
        for (&slot, &ability) in ability_slot_map.map.iter() {
            ability_state.set_action_data(
                ability,
                action_state.action_data_mut_or_default(&Action::Ability(slot)).clone()
            );
        }
    }
}

fn report_abilities_used<Ability: Actionlike + std::fmt::Debug>(
    query: Query<(&Name, &ActionState<Ability>)>
) {
    for (name, ability_state) in query.iter() {
        for ability in ability_state.get_just_pressed() {
            info!("{} used {:?}", name, ability);
        }
    }
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        use crate::{
            components,
            entities::player, // FIXME: we should remove this dependency
            plugins::{ dialogueview::not_in_dialogue, gamestate::GameState },
        };

        // Plugins
        app.add_plugins((
            // Action
            InputManagerPlugin::<Action>::default(),
            //InputManagerPlugin::<player::Ability>::default(), // I don't think we need this
        ))

            // PreUpdate: copy action state from the main action state to the ability one/
            .add_systems(
                PreUpdate,
                copy_ability_action_state::<player::Ability>.after(
                    InputManagerSystem::ManualControl
                )
            )
            // Update: runs systems consuming the inputs
            .add_systems(
                Update,
                (movement, report_abilities_used::<player::Ability>)
                    .run_if(not_in_dialogue.and_then(in_state(GameState::Playing)))
                    .after(components::ground::update_on_ground)
                    .after(components::climbing::detect_climb_range)
                    .after(components::swimming::detect_swim_range)
            );
    }
}

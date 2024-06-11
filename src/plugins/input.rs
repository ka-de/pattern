use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::{
    ecs::{query::With, system::Query, system::Res},
    input::{keyboard::KeyCode, ButtonInput},
};
use bevy_rapier2d::dynamics::Velocity;
use std::collections::HashMap;
use std::time::Instant;

use crate::components::ladders::Climber;
use crate::components::player::Player;
use crate::components::{ground::GroundDetection, water::Swimmer};

use super::{dialogueview::not_in_dialogue, gamestate::GameState};

#[derive(Debug)]
pub(crate) struct KeyPressState {
    pub count: u32,
    pub last_pressed: Instant,
}

#[derive(Resource, Default)]
pub(crate) struct KeyPressStates(pub HashMap<KeyCode, KeyPressState>);

pub(crate) fn movement(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Climber, &mut Swimmer, &GroundDetection), With<Player>>,
) {
    for (mut velocity, mut climber, mut swimmer, ground_detection) in &mut query {
        let right = if input.pressed(KeyCode::KeyD) {
            1.0
        } else {
            0.0
        };
        let left = if input.pressed(KeyCode::KeyA) {
            1.0
        } else {
            0.0
        };

        velocity.linvel.x = (right - left) * 200.0;

        if climber.intersecting_climbables.is_empty() {
            climber.climbing = false;
        } else if input.just_pressed(KeyCode::KeyW) || input.just_pressed(KeyCode::KeyS) {
            climber.climbing = true;
        }

        if climber.climbing {
            let up = if input.pressed(KeyCode::KeyW) {
                1.0
            } else {
                0.0
            };
            let down = if input.pressed(KeyCode::KeyS) {
                1.0
            } else {
                0.0
            };

            velocity.linvel.y = (up - down) * 200.0;
        }

        if input.just_pressed(KeyCode::Space) && (ground_detection.on_ground || climber.climbing) {
            velocity.linvel.y = 500.0;
            climber.climbing = false;
        }

        if swimmer.intersecting_swimmables.is_empty() {
            swimmer.swimming = false;
        } else {
            swimmer.swimming = true;
            velocity.linvel.x /= 2.0;
            velocity.linvel.y /= 2.0;
        }
    }
}

pub(crate) fn handle_keypress(
    time: Res<Time<Real>>,
    mut keyboard_input: EventReader<KeyboardInput>,
    mut key_press_states: ResMut<KeyPressStates>,
) {
    if keyboard_input.is_empty() {
        return;
    }
    let now = time.last_update().unwrap_or(Instant::now());
    for event in keyboard_input.read() {
        let is_pressed = event.state == ButtonState::Pressed;
        let key_code = event.key_code;
        if is_pressed {
            key_press_states
                .0
                .entry(key_code)
                .and_modify(|state| {
                    state.count += 1;
                    state.last_pressed = now;
                })
                .or_insert_with(|| KeyPressState {
                    count: 0,
                    last_pressed: now,
                });
        }
    }
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KeyPressStates>().add_systems(
            Update,
            (
                handle_keypress,
                movement.run_if(not_in_dialogue.and_then(in_state(GameState::Playing))),
            )
                .chain(),
        );
    }
}

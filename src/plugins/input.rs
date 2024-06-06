use bevy::{ input::keyboard::KeyboardInput, time::Stopwatch };
use bevy::input::ButtonState;
use bevy::prelude::*;
use std::collections::HashMap;
use bevy::{
    ecs::{ query::With, system::Query, system::Res },
    input::{ keyboard::KeyCode, ButtonInput },
};
use bevy_rapier2d::dynamics::Velocity;

use crate::components::ground::GroundDetection;
use crate::components::player::Player;
use crate::components::ladders::Climber;

#[derive(Resource, Default)]
pub(crate) struct KeyPressState {
    counts: HashMap<KeyCode, u32>,
    last_pressed: HashMap<KeyCode, bool>,
}

#[derive(Resource, Default)]
pub(crate) struct KeyPressTimers {
    timers: HashMap<KeyCode, Stopwatch>,
}

pub(crate) fn movement(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Climber, &GroundDetection), With<Player>>
) {
    for (mut velocity, mut climber, ground_detection) in &mut query {
        let right = if input.pressed(KeyCode::KeyD) { 1.0 } else { 0.0 };
        let left = if input.pressed(KeyCode::KeyA) { 1.0 } else { 0.0 };

        velocity.linvel.x = (right - left) * 200.0;

        if climber.intersecting_climbables.is_empty() {
            climber.climbing = false;
        } else if input.just_pressed(KeyCode::KeyW) || input.just_pressed(KeyCode::KeyS) {
            climber.climbing = true;
        }

        if climber.climbing {
            let up = if input.pressed(KeyCode::KeyW) { 1.0 } else { 0.0 };
            let down = if input.pressed(KeyCode::KeyS) { 1.0 } else { 0.0 };

            velocity.linvel.y = (up - down) * 200.0;
        }

        if input.just_pressed(KeyCode::Space) && (ground_detection.on_ground || climber.climbing) {
            velocity.linvel.y = 500.0;
            climber.climbing = false;
        }
    }
}

pub(crate) fn handle_keypress(
    mut keyboard_input: EventReader<KeyboardInput>,
    mut key_press_state: ResMut<KeyPressState>,
    mut key_press_timers: ResMut<KeyPressTimers>
) {
    for event in keyboard_input.read() {
        let is_pressed = event.state == ButtonState::Pressed;
        let key_code = event.key_code;

        if is_pressed && !key_press_state.last_pressed.contains_key(&key_code) {
            *key_press_state.counts.entry(key_code).or_insert(0) += 1;
            key_press_timers.timers
                .entry(key_code)
                .or_insert_with(|| Stopwatch::new())
                .reset();
        }

        key_press_state.last_pressed.insert(key_code, is_pressed);
    }
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KeyPressState>()
            .init_resource::<KeyPressTimers>()
            .add_systems(Update, (handle_keypress, movement).chain());
    }
}

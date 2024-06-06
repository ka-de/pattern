use bevy::{
    ecs::{ query::With, system::{ Query, Res } },
    input::{ keyboard::KeyCode, ButtonInput },
};
use bevy_rapier2d::dynamics::Velocity;

use super::{ ground::GroundDetection, ladders::Climber, player::Player };

pub fn movement(
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

use bevy::prelude::*;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// 💀 Zone
#[derive(Component)]
pub struct DeathZone {
    pub bounds: Collider,
}

impl DeathZone {
    pub fn new(width: f32, height: f32) -> Self {
        DeathZone {
            bounds: Collider::cuboid(width / 2.0, height / 2.0),
        }
    }
}

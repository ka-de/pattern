use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct DeathZone;

#[derive(Bundle)]
pub struct DeathZoneBundle {
    pub death_zone: DeathZone,
    pub collider: Collider,
    pub sensor: Sensor,
    pub transform: Transform,
}

impl DeathZoneBundle {
    pub fn new(width: f32, height: f32, position: Vec3) -> Self {
        DeathZoneBundle {
            death_zone: DeathZone,
            collider: Collider::cuboid(width / 2.0, height / 2.0),
            sensor: Sensor,
            transform: Transform::from_translation(position),
        }
    }
}

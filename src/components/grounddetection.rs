use bevy::prelude::*;

#[derive(Clone, Default, Component)]
pub struct GroundDetection {
    pub on_ground: bool,
}

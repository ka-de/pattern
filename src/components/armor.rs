use bevy::prelude::*;

#[derive(Component, Clone, Default)]
pub struct Armor {
    pub value: u32, // The armor value of the entity
}

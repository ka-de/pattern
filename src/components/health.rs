use bevy::ecs::component::Component;

use super::armor::Armor;

// 🩸
#[derive(Component, Clone)]
pub struct Health {
    pub current: u32, // The current health of the entity
    pub max: u32, // The maximum health of the entity
    pub hunger: u32, // The hunger of the entity
}

impl Default for Health {
    fn default() -> Self {
        Self {
            current: 100, // default value for current health
            max: 100, // default value for max health
            hunger: 0, // default value for hunger
        }
    }
}

impl Health {
    pub fn take_damage(&mut self, mut damage: u32, armor: Option<&Armor>) {
        if let Some(armor) = armor {
            let reduction = ((armor.value as f32) / 100.0) * (damage as f32);
            damage -= reduction.ceil() as u32;
        }

        self.current = self.current.saturating_sub(damage);
    }
}

use bevy::{ ecs::component::Component, reflect::Reflect };

use super::armor::Armor;

// 🩸
#[derive(Component, Clone, Reflect)]
pub struct Health {
    pub current: u32,
    pub max: u32,
    pub hunger: u32,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            current: 100,
            max: 100,
            hunger: 0,
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

use bevy::ecs::component::Component;

// 🩸
#[derive(Component)]
pub struct Health {
    pub current: u32, // The current health of the entity
    pub max: u32, // The maximum health of the entity
    pub hunger: u32, // The hunger of the entity
}

use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Armor {
    pub value: u32, // The armor value of the entity
}

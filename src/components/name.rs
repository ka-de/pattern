use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Name(&'static str);

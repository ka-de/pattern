use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Name(pub &'static str);

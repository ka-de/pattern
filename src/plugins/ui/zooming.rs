use bevy::{
    ecs::{ component::Component, entity::Entity, query::With, system::{ Commands, Query, Res } },
    time::Time,
};

#[derive(Component)]
pub struct Zooming;

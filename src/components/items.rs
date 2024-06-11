use bevy::{
    input::{ ButtonInput, keyboard::KeyCode },
    ecs::{ component::Component, system::Res, system::Query, query::With },
};
use bevy_ecs_ldtk::{ ldtk::ldtk_fields::LdtkFields, EntityInstance };

use crate::components::player::Player;

#[derive(Clone, Component, Debug, Eq, Default, PartialEq)]
pub struct Items(Vec<String>);

impl From<&EntityInstance> for Items {
    fn from(entity_instance: &EntityInstance) -> Self {
        Items(
            entity_instance
                .iter_enums_field("items")
                .expect("items field should be correctly typed")
                .cloned()
                .collect()
        )
    }
}

pub fn dbg_player_items(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Items, &EntityInstance), With<Player>>
) {
    for (items, entity_instance) in &mut query {
        if input.just_pressed(KeyCode::KeyP) {
            dbg!(&items);
            dbg!(&entity_instance);
        }
    }
}

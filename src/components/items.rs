use bevy::prelude::*;
use bevy_ecs_ldtk::{ ldtk::ldtk_fields::LdtkFields, EntityInstance };

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

use bevy::{ ecs::{ bundle::Bundle, component::Component }, sprite::SpriteSheetBundle };
use bevy_ecs_ldtk::prelude::LdtkEntity;

use super::{ ColliderBundle, PredefinedPath };
use crate::components::animals::Animal;

// 🐈‍⬛
#[derive(Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Cat {
    name: String,
}

// Implement methods for the 'Cat' struct
impl Cat {
    // Define a new function that takes a String as an argument and returns a new instance of 'Cat'
    fn new(name: String) -> Self {
        // Return a new 'Cat' instance with the given name
        Self { name }
    }
}

// Implement the 'Animal' trait for the 'Cat' struct
impl Animal for Cat {
    // Define the 'species' method to return the static string "Cat"
    fn species() -> &'static str {
        "Cat"
    }

    // Define the 'name' method to return a reference to the 'name' field of the 'Cat' instance
    fn name(&self) -> &String {
        &self.name
    }
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct CatBundle {
    #[sprite_sheet_bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    pub cat: Cat,
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct CatPatrolBundle {
    #[ldtk_entity]
    pub cat: CatBundle,
    #[ldtk_entity]
    pub predefined_path: PredefinedPath,
}

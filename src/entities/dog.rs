use bevy::{ ecs::{ bundle::Bundle, component::Component }, sprite::SpriteSheetBundle };
use bevy_ecs_ldtk::prelude::LdtkEntity;

use super::{ ColliderBundle, PredefinedPath };
use crate::components::animals::Animal;

// 🐕
#[derive(Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Dog {
    name: String,
}

// Implement methods for the 'Dog' struct
impl Dog {
    // Define a new function that takes a String as an argument and returns a new instance of 'Dog'
    fn new(name: String) -> Self {
        // Return a new 'Dog' instance with the given name
        Self { name }
    }
}

// Implement the 'Animal' trait for the 'Dog' struct
impl Animal for Dog {
    // Define the 'species' method to return the static string "Dog"
    fn species() -> &'static str {
        "Dog"
    }

    // Define the 'name' method to return a reference to the 'name' field of the 'Dog' instance
    fn name(&self) -> &String {
        &self.name
    }
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct DogBundle {
    #[sprite_sheet_bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    pub dog: Dog,
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct DogPatrolBundle {
    #[ldtk_entity]
    pub dog: DogBundle,
    #[ldtk_entity]
    pub predefined_path: PredefinedPath,
}

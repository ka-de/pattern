use super::{
    names,
    AnimationIndices,
    AnimationTimer,
    DeathAnimationPlayed,
    GravityScale,
    Health,
    Velocity,
};

use rand::thread_rng;
use rand::Rng;

use bevy::prelude::*;

// Keeps track of the facing direction of each animal.
#[derive(Component, Clone, Copy)]
pub struct FacingDirection {
    pub x: f32,
}

// Animal types
#[derive(Component, PartialEq, Eq)]
pub enum AnimalType {
    Cat,
    Dog,
}

// 🐾
pub trait Animal: Component {
    // A static method that returns a static string reference, representing the species of the animal
    fn species() -> &'static str;

    // A method that returns a reference to a String, representing the name of the animal
    fn name(&self) -> &String;

    // A method that returns an Option containing a static string reference, representing the gender of the animal
    fn gender(&self) -> Option<&'static str> {
        // Get the name of the animal
        let name = self.name();

        // Iterate over the ANIMAL_NAMES array
        for &(animal_name, gender, _) in names::ANIMAL_NAMES {
            // If the name of the animal matches the name in the array
            if animal_name == name {
                // Return the gender of the animal
                return Some(gender);
            }
        }

        // If no match is found, return None
        None
    }
}

// 🐈‍⬛
#[derive(Component)]
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

// 🐕
#[derive(Component)]
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

// Function to generate an animal name based on the given animal type
fn generate_animal_name(animal_type: AnimalType) -> String {
    // Create a random number generator
    let mut rng = thread_rng();

    // Choose a random animal name from the ANIMAL_NAMES array
    // The chosen element is a tuple containing the name, gender, and type of the animal
    let (name, _gender, name_type) = ANIMAL_NAMES.choose(&mut rng).unwrap();

    // If the type of the chosen animal matches the given animal type
    if *name_type == animal_type {
        // Return the name of the animal as a string
        (*name).to_string()
    } else {
        // If the types don't match, recursively call the function until a matching animal name is found
        generate_animal_name(animal_type)
    }
}

// Function to spawn an animal of type T, where T is a type that implements the Animal trait
fn spawn_animal<T: Animal>(
    commands: &mut Commands, // Commands to spawn entities and components
    asset_server: &AssetServer, // Asset server to load assets
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>, // Texture atlas layouts for sprite animation
    animal_type: AnimalType, // Type of the animal to spawn
    texture_path: String, // Path to the texture of the animal
    velocity: Velocity, // Velocity of the animal
    animal_factory: fn(String) -> T // Factory function to create an instance of the animal
) {
    // Create a random number generator
    let mut rng = thread_rng();
    // Generate a random x-coordinate for the animal
    let x = rng.gen_range(-25.0..25.0);

    let facing_direction = FacingDirection {
        x: velocity.x.signum(),
    };

    // Generate a name for the animal
    let animal_name = generate_animal_name(animal_type);
    // Load the texture of the animal
    let animal_texture = asset_server.load(&texture_path);
    // Create a texture atlas layout for the animal
    let animal_layout = TextureAtlasLayout::from_grid(
        Vec2::new(26.0, 26.0),
        4,
        4,
        Some(Vec2::new(2.0, 2.0)),
        None
    );
    // Add the texture atlas layout to the assets
    let animal_texture_atlas_layout = texture_atlas_layouts.add(animal_layout);
    // Define the indices for the animal's animation
    let animal_animation_indices = AnimationIndices {
        first: 0,
        last: 3,
        current_index: 0,
    }; // idle animation
    let facing_direction_clone = facing_direction;
    let animal_entity = commands
        .spawn((
            animal_factory(animal_name.clone()),
            Health {
                current: 100,
                max: 100,
                hunger: 100,
            },
            SpriteSheetBundle {
                texture: animal_texture.clone(),
                atlas: TextureAtlas {
                    layout: animal_texture_atlas_layout,
                    index: animal_animation_indices.first,
                },
                transform: Transform::from_xyz(x, 50.0, 0.0),
                ..Default::default()
            },
            AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
            animal_animation_indices.clone(),
            velocity,
            DeathAnimationPlayed(false),
            GravityScale(1.0),
            Name::new(animal_name.clone()),
            facing_direction_clone, // Clone the facing_direction
        ))
        .with_children(|parent| {
            parent.spawn(Text2dBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: animal_name.clone(),
                        style: TextStyle {
                            font: asset_server.load("fonts/bahnschrift.ttf"),
                            font_size: 9.0,
                            color: Color::WHITE,
                        },
                    }],
                    ..Default::default()
                },
                transform: Transform::from_xyz(0.0, 10.0, 1.0),
                ..Default::default()
            });
        });
}

// Function to spawn a cat
pub fn spawn_cat(
    commands: &mut Commands, // Commands to spawn entities and components
    asset_server: &AssetServer, // Asset server to load assets
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout> // Texture atlas layouts for sprite animation
) {
    // Call the spawn_animal function with the parameters for a cat
    spawn_animal::<Cat>(
        commands,
        asset_server,
        texture_atlas_layouts,
        AnimalType::Cat,
        "textures/cat-texture.png".to_string(),
        Velocity { x: 15.0, y: 0.0 },
        Cat::new
    );
}

// Function to spawn a dog
pub fn spawn_dog(
    commands: &mut Commands, // Commands to spawn entities and components
    asset_server: &AssetServer, // Asset server to load assets
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout> // Texture atlas layouts for sprite animation
) {
    // Call the spawn_animal function with the parameters for a dog
    spawn_animal::<Dog>(
        commands,
        asset_server,
        texture_atlas_layouts,
        AnimalType::Dog,
        "textures/dog-texture.png".to_string(),
        Velocity { x: -2.0, y: 0.0 },
        Dog::new
    );
}

use crate::components::{
    AnimationIndices,
    AnimationTimer,
    DeathAnimationPlayed,
    GravityScale,
    Health,
    Velocity,
};

use rand::seq::SliceRandom;
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
enum AnimalType {
    Dog,
    Cat,
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
        for &(animal_name, gender, _) in ANIMAL_NAMES {
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
        name.to_string()
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
    let facing_direction_clone = facing_direction.clone();
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
        .id();

    commands.entity(animal_entity).with_children(|parent| {
        let text_transform = if facing_direction_clone.x < 0.0 {
            Transform::from_xyz(0.0, 10.0, 1.0).with_scale(Vec3::new(-1.0, 1.0, 1.0))
        } else {
            Transform::from_xyz(0.0, 10.0, 1.0)
        };

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
            transform: text_transform,
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

#[rustfmt::skip]
const ANIMAL_NAMES: &[(&str, &str, AnimalType)] = &[
    ("Malcolm", "male", AnimalType::Dog), ("Zoe", "female", AnimalType::Dog), ("Wash", "male", AnimalType::Dog),
    ("Inara", "female", AnimalType::Dog), ("Jayne", "male", AnimalType::Dog), ("Kaylee", "female", AnimalType::Dog),
    ("Simon", "male", AnimalType::Dog), ("River", "female", AnimalType::Dog), ("Book", "male", AnimalType::Dog),
    ("Saffron", "female", AnimalType::Dog), ("Badger", "male", AnimalType::Dog), ("Nandi", "female", AnimalType::Dog),
    ("Bester", "male", AnimalType::Dog), ("Dobson", "male", AnimalType::Dog), ("Atherton", "male", AnimalType::Dog),
    ("Gabriel", "male", AnimalType::Dog), ("Regan", "female", AnimalType::Dog), ("Tracey", "male", AnimalType::Dog),
    ("Amnon", "male", AnimalType::Dog), ("Fess", "male", AnimalType::Dog), ("Rance", "male", AnimalType::Dog),
    ("Magistrate", "male", AnimalType::Dog), ("Lucy", "female", AnimalType::Dog), ("Ruth", "female", AnimalType::Dog),
    ("Bree", "female", AnimalType::Dog), ("Jubal", "male", AnimalType::Dog), ("Fanty", "male", AnimalType::Dog),
    ("Mingo", "male", AnimalType::Dog), ("Durin", "male", AnimalType::Dog), ("Bridget", "female", AnimalType::Dog),
    ("Matty", "male", AnimalType::Dog), ("Ranse", "male", AnimalType::Dog), ("Heinrich", "male", AnimalType::Dog),
    ("Lawrence", "male", AnimalType::Dog), ("Lund", "male", AnimalType::Dog), ("Monty", "male", AnimalType::Dog),
    ("Corbin", "male", AnimalType::Dog), ("Petaline", "female", AnimalType::Dog), ("Helen", "female", AnimalType::Dog),
    ("Fanti", "male", AnimalType::Dog), ("Kess", "female", AnimalType::Dog), ("Ransome", "male", AnimalType::Dog),
    ("Sanda", "female", AnimalType::Dog), // End of 🐕
    ("Picard", "male", AnimalType::Cat), ("Beverly", "female", AnimalType::Cat), ("Data", "male", AnimalType::Cat),
    ("Troi", "female", AnimalType::Cat), ("Laforge", "male", AnimalType::Cat), ("Crusher", "male", AnimalType::Cat),
    ("Yar", "female", AnimalType::Cat), ("Kirk", "male", AnimalType::Cat), ("Spock", "male", AnimalType::Cat),
    ("Mccoy", "male", AnimalType::Cat), ("Scotty", "male", AnimalType::Cat), ("Uhura", "female", AnimalType::Cat),
    ("Sulu", "male", AnimalType::Cat), ("Chekov", "male", AnimalType::Cat), ("Chakotay", "male", AnimalType::Cat),
    ("Tuvok", "male", AnimalType::Cat), ("Sisko", "male", AnimalType::Cat), ("Kira", "female", AnimalType::Cat),
    ("Dax", "female", AnimalType::Cat), ("Bashir", "male", AnimalType::Cat), ("Odo", "male", AnimalType::Cat),
    ("Quark", "male", AnimalType::Cat), ("Archer", "male", AnimalType::Cat), ("Tucker", "male", AnimalType::Cat),
    ("Tpol", "female", AnimalType::Cat), ("Reed", "male", AnimalType::Cat), ("Mayweather", "male", AnimalType::Cat),
    ("Phlox", "male", AnimalType::Cat), ("Sato", "female", AnimalType::Cat), ("Sevenofnine", "female", AnimalType::Cat),
    ("Doctor", "male", AnimalType::Cat), ("Paris", "male", AnimalType::Cat), ("Harrykim", "male", AnimalType::Cat),
    ("Belanna", "female", AnimalType::Cat), ("Torres", "female", AnimalType::Cat), ("Jeanluc", "male", AnimalType::Cat),
    ("Lorca", "male", AnimalType::Cat), ("Burnham", "female", AnimalType::Cat), ("Saru", "male", AnimalType::Cat),
    ("Stamets", "male", AnimalType::Cat), ("Tilly", "female", AnimalType::Cat), ("Georgiou", "female", AnimalType::Cat), 
    ("Culber", "male", AnimalType::Cat), ("Cornwell", "female", AnimalType::Cat), ("Leland", "male", AnimalType::Cat),
    ("Vance", "male", AnimalType::Cat), ("Reno", "female", AnimalType::Cat), ("Booker", "male", AnimalType::Cat),
    ("Grudge", "female", AnimalType::Cat), ("Shaxs", "male", AnimalType::Cat), ("Detmer", "female", AnimalType::Cat),
    ("Owosekun", "female", AnimalType::Cat), ("Rhys", "male", AnimalType::Cat), ("Pike", "male", AnimalType::Cat),
    ("Number One", "male", AnimalType::Cat), ("Laan", "male", AnimalType::Cat), ("Chapel", "female", AnimalType::Cat), 
    ("Kyle", "male", AnimalType::Cat), ("Vina", "female", AnimalType::Cat), ("Mudd", "male", AnimalType::Cat),
    ("Garak", "male", AnimalType::Cat), ("Leyton", "male", AnimalType::Cat), ("Ross", "male", AnimalType::Cat),
    ("Nog", "male", AnimalType::Cat), ("Jake", "male", AnimalType::Cat), ("Seven", "female", AnimalType::Cat),
    ("Janeway", "female", AnimalType::Cat), ("Tuvix", "male", AnimalType::Cat), ("Neelix", "male", AnimalType::Cat),
    ("Kes", "female", AnimalType::Cat), ("Carey", "male", AnimalType::Cat), ("Vorik", "male", AnimalType::Cat),
    ("Wildman", "female", AnimalType::Cat), ("Zahir", "male", AnimalType::Cat), ("Seska", "female", AnimalType::Cat),
    ("Jonas", "male", AnimalType::Cat), ("Rio", "male", AnimalType::Cat), ("Maxwell", "male", AnimalType::Cat),
    ("Tryla", "female", AnimalType::Cat), ("Lorian", "male", AnimalType::Cat), ("Icheb", "male", AnimalType::Cat), 
    ("Q", "male", AnimalType::Cat), ("Guinan", "female", AnimalType::Cat), ("Pulaski", "female", AnimalType::Cat),
    ("Ro", "female", AnimalType::Cat), ("Hwomyn", "female", AnimalType::Cat), ("Riker", "male", AnimalType::Cat),
    ("Shelby", "female", AnimalType::Cat), ("Obrien", "male", AnimalType::Cat), ("Keiko", "female", AnimalType::Cat),
    ("Molly", "female", AnimalType::Cat), ("Kirayoshi", "male", AnimalType::Cat), ("Naomi", "female", AnimalType::Cat),
    ("Ezri", "female", AnimalType::Cat), ("Kassidy", "female", AnimalType::Cat), ("Leeta", "female", AnimalType::Cat), 
    ("Nog", "male", AnimalType::Cat), ("Rom", "male", AnimalType::Cat), ("Brunt", "male", AnimalType::Cat),
    ("Ishka", "female", AnimalType::Cat), ("Worf", "male", AnimalType::Cat), ("Martok", "male", AnimalType::Cat),
    ("Grilka", "female", AnimalType::Cat), ("Sharan", "male", AnimalType::Cat), ("Alexander", "male", AnimalType::Cat), 
    ("Kehleyr", "female", AnimalType::Cat), ("Lwaxana", "female", AnimalType::Cat), ("Kamala", "female", AnimalType::Cat),
    ("Vash", "female", AnimalType::Cat), ("Tasha", "female", AnimalType::Cat), ("Ogawa", "female", AnimalType::Cat),
    ("Barclay", "male", AnimalType::Cat), ("Maddox", "male", AnimalType::Cat), ("Soong", "male", AnimalType::Cat),
    ("Juliana", "female", AnimalType::Cat), ("Sela", "female", AnimalType::Cat), ("Toral", "male", AnimalType::Cat),
    ("Ziyal", "female", AnimalType::Cat), ("Dukat", "male", AnimalType::Cat), ("Damar", "male", AnimalType::Cat), 
    ("Weyoun", "male", AnimalType::Cat), ("Eddington", "male", AnimalType::Cat), ("Michael", "male", AnimalType::Cat),
    ("Sarina", "female", AnimalType::Cat), ("Hugh", "male", AnimalType::Cat), ("Lore", "male", AnimalType::Cat),
    ("Elaurian", "male", AnimalType::Cat), // End of 🐈‍⬛
];

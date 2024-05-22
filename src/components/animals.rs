use crate::components::{
    AnimationIndices, AnimationTimer, DeathAnimationPlayed, GravityScale, Health, Velocity,
};

use rand::seq::SliceRandom;
use rand::thread_rng;

use bevy::prelude::*;

/**
 * A lot of names for the 🐕 and 🐈‍⬛.
 */
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
    ("Bree", "female", AnimalType::Dog), // End of 🐕
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
    ("Thedoctor", "male", AnimalType::Cat), ("Tomparis", "male", AnimalType::Cat), ("Harrykim", "male", AnimalType::Cat),
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
    ("Elaurian", "male", AnimalType::Cat) // End of 🐈‍⬛
];

#[derive(Component, PartialEq, Eq)]
enum AnimalType {
    Dog,
    Cat,
}

fn generate_animal_name(animal_type: AnimalType) -> String {
    let mut rng = thread_rng();
    let (name, _gender, name_type) = ANIMAL_NAMES.choose(&mut rng).unwrap();
    if *name_type == animal_type {
        name.to_string()
    } else {
        generate_animal_name(animal_type)
    }
}

/**
 * The 🐾 struct.
 */
pub trait Animal {
    fn species() -> &'static str;
    fn name(&self) -> &String;
    fn gender(&self) -> Option<&'static str> {
        let name = self.name();
        for &(animal_name, gender, _) in ANIMAL_NAMES {
            if animal_name == name {
                return Some(gender);
            }
        }
        None
    }
}

#[derive(Component)]
pub struct Cat {
    name: String,
}

impl Animal for Cat {
    fn species() -> &'static str {
        "Cat"
    }

    fn name(&self) -> &String {
        &self.name
    }
}

#[derive(Component)]
pub struct Dog {
    name: String,
}

impl Animal for Dog {
    fn species() -> &'static str {
        "Dog"
    }

    fn name(&self) -> &String {
        &self.name
    }
}

pub(crate) fn spawn_cat(
    commands: &mut Commands,
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) {
    let cat_texture = asset_server.load("textures/cat-texture.png");
    let cat_layout = TextureAtlasLayout::from_grid(Vec2::new(26.0, 26.0), 4, 4, None, None);
    let cat_texture_atlas_layout = texture_atlas_layouts.add(cat_layout);
    let cat_animation_indices = AnimationIndices {
        first: 0,
        last: 3,
        current_index: 0,
    }; // idle animation
    let _cat_entity = commands.spawn((
        Cat {
            name: generate_animal_name(AnimalType::Cat),
        },
        Health {
            current: 100,
            max: 100,
            hunger: 100,
        },
        SpriteSheetBundle {
            texture: cat_texture.clone(),
            atlas: TextureAtlas {
                layout: cat_texture_atlas_layout,
                index: cat_animation_indices.first,
            },
            transform: Transform::from_xyz(25.0, 50.0, 0.0),
            ..Default::default()
        },
        AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
        cat_animation_indices.clone(),
        Velocity { x: 15.0, y: 0.0 },
        DeathAnimationPlayed(false),
        GravityScale(1.0),
        Name::new(""),
    ));
}

pub(crate) fn spawn_dog(
    commands: &mut Commands,
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) {
    let dog_texture = asset_server.load("textures/dog-texture.png");
    let dog_layout = TextureAtlasLayout::from_grid(Vec2::new(26.0, 26.0), 4, 4, None, None);
    let dog_texture_atlas_layout = texture_atlas_layouts.add(dog_layout);
    let dog_animation_indices = AnimationIndices {
        first: 0,
        last: 3,
        current_index: 0,
    }; // idle animation
    let _dog_entity = commands.spawn((
        Dog {
            name: generate_animal_name(AnimalType::Dog),
        },
        Health {
            current: 100,
            max: 100,
            hunger: 100,
        },
        SpriteSheetBundle {
            texture: dog_texture.clone(),
            atlas: TextureAtlas {
                layout: dog_texture_atlas_layout,
                index: dog_animation_indices.first,
            },
            transform: Transform::from_xyz(-25.0, 50.0, 0.0),
            ..Default::default()
        },
        AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
        dog_animation_indices.clone(),
        Velocity { x: -2.0, y: 0.0 },
        DeathAnimationPlayed(false),
        GravityScale(1.0),
        Name::new(""),
    ));
}

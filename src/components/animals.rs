use bevy::prelude::Component;
use rand::seq::SliceRandom;
use rand::thread_rng;

use super::names;

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

// Function to generate an animal name based on the given animal type
fn generate_animal_name(animal_type: AnimalType) -> String {
    // Create a random number generator
    let mut rng = thread_rng();

    // Choose a random animal name from the ANIMAL_NAMES array
    // The chosen element is a tuple containing the name, gender, and type of the animal
    match names::ANIMAL_NAMES.choose(&mut rng) {
        Some((name, _gender, name_type)) => {
            // If the type of the chosen animal matches the given animal type
            if *name_type == animal_type {
                // Return the name of the animal as a string
                name.to_string()
            } else {
                // If the types don't match, recursively call the function until a matching animal name is found
                generate_animal_name(animal_type)
            }
        }
        None => {
            // If the ANIMAL_NAMES array is empty, return a default name
            "Default Animal Name".to_string()
        }
    }
}

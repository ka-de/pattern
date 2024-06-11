use super::names;

use rand::seq::SliceRandom;
use rand::thread_rng;

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
    let (name, _gender, name_type) = names::ANIMAL_NAMES.choose(&mut rng).unwrap();

    // If the type of the chosen animal matches the given animal type
    if *name_type == animal_type {
        // Return the name of the animal as a string
        (*name).to_string()
    } else {
        // If the types don't match, recursively call the function until a matching animal name is found
        generate_animal_name(animal_type)
    }
}

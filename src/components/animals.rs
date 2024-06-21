use bevy::prelude::Component;
use rand::seq::SliceRandom;
use rand::thread_rng;

// 🐾

// Animal types
#[derive(Component, PartialEq, Eq)]
pub enum AnimalType {
    Cat,
    Dog,
}

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

#[rustfmt::skip]
pub const ANIMAL_NAMES: &[(&str, &str, AnimalType)] = &[
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

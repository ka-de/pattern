use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Age(u32);

#[derive(Component)]
struct MaxAge(u32);

fn add_people(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    let names = vec!["Elaina Proctor", "Renzo Hume", "Zayna Nieves"];

    for name in names {
        let age: u32 = rng.gen_range(25..=35);
        let max_age: u32 = rng.gen_range((age + 1)..=100);
        commands.spawn((Person, Name(name.to_string()), Age(age), MaxAge(max_age)));
    }
}

fn hello_world() {
    println!("hello world!");
}

fn greet_people(query: Query<(&Name, &Age), With<Person>>) {
    for (name, age) in query.iter() {
        println!("hello {}! You are {} years old.", name.0, age.0);
    }
}

fn update_people(
    mut query: Query<(Entity, &mut Name, &mut Age, &MaxAge), With<Person>>,
    mut commands: Commands,
) {
    for (entity, mut name, mut age, max_age) in query.iter_mut() {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
        }
        age.0 += 1; // Increase age by 1
        if age.0 >= max_age.0 {
            commands.entity(entity).despawn(); // Remove the entity
        }
    }
}

fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, (update_people, greet_people).chain()))
        .run();
}

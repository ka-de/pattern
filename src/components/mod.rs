pub(crate) mod animals;
pub(crate) mod perfui;
pub(crate) mod ui;
pub(crate) mod world;
pub(crate) mod misc;
pub(crate) mod systems;
pub(crate) mod health;
pub(crate) mod gamestate;
pub(crate) mod climber;
pub(crate) mod player;
pub(crate) mod colliderbundle;
pub(crate) mod ground;
pub(crate) mod items;
pub(crate) mod sensorbundle;
pub(crate) mod gamemode;
pub(crate) mod camera;
pub(crate) mod hunger;
pub(crate) mod names;
pub(crate) mod armor;

pub use animals::{ spawn_cat, spawn_dog, Cat, Dog };
pub use perfui::CustomPerfUiAppExt;
pub use world::{ death_zone_bundle, tile_bundle };

use crate::components::health::Health;

use bevy::prelude::*;

// ↗️
#[derive(Component, Debug)]
struct Velocity {
    x: f32, // The x-component of the velocity
    y: f32, // The y-component of the velocity
}

// ⬇️
#[derive(Component, Default)]
struct GravityScale(f32); // The scale of the gravity

// 🎞️
#[derive(Component, Clone)]
struct AnimationIndices {
    first: usize, // The first index of the animation
    last: usize, // The last index of the animation
    current_index: usize, // The current index of the animation
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer); // The timer for the animation

#[derive(Component)]
struct DeathAnimationPlayed(bool); // A boolean to track if the death animation has been played

// Function to update the animation based on the velocity and health of the entities
fn update_animation(mut query: Query<(&mut AnimationIndices, &Velocity, &Health)>) {
    // Iterate over the entities
    for (mut animation_indices, velocity, health) in &mut query {
        // If the health of the entity is greater than 0
        if health.current > 0 {
            // Get the absolute value of the x-component of the velocity
            let abs_velocity = velocity.x.abs();
            // If the absolute velocity is less than 0.01
            if abs_velocity < 0.01 {
                // Set the indices for the idle animation
                if animation_indices.first != 0 {
                    animation_indices.first = 0;
                    animation_indices.last = 3;
                    animation_indices.current_index = 0;
                }
                // If the absolute velocity is less than 2.1
            } else if abs_velocity < 2.1 {
                // Set the indices for the walking animation
                if animation_indices.first != 8 {
                    animation_indices.first = 8;
                    animation_indices.last = 11;
                    animation_indices.current_index = 8;
                }
                // If the absolute velocity is greater than or equal to 2.1
            } else {
                // Set the indices for the running animation
                if animation_indices.first != 12 {
                    animation_indices.first = 12;
                    animation_indices.last = 15;
                    animation_indices.current_index = 12;
                }
            }
            // If the health of the entity is less than or equal to 0
        } else {
            // Set the indices for the death animation
            if animation_indices.first != 4 {
                animation_indices.first = 4;
                animation_indices.last = 4;
                animation_indices.current_index = 4;
            }
        }
    }
}

// Function to update the facing direction of the entities based on their velocity
fn update_facing_direction(mut query: Query<(&mut Sprite, &Velocity)>) {
    // Iterate over the entities
    for (mut sprite, velocity) in &mut query {
        // Flip the sprite if the x-component of the velocity is less than 0
        sprite.flip_x = velocity.x < 0.0;
    }
}

// Function to play the death animation for entities with 0 health
fn play_death_animation(
    mut query: Query<(&mut AnimationIndices, &Health, &mut DeathAnimationPlayed, &mut TextureAtlas)>
) {
    // Iterate over the entities
    for (mut animation_indices, health, mut death_animation_played, mut atlas) in &mut query {
        // If the health of the entity is 0 and the death animation has not been played
        if health.current == 0 && !death_animation_played.0 {
            // Set the indices for the death animation
            animation_indices.first = 4;
            animation_indices.last = 4;
            animation_indices.current_index = 4;
            // Update the TextureAtlas index
            atlas.index = animation_indices.current_index;
            // Mark the death animation as played
            death_animation_played.0 = true;
        }
    }
}

// Function to move entities based on their velocity and health
fn move_entities(time: Res<Time>, mut query: Query<(&mut Transform, &mut Velocity, &Health)>) {
    // Iterate over the entities
    for (mut transform, mut velocity, health) in &mut query {
        // If the health of the entity is greater than 0
        if health.current > 0 {
            // Calculate the change in time
            let delta_seconds = time.delta_seconds();
            // Update the position of the entity based on its velocity and the change in time
            transform.translation.x += velocity.x * delta_seconds;
            transform.translation.y += velocity.y * delta_seconds;
            // If the health of the entity is 0
        } else {
            // Set the velocity of the entity to 0
            velocity.x = 0.0;
            velocity.y = 0.0;
        }
    }
}

// Function to animate the sprite of entities of type T
fn animate_sprite<T: Component>(
    time: Res<Time>, // The current time
    mut query: Query<(&mut AnimationIndices, &mut AnimationTimer, &mut TextureAtlas), With<T>>
) {
    // Iterate over the entities
    for (mut indices, mut timer, mut atlas) in &mut query {
        // Update the animation timer
        timer.tick(time.delta());
        // If the animation timer has just finished
        if timer.just_finished() {
            // Update the current index of the animation
            indices.current_index = if indices.current_index == indices.last {
                if indices.first == 4 {
                    // Loop back to the first frame of the death animation
                    4
                } else {
                    // Loop back to the first frame of the animation
                    indices.first
                }
            } else {
                // Move to the next frame of the animation
                indices.current_index + 1
            };
            // Update the TextureAtlas index
            atlas.index = indices.current_index;
        }
    }
}

// Define a trait named 'CustomSystemsAppExt' for adding custom systems to the app
pub trait CustomSystemsAppExt {
    // Method to add custom systems to the app
    fn add_custom_systems(&mut self) -> &mut Self;
}

// Implement the 'CustomSystemsAppExt' trait for the 'App' struct
impl CustomSystemsAppExt for App {
    // Method to add custom systems to the app
    fn add_custom_systems(&mut self) -> &mut Self {
        // Initialize the hunger timer resource
        self.init_resource::<hunger::HungerTimer>()
            // Add the decrease_hunger system to the update stage
            .add_systems(Update, hunger::decrease_hunger)
            // Add the move_entities system to the update stage
            .add_systems(Update, move_entities)
            // Add the update_facing_direction system to the update stage
            .add_systems(Update, update_facing_direction)
            // Add the animate_sprite system for the Cat component to the update stage
            .add_systems(Update, animate_sprite::<Cat>)
            // Add the animate_sprite system for the Dog component to the update stage
            .add_systems(Update, animate_sprite::<Dog>)
            // Add the update_animation system to the update stage
            .add_systems(Update, update_animation)
            // Add the play_death_animation system to the update stage
            .add_systems(Update, play_death_animation);

        // Setup world systems
        world::setup_world_systems(self);
        // Setup UI
        ui::setup_ui(self);

        systems::setup_ldtk(self);

        self
    }
}

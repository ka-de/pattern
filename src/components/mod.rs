pub(crate) mod animals;
#[cfg(debug_assertions)]
pub(crate) mod perfui;
pub(crate) mod ui;
pub(crate) mod world;
pub(crate) mod misc;
pub(crate) mod systems;
pub(crate) mod health;
pub(crate) mod gamestate;
pub(crate) mod ladders;
pub(crate) mod player;
pub(crate) mod collision;
pub(crate) mod ground;
pub(crate) mod items;
pub(crate) mod sensorbundle;
pub(crate) mod camera;
pub(crate) mod hunger;
pub(crate) mod names;
pub(crate) mod armor;
pub(crate) mod torch;
pub(crate) mod enemy;
pub(crate) mod patrol;
pub(crate) mod input;

use crate::components::health::Health;

use bevy::prelude::*;

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

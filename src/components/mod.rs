mod animals;
mod perfui;
mod ui;
mod world;

pub use animals::{spawn_cat, spawn_dog, Cat, Dog};
pub use perfui::CustomPerfUiAppExt;
pub use world::{death_zone_bundle, tile_bundle};

use bevy::prelude::*;
use bevy::utils::Duration;

/**
 * ↗️
 */
#[derive(Component, Debug)]
struct Velocity {
    x: f32,
    y: f32,
}

/**
 * ⬇️
 */
#[derive(Component, Default)]
struct GravityScale(f32);

#[derive(Component, Clone)]
struct AnimationIndices {
    first: usize,
    last: usize,
    current_index: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn update_animation(mut query: Query<(&mut AnimationIndices, &Velocity, &Health)>) {
    for (mut animation_indices, velocity, health) in query.iter_mut() {
        if health.current > 0 {
            let abs_velocity = velocity.x.abs();
            if abs_velocity < 0.01 {
                // idle animation
                if animation_indices.first != 0 {
                    animation_indices.first = 0;
                    animation_indices.last = 3;
                    animation_indices.current_index = 0;
                }
            } else if abs_velocity < 2.1 {
                // walking animation
                if animation_indices.first != 8 {
                    animation_indices.first = 8;
                    animation_indices.last = 11;
                    animation_indices.current_index = 8;
                }
            } else {
                // running animation
                if animation_indices.first != 12 {
                    animation_indices.first = 12;
                    animation_indices.last = 15;
                    animation_indices.current_index = 12;
                }
            }
        } else {
            // Death animation
            if animation_indices.first != 4 {
                animation_indices.first = 4;
                animation_indices.last = 4;
                animation_indices.current_index = 4;
            }
        }
    }
}

fn update_facing_direction(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        // Flip the sprite based on the direction of movement
        if velocity.x < 0.0 {
            transform.scale.x = transform.scale.x.abs() * -1.0;
        } else {
            transform.scale.x = transform.scale.x.abs();
        }
    }
}

#[derive(Component)]
struct DeathAnimationPlayed(bool);

fn play_death_animation(
    mut query: Query<(
        &mut AnimationIndices,
        &Health,
        &mut DeathAnimationPlayed,
        &mut TextureAtlas,
    )>,
) {
    for (mut animation_indices, health, mut death_animation_played, mut atlas) in query.iter_mut() {
        if health.current == 0 && !death_animation_played.0 {
            animation_indices.first = 4;
            animation_indices.last = 4;
            animation_indices.current_index = 4;
            atlas.index = animation_indices.current_index; // Update the TextureAtlas index
            death_animation_played.0 = true;
        }
    }
}

fn move_entities(time: Res<Time>, mut query: Query<(&mut Transform, &mut Velocity, &Health)>) {
    for (mut transform, mut velocity, health) in query.iter_mut() {
        if health.current > 0 {
            let delta_seconds = time.delta_seconds();
            transform.translation.x += velocity.x * delta_seconds;
            transform.translation.y += velocity.y * delta_seconds;
        } else {
            velocity.x = 0.0;
            velocity.y = 0.0;
        }
    }
}

/**
 * Hunger 🍗
 */
#[derive(Resource, Default)]
struct HungerTimer(Timer);

fn decrease_hunger(
    time: Res<Time>,
    mut hunger_timer: ResMut<HungerTimer>,
    mut health_query: Query<&mut Health>,
) {
    hunger_timer.0.tick(time.delta());
    if hunger_timer.0.just_finished() {
        for mut health in health_query.iter_mut() {
            // Decrease hunger by n every m second.
            health.hunger = health.hunger.saturating_sub(1);

            // If hunger reaches 0, decrease health by n every second.
            if health.hunger == 0 {
                health.current = health.current.saturating_sub(1);
            }
        }
        // Set the timer's duration to n seconds for periodic decrease
        hunger_timer.0.set_duration(Duration::from_secs(20));
        // Reset the timer to count down again.
        hunger_timer.0.reset();
    }
}

/**
 * The Health Component 🩸
 */
#[derive(Component)]
pub struct Health {
    current: u32,
    max: u32,
    hunger: u32,
}

/**
 * Identifies the main camera. 🎥
 */
#[derive(Component)]
pub struct MainCamera;

fn animate_sprite<T: Component>(
    time: Res<Time>,
    mut query: Query<
        (
            &mut AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlas,
        ),
        With<T>,
    >,
) {
    for (mut indices, mut timer, mut atlas) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            indices.current_index = if indices.current_index == indices.last {
                if indices.first == 4 {
                    // Death animation
                    4 // Loop back to the first frame of the death animation
                } else {
                    indices.first
                }
            } else {
                indices.current_index + 1
            };
            atlas.index = indices.current_index;
        }
    }
}


pub trait CustomSystemsAppExt {
    fn add_custom_systems(&mut self) -> &mut Self;
}

impl CustomSystemsAppExt for App {
    fn add_custom_systems(&mut self) -> &mut Self {
        self.init_resource::<HungerTimer>()
        .add_systems(Update, decrease_hunger) // Nyeheh
            .add_systems(Update, move_entities)
            .add_systems(Update, update_facing_direction)
            .add_systems(Update, animate_sprite::<Cat>)
            .add_systems(Update, animate_sprite::<Dog>)
            .add_systems(Update, update_animation)
            .add_systems(Update, play_death_animation);

        world::setup_world_systems(self);
        ui::setup_ui(self)
    }
}

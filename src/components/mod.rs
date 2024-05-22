mod animals;
mod perfui;
mod world;

pub use animals::{spawn_cat, spawn_dog, Cat, Dog};
pub use perfui::CustomPerfUiAppExt;
pub use world::{death_zone_bundle, tile_bundle};

use bevy::input::keyboard::KeyCode;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::utils::Duration;
use bevy::window::PrimaryWindow;

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

#[derive(Resource, Default)]
pub struct SpaceKeyPressCount {
    count: u32,
}

#[derive(Resource, Default)]
pub struct TimeSinceLastClick {
    last_click: Duration,
}

#[derive(Resource, Default)]
pub struct TimeSinceLastKeypress {
    last_keypress: Duration,
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

/**
 * Stores the world position of the mouse cursor.
 */
#[derive(Resource, Default)]
pub struct CursorWorldCoordinates(Vec2);

/**
 * Function to handle the mouse cursor with world coordinates.
 */
fn cursor_system(
    mut coords: ResMut<CursorWorldCoordinates>,
    // Get the window.
    window_query: Query<&Window, With<PrimaryWindow>>,
    // Get the camera transform.
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // Get the camera info and transform.
    let (camera, camera_transform) = camera_query.single();

    // There is only one primary window, so we can get it from the query.
    let window = window_query.single();

    // Check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z.
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        coords.0 = world_position;
    }
}

/**
 * Function to handle mouse clicks.
 */
fn handle_click(
    time: Res<Time>,
    mut lastclick: ResMut<TimeSinceLastClick>,
    mut evr_mouse: EventReader<MouseButtonInput>,
) {
    for ev in evr_mouse.read() {
        if ev.state == ButtonState::Pressed {
            lastclick.last_click = time.elapsed();
        }
    }
}

/**
 * Function to handle key presses.
 */
fn handle_keypress(
    time: Res<Time>,
    mut lastkeypress: ResMut<TimeSinceLastKeypress>,
    mut evr_keyboard: EventReader<KeyboardInput>,
) {
    for ev in evr_keyboard.read() {
        if ev.state == ButtonState::Pressed {
            lastkeypress.last_keypress = time.elapsed();
        }
    }
}

/**
 * Struct for tracking if the Space key is being held.
 */
#[derive(Resource, Default)]
struct SpaceKeyPressState {
    last_pressed: bool,
}

/**
 * Function to handle when the Space key is being pressed.
 */
fn handle_space_keypress(
    mut evr_keyboard: EventReader<KeyboardInput>,
    mut space_key_press_count: ResMut<SpaceKeyPressCount>,
    mut space_key_press_state: ResMut<SpaceKeyPressState>,
) {
    for ev in evr_keyboard.read() {
        if ev.key_code == KeyCode::Space {
            if ev.state == ButtonState::Pressed && !space_key_press_state.last_pressed {
                space_key_press_count.count += 1;
                *space_key_press_state = SpaceKeyPressState { last_pressed: true };
            } else if ev.state == ButtonState::Released {
                *space_key_press_state = SpaceKeyPressState {
                    last_pressed: false,
                };
            }
        }
    }
}

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

pub trait CustomResourcesAppExt {
    fn add_custom_resources(&mut self) -> &mut Self;
}

impl CustomResourcesAppExt for App {
    fn add_custom_resources(&mut self) -> &mut Self {
        self.init_resource::<CursorWorldCoordinates>()
            .init_resource::<TimeSinceLastClick>()
            .init_resource::<TimeSinceLastKeypress>()
            .init_resource::<SpaceKeyPressCount>()
            .init_resource::<SpaceKeyPressState>()
            .init_resource::<HungerTimer>()
    }
}

pub trait CustomSystemsAppExt {
    fn add_custom_systems(&mut self) -> &mut Self;
}

impl CustomSystemsAppExt for App {
    fn add_custom_systems(&mut self) -> &mut Self {
        self.add_systems(Update, decrease_hunger) // Nyeheh
            .add_systems(Update, cursor_system)
            .add_systems(Update, handle_click)
            .add_systems(Update, handle_keypress)
            .add_systems(Update, handle_space_keypress)
            .add_systems(Update, move_entities)
            .add_systems(Update, update_facing_direction)
            .add_systems(Update, animate_sprite::<Cat>)
            .add_systems(Update, animate_sprite::<Dog>)
            .add_systems(Update, update_animation)
            .add_systems(Update, play_death_animation);

        world::setup_world_systems(self)
    }
}

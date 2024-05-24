use bevy::input::keyboard::KeyCode;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::utils::Duration;
use bevy::window::PrimaryWindow;

use crate::components::MainCamera;

#[derive(Resource, Default)]
pub struct SpaceKeyPressCount {
    pub count: u32,
}

#[derive(Resource, Default)]
pub struct TimeSinceLastClick {
    pub last_click: Duration,
}

#[derive(Resource, Default)]
pub struct TimeSinceLastKeypress {
    pub last_keypress: Duration,
}

/**
 * Stores the world position of the mouse cursor.
 */
#[derive(Resource, Default)]
pub struct CursorWorldCoordinates(pub Vec2);

/**
 * Function to handle the mouse cursor with world coordinates.
 */
fn cursor_system(
    mut coords: ResMut<CursorWorldCoordinates>,
    // Get the window.
    window_query: Query<&Window, With<PrimaryWindow>>,
    // Get the camera transform.
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>
) {
    // Get the camera info and transform.
    let (camera, camera_transform) = camera_query.single();

    // There is only one primary window, so we can get it from the query.
    let window = window_query.single();

    // Check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z.
    if
        let Some(world_position) = window
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
    mut evr_mouse: EventReader<MouseButtonInput>
) {
    for ev in evr_mouse.read() {
        if ev.state == ButtonState::Pressed {
            lastclick.last_click = time.elapsed();
        }
    }
}

// Function to handle key presses.
fn handle_keypress(
    time: Res<Time>,
    mut lastkeypress: ResMut<TimeSinceLastKeypress>,
    mut evr_keyboard: EventReader<KeyboardInput>,
    mut space_key_press_count: ResMut<SpaceKeyPressCount>,
    mut space_key_press_state: ResMut<SpaceKeyPressState>
) {
    // Iterate through all keyboard input events.
    for ev in evr_keyboard.read() {
        // Update the time since the last key press to the current elapsed time.
        lastkeypress.last_keypress = time.elapsed();

        // Check if the key pressed is the space bar.
        if ev.key_code == KeyCode::Space {
            // If the space bar is pressed and it was not already pressed,
            // update the last key press time, increment the count, and set the state to pressed.
            if ev.state == ButtonState::Pressed && !space_key_press_state.last_pressed {
                lastkeypress.last_keypress = time.elapsed();
                space_key_press_count.count += 1;
                space_key_press_state.last_pressed = true;
            } else if
                // If the space bar is released, set the state to not pressed.
                ev.state == ButtonState::Released
            {
                space_key_press_state.last_pressed = false;
            }
        }
    }
}

// Struct for tracking if the Space key is being held.
#[derive(Resource, Default)]
struct SpaceKeyPressState {
    last_pressed: bool,
}

pub fn setup_ui(app: &mut App) -> &mut App {
    app.init_resource::<CursorWorldCoordinates>()
        .init_resource::<TimeSinceLastClick>()
        .init_resource::<TimeSinceLastKeypress>()
        .init_resource::<SpaceKeyPressCount>()
        .init_resource::<SpaceKeyPressState>()
        .add_systems(Update, cursor_system)
        .add_systems(Update, handle_click)
        .add_systems(Update, handle_keypress)
}

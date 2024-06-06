use bevy::input::keyboard::KeyCode;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::utils::Duration;

#[derive(Resource, Default)]
struct EKeyPressState {
    last_pressed: bool,
}

#[derive(Resource, Default)]
pub struct EKeyPressCount {
    pub count: u32,
}

#[derive(Resource, Default)]
struct SpaceKeyPressState {
    last_pressed: bool,
}

#[derive(Resource, Default)]
pub struct SpaceKeyPressCount {
    pub count: u32,
}

#[derive(Resource, Default)]
pub struct TimeSinceLastKeypress {
    pub last_keypress: Duration,
}

// Function to handle key presses.
fn handle_keypress(
    time: Res<Time>,
    mut lastkeypress: ResMut<TimeSinceLastKeypress>,
    mut evr_keyboard: EventReader<KeyboardInput>,
    mut space_key_press_count: ResMut<SpaceKeyPressCount>,
    mut space_key_press_state: ResMut<SpaceKeyPressState>,
    mut e_key_press_count: ResMut<EKeyPressCount>,
    mut e_key_press_state: ResMut<EKeyPressState>
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
                space_key_press_count.count += 1;
                space_key_press_state.last_pressed = true;
            } else if ev.state == ButtonState::Released {
                space_key_press_state.last_pressed = false;
            }
        } else if ev.key_code == KeyCode::KeyE {
            if ev.state == ButtonState::Pressed && !space_key_press_state.last_pressed {
                e_key_press_count.count += 1;
                e_key_press_state.last_pressed = true;
            } else if ev.state == ButtonState::Released {
                e_key_press_state.last_pressed = false;
            }
        }
    }
}

pub fn setup_ui(app: &mut App) {
    app.init_resource::<TimeSinceLastKeypress>()
        .init_resource::<SpaceKeyPressCount>()
        .init_resource::<SpaceKeyPressState>()
        .init_resource::<EKeyPressCount>()
        .init_resource::<EKeyPressState>()
        .add_systems(Update, handle_keypress);
}

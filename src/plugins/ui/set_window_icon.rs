// Import necessary Bevy and Winit types.
use bevy::{ prelude::NonSend, winit::WinitWindows };
use winit::window::Icon;

// Define a function to set the window icon for all windows managed by Bevy.
pub fn set_window_icon(windows: NonSend<WinitWindows>) {
    // Attempt to load the image from the specified path and convert it to RGBA format.
    // If the image cannot be opened, log an error message and return early from the function.
    let (icon_rgba, icon_width, icon_height) = match
        image::open("assets/icon.png").map(|image| image.into_rgba8())
    {
        Ok(image) => {
            // If the image is successfully loaded, extract its dimensions and raw pixel data.
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            (rgba, width, height)
        }
        Err(_) => {
            // Log an error message to the standard error output and exit the function.
            eprintln!("Failed to open icon path");
            return;
        }
    };

    // Attempt to create an icon from the raw pixel data.
    // If the icon creation fails, log an error message and return early from the function.
    let icon = match Icon::from_rgba(icon_rgba, icon_width, icon_height) {
        Ok(icon) => icon,
        Err(_) => {
            // Log an error message to the standard error output and exit the function.
            eprintln!("Failed to create icon");
            return;
        }
    };

    // Iterate over all windows managed by Bevy and set the window icon.
    // The icon is cloned for each window to ensure each window gets its own instance.
    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}

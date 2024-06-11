// Provides functions to read and manipulate environment variables.
use std::env;

use wgpu::Backends;

// Allow the user to set the WGPU_BACKEND but have sane defaults for each platform.
pub fn get_backend() -> Option<Backends> {
    // Check if the WGPU_BACKEND environment variable is set
    if let Ok(backend_str) = env::var("WGPU_BACKEND") {
        // Convert the environment variable value to a Backend
        match backend_str.to_lowercase().as_str() {
            "vulkan" => {
                return Some(Backends::VULKAN);
            }
            "dx12" => {
                return Some(Backends::DX12);
            }
            "metal" => {
                return Some(Backends::METAL);
            }
            _ => eprintln!("Unsupported backend: {}", backend_str),
        }
    }

    // If the environment variable is not set, use the default logic
    if cfg!(target_os = "linux") {
        Some(Backends::VULKAN)
    } else if cfg!(target_os = "windows") {
        Some(Backends::DX12)
    } else if cfg!(target_os = "macos") {
        Some(Backends::METAL)
    } else {
        panic!("Unsupported Operating System!");
    }
}

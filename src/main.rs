// Turn clippy into a real bitch
#![warn(clippy::all, clippy::pedantic)]

// This changes the executable to a graphical application instead of a CLI one
// only for Release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Provides functions to read and manipulate environment variables.
use std::env;

// Particle effects
// ⚠️ TODO: Move to plugin or something?
use bevy_hanabi::prelude::*;

use bevy::render::RenderPlugin;
use bevy::render::settings::{ WgpuFeatures, WgpuSettings };
use wgpu::Backends;

mod components;
mod plugins;

use bevy::prelude::*;
use bevy_tweening::*;
// Steamworks
use bevy_steamworks::*;

// Used for setting the Window icon
use bevy::winit::WinitWindows;
use winit::window::Icon;

// ⚠️ TODO: Move this with Game Settings
use components::settings::GameSettings;

// ⚠️ TODO: This will need to get eventually removed from main.
// RANDOM GAMEPLAY COMPONENTS
// use components::player::Player;
use components::torch::Torch;

// ⚠️ TODO: Move to ui!
fn set_window_icon(
    // we have to use `NonSend` here
    windows: NonSend<WinitWindows>
) {
    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/icon.png").expect("Failed to open icon path").into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    // do it for all windows
    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}

// ⚠️ TODO: Move audio stuff to its own thing
use bevy::audio::{ SpatialScale, AudioPlugin };
use bevy::audio::Volume;

const AUDIO_SCALE: f32 = 1.0 / 100.0;

fn change_global_volume(mut volume: ResMut<GlobalVolume>) {
    volume.volume = Volume::new(0.5);
}

// ⚠️ TODO: Currently very dumb, just plays one music on repeat!
fn play_background_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
    // Create an entity dedicated to playing our background music
    commands.spawn(AudioBundle {
        source: asset_server.load("music/garam_masala_wip.ogg"),
        settings: PlaybackSettings::LOOP,
    });
}

// ⚠️ TODO: This is at the moment just testing Spatial Audio
//
//
fn play_2d_spatial_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn our emitter
    commands.spawn((
        Torch,
        AudioBundle {
            source: asset_server.load("vo/dogspeak.ogg"),
            settings: PlaybackSettings::LOOP, // ⚠️ TODO: Change it later to `ONCE` when done testing.
            //settings: PlaybackSettings::ONCE,
        },
    ));

    // Spawn our listener
    commands.spawn((
        SpatialListener::new(100.0), // Gap between the ears
        SpatialBundle::default(),
    ));
}
// End of TODO

// Allow the user to set the WGPU_BACKEND but have sane defaults for each platform.
fn get_backend() -> Option<Backends> {
    // Check if the WGPU_BACKEND environment variable is set
    if let Ok(backend_str) = env::var("WGPU_BACKEND") {
        // Convert the environment variable value to a Backend
        match backend_str.to_lowercase().as_str() {
            "vulkan" => {
                return Some(Backends::VULKAN);
            }
            "dx12" | "direct3d12" => {
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

fn main() {
    #[cfg(not(debug_assertions))] // ⚠️ TODO: At some point we will need to dev with Steam.
    match SteamworksPlugin::init_app(981370) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    }

    let mut wgpu_settings = WgpuSettings::default();
    wgpu_settings.features.set(WgpuFeatures::VERTEX_WRITABLE_STORAGE, true);

    let backend = get_backend();
    wgpu_settings.backends = backend;

    let mut app = App::new();

    //app.add_systems(Startup, play_background_audio);

    app.insert_resource(Msaa::Off) // Disable Multi-Sample Anti-Aliasing
        // DefaultPlugins
        .add_plugins((
            DefaultPlugins.set(RenderPlugin {
                render_creation: wgpu_settings.into(),
                synchronous_pipeline_compilation: false,
                ..default()
            })
                .set(ImagePlugin::default_nearest())
                .set(plugins::debug::make_log_plugin())
                // ⚠️ TODO: Maybe move this to its own thing? I'm not sure!
                .set(AudioPlugin {
                    default_spatial_scale: SpatialScale::new_2d(AUDIO_SCALE),
                    ..default()
                }),
            HanabiPlugin,
            TweeningPlugin,
            plugins::gamestate::game_state_plugin,
            components::systems::setup_ldtk,
            plugins::dialogueview::YarnSpinnerDialogueViewPlugin,
            plugins::debug::plugin,
            plugins::input::InputPlugin,
            plugins::ui::plugin,
        ))
        .add_systems(Startup, set_window_icon) // Set the Window icon.

        // AUDIO TESTING ⚠️
        .insert_resource(GlobalVolume::new(0.2)) // Set the GlobalVolume ⚠️ WIP
        .add_systems(Startup, change_global_volume) // Change the GlobalVolume ⚠️ WIP

        //.add_systems(Startup, play_2d_spatial_audio)

        // GAME SETTINGS ⚠️
        .insert_resource(GameSettings::default());

    app.run();
}

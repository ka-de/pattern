// Turn clippy into a real bitch
#![warn(clippy::all, clippy::pedantic)]

// This changes the executable to a graphical application instead of a CLI one
// only for Release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Provides functions to read and manipulate environment variables.
use std::env;

use bevy::render::settings::WgpuSettings;
use bevy::render::RenderPlugin;
use sickle_ui::{
    dev_panels::{
        hierarchy::{ HierarchyTreeViewPlugin, UiHierarchyExt },
        scene_view::{ SceneView, SceneViewPlugin, SpawnSceneViewPreUpdate, UiSceneViewExt },
    },
    ui_builder::{ UiBuilderExt, UiContextRoot, UiRoot },
    ui_commands::{ SetCursorExt, SetTextExt as _ },
    ui_style::{
        SetBackgroundColorExt,
        SetNodeHeightExt,
        SetNodePositionTypeExt as _,
        SetNodeRightExt as _,
        SetNodeTopExt as _,
        SetNodeWidthExt,
    },
    widgets::{ prelude::*, tab_container::UiTabContainerSubExt, WidgetLibraryUpdate },
    SickleUiPlugin,
};
use wgpu::Backends;

mod components;
mod plugins;

use bevy::prelude::*;
use bevy_tweening::*;
// Steamworks
use bevy_steamworks::*;

/// ⚠️ UI TEST ⚠️
fn spawn_simple_widget(mut commands: Commands) {
    // Let's create a simple column widget on the screen.
    commands.ui_builder(UiRoot).column(|column| {
        // We can style our widget directly in code using the style method.
        column
            .style()
            // The column will be located 100 pixels from the right and 100 pixels from the top of the screen.
            // The absolute position means we are not set relative to any parent.
            .position_type(PositionType::Absolute)
            .right(Val::Px(100.0))
            .top(Val::Px(100.0))
            // We'll bound the height of our column to the total height of our contents.
            // By default, a column will be 100% of the parent's height which would be the entire length of the screen.,
            .height(Val::Auto)
            // Lets give it a visible background color.
            .background_color(Color::rgb(0.1, 0.1, 0.1));
        // Print out "DEVELOPMENT BUILD" when not in release mode.
        #[cfg(debug_assertions)]
        column
            .label(LabelConfig::default())
            .entity_commands()
            // We can use the set_text method to set the text of a label.
            .set_text("DEVELOPMENT BUILD", None);
        // ⚠️ TODO: This will have to go away from the actual release build
        // Print out "ALPHA RELEASE BUILD" when in release mode.
        #[cfg(not(debug_assertions))]
        column
            .label(LabelConfig::default())
            .entity_commands()
            .set_text("ALPHA RELEASE BUILD", None);
    });
}
/// END OF UI TEST ⚠️

// Used for setting the Window icon

use bevy::winit::WinitWindows;
use winit::window::Icon;

// ⚠️ TODO: This will need to get eventually removed from main.
// RANDOM GAMEPLAY COMPONENTS
// use components::player::Player;
use components::torch::Torch;

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

    let backend = get_backend();
    let mut app = App::new();

    //app.add_systems(Startup, play_background_audio);

    #[cfg(target_arch = "wasm32")]
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            canvas: Some("#pattern-canvas".into()),
            ..default()
        }),
        ..default()
    };

    #[cfg(not(target_arch = "wasm32"))]
    let window_plugin = WindowPlugin::default();

    #[cfg(target_arch = "wasm32")]
    app.insert_resource(AssetMetaCheck::Never);

    app.insert_resource(Msaa::Off) // Disable Multi-Sample Anti-Aliasing
        // DefaultPlugins
        .add_plugins((
            DefaultPlugins.set(RenderPlugin {
                render_creation: (WgpuSettings {
                    backends: backend,
                    ..default()
                }).into(),
                ..default()
            })
                .set(window_plugin)
                .set(ImagePlugin::default_nearest())
                .set(plugins::debug::make_log_plugin())
                // ⚠️ TODO: Maybe move this to its own thing? I'm not sure!
                .set(AudioPlugin {
                    default_spatial_scale: SpatialScale::new_2d(AUDIO_SCALE),
                    ..default()
                }),
            // Tweening
            TweeningPlugin,
            components::gamestate::game_state_plugin,
            components::ui::setup_ui,
            components::systems::setup_ldtk,
            plugins::debug::plugin,
        ))
        .add_systems(Startup, set_window_icon) // Set the Window icon.
        // UI TESTING ⚠️
        .add_systems(Startup, spawn_simple_widget)
        .insert_resource(GlobalVolume::new(0.2)) // Set the GlobalVolume ⚠️ WIP
        .add_systems(Startup, change_global_volume); // Change the GlobalVolume ⚠️ WIP
    //.add_systems(Startup, play_2d_spatial_audio);

    app.run();
}

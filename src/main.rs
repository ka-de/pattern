// Turn clippy into a real bitch
#![warn(clippy::all, clippy::pedantic)]

mod components;
mod plugins;

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_tweening::*;
use bevy::asset::AssetMetaCheck;

#[cfg(debug_assertions)]
use bevy::input::common_conditions::input_toggle_active;
#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn play_background_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
    // Create an entity dedicated to playing our background music
    commands.spawn(AudioBundle {
        source: asset_server.load("music/garam_masala_wip.ogg"),
        settings: PlaybackSettings::LOOP,
    });
}

fn main() {
    let mut app = App::new();

    app.add_systems(Startup, play_background_audio);

    // this code is compiled only if debug assertions are enabled (debug mode)
    #[cfg(debug_assertions)]
    let log_plugin = LogPlugin {
        level: bevy::log::Level::DEBUG,
        filter: "info,pattern=debug,wgpu_core=warn,wgpu_hal=warn,pattern=debug".into(),
        update_subscriber: None,
    };

    // this code is compiled only if debug assertions are disabled (release mode)
    #[cfg(not(debug_assertions))]
    let log_plugin = LogPlugin {
        level: bevy::log::Level::INFO,
        filter: "warning,pattern=info".into(),
        update_subscriber: None,
    };

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
            DefaultPlugins.set(window_plugin).set(ImagePlugin::default_nearest()).set(log_plugin),
            // Tweening
            TweeningPlugin,
            components::gamestate::game_state_plugin,
            components::ui::setup_ui,
            components::systems::setup_ldtk,
        ));

    #[cfg(debug_assertions)]
    app.add_plugins((
        // FrameTimeDiagnosticsPlugin
        bevy::diagnostic::FrameTimeDiagnosticsPlugin,
        // WorldInspectorPlugin
        WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F11)),
        // PerformanceUI
        components::perfui::setup_perf_ui,
    ));

    app.run();
}

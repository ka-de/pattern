// Turn clippy into a real bitch
#![warn(clippy::all, clippy::pedantic)]

mod components;
mod plugins;

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_tweening::*;
use bevy::asset::AssetMetaCheck;

// ⚠️ TODO: This will need to get eventually removed from main.
use components::player::Player;
use components::torch::Torch;

#[cfg(debug_assertions)]
use bevy::input::common_conditions::input_toggle_active;
#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

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
            DefaultPlugins.set(window_plugin)
                .set(ImagePlugin::default_nearest())
                .set(log_plugin)
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
        ))
        .insert_resource(GlobalVolume::new(0.2))
        .add_systems(Startup, change_global_volume)
        .add_systems(Startup, play_2d_spatial_audio);

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

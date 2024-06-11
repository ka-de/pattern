// Turn clippy into a real bitch
#![warn(clippy::all, clippy::pedantic)]

// This changes the executable to a graphical application instead of a CLI one
// only for Release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Particle effects
// ⚠️ TODO: Move to plugin or something?
use bevy_hanabi::prelude::*;

use bevy::{
    prelude::PluginGroup,
    utils::default,
    app::{ App, Startup },
    DefaultPlugins,
    asset::AssetServer,
    // ⚠️ TODO: Move audio stuff!
    audio::{
        SpatialScale,
        SpatialListener,
        AudioPlugin,
        Volume,
        GlobalVolume,
        AudioBundle,
        PlaybackSettings,
    },
    ecs::system::{ ResMut, Res, Commands },
    render::{
        view::Msaa,
        // ⚠️ - Audio stuff!
        prelude::SpatialBundle,
        // End of audio stuff!
        RenderPlugin,
        settings::WgpuFeatures,
        settings::WgpuSettings,
        texture::ImagePlugin,
    },
};
use plugins::gamestate::GameState;

mod components;
mod plugins;

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

use crate::plugins::ui::set_window_icon::set_window_icon;
use crate::plugins::get_backend::get_backend;
use crate::plugins::audio::insert_spatial_listener::insert_spatial_listener;

// ⚠️ TODO: Move audio stuff to its own thing
const AUDIO_SCALE: f32 = 1.0 / 100.0;

fn change_global_volume(mut volume: ResMut<GlobalVolume>) {
    volume.volume = Volume::new(0.5);
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
            TweeningPlugin,
            plugins::gamestate::game_state_plugin,
            components::systems::setup_ldtk,
            plugins::dialogueview::YarnSpinnerDialogueViewPlugin {
                loading_state: GameState::SplashScreen,
                playing_state: GameState::Playing,
            },
            plugins::debug::plugin,
            plugins::input::InputPlugin,
            plugins::ui::plugin,
            //HanabiPlugin,
        ))
        .add_systems(Startup, set_window_icon) // Set the Window icon.

        // AUDIO TESTING ⚠️
        .insert_resource(GlobalVolume::new(1.0)) // Set the GlobalVolume ⚠️ WIP
        .add_systems(Startup, change_global_volume) // Change the GlobalVolume ⚠️ WIP

        .add_systems(bevy::app::Update, insert_spatial_listener)

        // GAME SETTINGS ⚠️
        .insert_resource(GameSettings::default());

    app.run();
}

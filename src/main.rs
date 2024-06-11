// Turn clippy into a real bitch
#![warn(clippy::all, clippy::pedantic)]

// This changes the executable to a graphical application instead of a CLI one
// only for Release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Particle effects
// ⚠️ TODO: Move to plugin or something?
use bevy_hanabi::prelude::*;

use bevy::{
    app::{ App, Startup, Update },
    audio::{ AudioPlugin, GlobalVolume, SpatialScale },
    prelude::PluginGroup,
    render::{
        settings::{ WgpuFeatures, WgpuSettings },
        texture::ImagePlugin,
        view::Msaa,
        RenderPlugin,
    },
    utils::default,
    DefaultPlugins,
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

use crate::plugins::ui::set_window_icon::set_window_icon;
use crate::plugins::get_backend::get_backend;
use crate::plugins::audio::insert_audio_components::{
    insert_spatial_listener,
    insert_audio_sources,
};

// ⚠️ TODO: Move audio stuff to its own thing
const AUDIO_SCALE: f32 = 1.0 / 100.0;

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
        .insert_resource(GlobalVolume::new(1.0)) // Set the GlobalVolume
        .add_systems(Update, (insert_spatial_listener, insert_audio_sources))

        // GAME SETTINGS ⚠️
        .insert_resource(GameSettings::default());

    app.run();
}

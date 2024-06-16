// Turn clippy into a real bitch
#![warn(clippy::all, clippy::pedantic)]

// This changes the executable to a graphical application instead of a CLI one
// only for Release builds.
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rand::RngCore;

// bevy_rand
use bevy_prng::*;
use bevy_rand::{ resource::GlobalEntropy, prelude::EntropyPlugin };
use bevy::{
    app::{ App, PreUpdate, Startup, Update },
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{ Commands, Query, Res, ResMut },
    },
    log::{ self, debug, trace },
    prelude::{ IntoSystemConfigs, PluginGroup },
    render::{
        settings::{ WgpuFeatures, WgpuSettings },
        texture::ImagePlugin,
        view::Msaa,
        RenderPlugin,
    },
    time::Time,
    utils::default,
    DefaultPlugins,
};
use plugins::gamestate::GameState;

mod components;
mod systems;
mod entities;
mod plugins;

use bevy_tweening::*;

// 🧠
use big_brain::{
    prelude::{
        ActionBuilder,
        ActionState,
        ActionSpan,
        FirstToScore,
        HasThinker,
        Thinker,
        ScorerBuilder,
        Actor,
        Score,
        ScorerSpan,
    },
    BigBrainSet,
    BigBrainPlugin,
};

// bevy_asepritesheet
use bevy_asepritesheet::prelude::*;

// ✨ - ?
// ✨ - Ray Marching
/*
use bevy_incandescent::{
    ecs::{ PointLight2d, PointLight2dBundle, ShadowCaster2dBundle, SpotLight2d, SpotLight2dBundle },
    math::CircularSector,
    IncandescentPlugin,
};
*/
// ✨ - Particle effects
// ⚠️ TODO: Move to plugin or something?
//use bevy_hanabi::prelude::*;

use crate::components::ai::{
    thirst::thirst_system,
    scorers::thirsty::thirsty_scorer_system,
    actions::drink::drink_action_system,
};

// ⚠️ TODO: Move this with Game Settings
use components::settings::GameSettings;

use crate::plugins::ui::set_window_icon::set_window_icon;
use crate::plugins::get_backend::get_backend;
// ⚠️ TODO: Move audio stuff to its own thing

// 🎲 - Random Number Generation
fn print_random_value(mut rng: ResMut<GlobalEntropy<WyRand>>) {
    println!("Random value: {}", rng.next_u32());
}

fn main() {
    #[cfg(not(debug_assertions))] // ⚠️ TODO: At some point we will need to dev with Steam.
    match bevy_steamworks::SteamworksPlugin::init_app(981370) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    }

    // The Seed 🌱
    let seed: u64 = 1990;

    let mut wgpu_settings = WgpuSettings::default();
    wgpu_settings.features.set(WgpuFeatures::VERTEX_WRITABLE_STORAGE, true);

    let backend = get_backend();
    wgpu_settings.backends = backend;

    let mut app = App::new();

    //app.add_systems(Startup, play_background_audio);

    app.insert_resource(Msaa::Off) // Disable Multi-Sample Anti-Aliasing
        // 🎲
        .add_plugins(EntropyPlugin::<WyRand>::with_seed(seed.to_ne_bytes()))
        // 🧠
        .add_plugins(BigBrainPlugin::new(PreUpdate))
        .add_systems(Startup, components::ai::setup)
        //.add_systems(Update, search_for_weapon)
        //.add_systems(Update, drop_weapon)
        .add_systems(PreUpdate, (
            drink_action_system.in_set(BigBrainSet::Actions),
            thirsty_scorer_system.in_set(BigBrainSet::Scorers),
        ))
        .add_systems(Update, thirst_system)
        // ~~~
        .add_plugins((
            DefaultPlugins.build()
                .set(RenderPlugin {
                    render_creation: wgpu_settings.into(),
                    synchronous_pipeline_compilation: false,
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(plugins::debug::make_log_plugin()),
            TweeningPlugin,
            plugins::gamestate::game_state_plugin,
            systems::setup_world_systems,
            plugins::dialogueview::YarnSpinnerDialogueViewPlugin {
                loading_state: GameState::SplashScreen,
                playing_state: GameState::Playing,
            },
            plugins::input::InputPlugin,
            plugins::ui::plugin,
            plugins::audio::plugin,
            plugins::pathfinding::plugin,
            AsepritesheetPlugin::new(&["sprite.json"]),
            //IncandescentPlugin,
            //HanabiPlugin,
        ))
        .add_systems(Startup, set_window_icon)
        .add_systems(Startup, print_random_value) // Set the Window icon.
        // GAME SETTINGS ⚠️
        .insert_resource(GameSettings::default())
        // Debug plugin comes last, allowing to inspect the final app state.
        .add_plugins(plugins::debug::plugin);

    // Actually start the game now!
    app.run();
}

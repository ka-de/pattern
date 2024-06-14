// Turn clippy into a real bitch
#![warn(clippy::all, clippy::pedantic)]

// This changes the executable to a graphical application instead of a CLI one
// only for Release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rand::RngCore;

// Particle effects
// ⚠️ TODO: Move to plugin or something?
use bevy_hanabi::prelude::*;

// bevy_rand
use bevy_prng::*;
use bevy_rand::{ resource::GlobalEntropy, prelude::EntropyPlugin };
use bevy::{
    ecs::{
        system::{ Commands, Query, ResMut, Res },
        entity::Entity,
        query::With,
        component::Component,
    },
    app::{
        App,
        StateTransition,
        RunFixedMainLoop,
        FixedMain,
        First,
        PreStartup,
        Startup,
        PostStartup,
        PreUpdate,
        Update,
        PostUpdate,
        Last,
    },
    prelude::PluginGroup,
    render::{
        settings::{ WgpuFeatures, WgpuSettings },
        texture::ImagePlugin,
        view::Msaa,
        RenderPlugin,
    },
    utils::default,
    DefaultPlugins,
    log::{ debug, trace, LogPlugin },
    time::Time,
};
use plugins::gamestate::GameState;

mod components;
mod systems;
mod entities;
mod plugins;

use bevy_tweening::*;
// Steamworks
use bevy_steamworks::*;

// 🧠
use big_brain::{
    prelude::{ ActionBuilder, ActionState, ActionSpan, FirstToScore, HasThinker, Thinker },
    BigBrainPlugin,
};

// ⚠️ TODO: Move this with Game Settings
use components::settings::GameSettings;

use crate::plugins::ui::set_window_icon::set_window_icon;
use crate::plugins::get_backend::get_backend;
// ⚠️ TODO: Move audio stuff to its own thing

fn print_random_value(mut rng: ResMut<GlobalEntropy<WyRand>>) {
    println!("Random value: {}", rng.next_u32());
}

#[derive(Clone, Component, Debug, ActionBuilder)]
struct OneOff;

fn one_off_action_system(mut query: Query<(&mut ActionState, &ActionSpan), With<OneOff>>) {
    for (mut state, span) in &mut query {
        let _guard = span.span().enter();
        match *state {
            ActionState::Init => {
                debug!("One-Off ActionState init.");
            }
            ActionState::Requested => {
                debug!("One-Off ActionState requested.");
                *state = ActionState::Success;
            }
            ActionState::Cancelled => {
                debug!("One-off ActionState was cancelled, resulting in failure.");
                *state = ActionState::Failure;
            }
            _ => {}
        }
    }
}

pub fn init_entities(mut cmd: Commands) {
    // You at least need to have a Thinker in order to schedule one-off
    // actions. It's not a general-purpose task scheduler.
    cmd.spawn((
        Thirst::new(75.0, 2.0),
        Thinker::build().label("AIBrain").picker(FirstToScore { threshold: 0.8 }),
    ));
}

#[derive(Component, Debug)]
pub struct Thirst {
    pub per_second: f32,
    pub thirst: f32,
}

impl Thirst {
    pub fn new(thirst: f32, per_second: f32) -> Self {
        Self { thirst, per_second }
    }
}

pub fn thirst_system(
    time: Res<Time>,
    mut thirsts: Query<(Entity, &mut Thirst)>,
    // We need to get to the Thinker. That takes a couple of steps.
    has_thinkers: Query<&HasThinker>,
    mut thinkers: Query<(&mut Thinker, &ActionSpan)>
) {
    for (actor, mut thirst) in &mut thirsts {
        thirst.thirst += thirst.per_second * ((time.delta().as_micros() as f32) / 1_000_000.0);
        if thirst.thirst >= 100.0 {
            let thinker_ent = has_thinkers.get(actor).unwrap().entity();
            let (mut thinker, span) = thinkers.get_mut(thinker_ent).unwrap();
            let _guard = span.span().enter();
            debug!("Scheduling one-off action");
            thinker.schedule_action(OneOff);
            thirst.thirst = 0.0;
        }
        trace!("Thirst: {}", thirst.thirst);
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

    // The Seed 🌱
    let seed: u64 = 1990;

    let mut wgpu_settings = WgpuSettings::default();
    wgpu_settings.features.set(WgpuFeatures::VERTEX_WRITABLE_STORAGE, true);

    let backend = get_backend();
    wgpu_settings.backends = backend;

    let mut app = App::new();

    //app.add_systems(Startup, play_background_audio);

    app.insert_resource(Msaa::Off) // Disable Multi-Sample Anti-Aliasing
        .add_plugins(EntropyPlugin::<WyRand>::with_seed(seed.to_ne_bytes()))
        .add_plugins(BigBrainPlugin::new(PreUpdate))
        .add_systems(Startup, init_entities)
        .add_systems(Update, thirst_system)
        .add_plugins((
            DefaultPlugins.build()
                // ⚠️ TODO: Make this a feature flag or something!
                //.disable::<LogPlugin>()
                .set(plugins::debug::make_log_plugin())
                .set(RenderPlugin {
                    render_creation: wgpu_settings.into(),
                    synchronous_pipeline_compilation: false,
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            TweeningPlugin,
            plugins::gamestate::game_state_plugin,
            systems::setup_world_systems,
            plugins::dialogueview::YarnSpinnerDialogueViewPlugin {
                loading_state: GameState::SplashScreen,
                playing_state: GameState::Playing,
            },
            plugins::debug::plugin,
            plugins::input::InputPlugin,
            plugins::ui::plugin,
            plugins::audio::plugin,
            //HanabiPlugin,
        ))
        .add_systems(Startup, set_window_icon)
        .add_systems(Startup, print_random_value) // Set the Window icon.
        // GAME SETTINGS ⚠️
        .insert_resource(GameSettings::default());

    // Actually start the game now!
    app.run();
}

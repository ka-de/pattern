// Turn clippy into a real bitch
#![warn(clippy::all, clippy::pedantic)]

// This changes the executable to a graphical application instead of a CLI one
// only for Release builds.
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rand::RngCore;

use bevy::{ prelude::*, render::RenderPlugin, render::settings::{ WgpuFeatures, WgpuSettings } };
// seldom_state
use seldom_state::prelude::*;
// bevy_rand
use bevy_rand::prelude::{ WyRand, EntropyPlugin, GlobalEntropy };

use plugins::gamestate::GameState;

mod components;
mod systems;
mod entities;
mod plugins;

//use bevy_tweening::*;

/*
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
*/

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

// ⚠️ TODO: Move this with Game Settings
use components::settings::GameSettings;

use crate::plugins::ui::set_window_icon::set_window_icon;
use crate::plugins::get_backend::get_backend;
// ⚠️ TODO: Move audio stuff to its own thing

// 🎲 - Random Number Generation
fn print_random_value(mut rng: ResMut<GlobalEntropy<WyRand>>) {
    println!("Random value: {}", rng.next_u32());
}

//////////////////////////////////////////////////////////////////////////////////////////
/// STATE
use crate::entities::Player;

// Entities in the `Idle` state do nothing
#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
struct Idle;

// Entities in the `Follow` state move toward the given entity at the given speed
#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
struct Follow {
    target: Entity,
    speed: f32,
}

// Let's define some behavior for entities in the follow state
fn follow(
    mut transforms: Query<&mut Transform>,
    follows: Query<(Entity, &Follow)>,
    time: Res<Time>
) {
    for (entity, follow) in &follows {
        // Get the positions of the follower and target
        let target_translation = transforms.get(follow.target).unwrap().translation;
        let follow_transform = &mut transforms.get_mut(entity).unwrap();
        let follow_translation = follow_transform.translation;

        // Find the direction from the follower to the target and go that way
        follow_transform.translation +=
            (target_translation - follow_translation).normalize_or_zero() *
            follow.speed *
            time.delta_seconds();
    }
}

// For the sake of example, this is a function that returns the `near_player` trigger from before.
// This may be useful so that triggers that accept case-by-case values may be used across the
// codebase. Triggers that don't need to accept any values from local code may be defined as normal
// Bevy systems (see the `done` example). Also consider implementing the `Trigger` trait directly.
#[allow(dead_code)]
fn near(target: Entity) -> impl Trigger<Out = Result<f32, f32>> {
    (move |In(entity): In<Entity>, transforms: Query<&Transform>| {
        let distance = transforms
            .get(target)
            .unwrap()
            .translation.truncate()
            .distance(transforms.get(entity).unwrap().translation.truncate());

        // Check whether the target is within range. If it is, return `Ok` to trigger!
        match distance <= 300.0 {
            true => Ok(distance),
            false => Err(distance),
        }
    }).into_trigger()
}

fn state(player_query: Query<&Transform, With<Player>>) {
    // This is our trigger, which is a Bevy system that returns a `bool`, `Option`, or `Result`. We
    // define the trigger as a closure within this function so it can use variables in the scope
    // (namely, `player`). For the sake of example, we also define this trigger as an external
    // function later.
    //
    // Triggers are reinitialized after each transition, so they won't read events that occurred in a
    // previous state, `Local`s are reset between transitions, etc.
    let near_player = move |In(entity): In<Entity>, transforms: Query<&Transform>| {
        let distance = transforms
            .get(player)
            .unwrap()
            .translation.truncate()
            .distance(transforms.get(entity).unwrap().translation.truncate());

        // Check whether the target is within range. If it is, return `Ok` to trigger!
        match distance <= 300.0 {
            true => Ok(distance),
            false => Err(distance),
        }
    };
}

///////////////////////////////////////////////////////////////////////////////////////

fn main() {
    #[cfg(not(feature = "dev_features"))] // ⚠️ TODO: At some point we will need to dev with Steam.
    if std::env::var_os("NO_STEAM") == None {
        match bevy_steamworks::SteamworksPlugin::init_app(981370) {
            Ok(_) => (),
            Err(err) => {
                eprintln!("{}", err);
                return;
            }
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
    // Insert Resource
    app.insert_resource(Msaa::Off) // Disable Multi-Sample Anti-Aliasing
        // GAME SETTINGS ⚠️
        .insert_resource(GameSettings::default())
        // Plugins
        // 🎲
        .add_plugins(EntropyPlugin::<WyRand>::with_seed(seed.to_ne_bytes()))
        // 🧠
        /*
        .add_plugins(BigBrainPlugin::new(PreUpdate))
        .add_systems(Startup, components::ai::setup)
        //.add_systems(Update, search_for_weapon)
        //.add_systems(Update, drop_weapon)
        .add_systems(PreUpdate, (
            drink_action_system.in_set(BigBrainSet::Actions),
            thirsty_scorer_system.in_set(BigBrainSet::Scorers),
        ))
        .add_systems(Update, thirst_system)
		*/
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
            //TweeningPlugin,
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
            bevy_mod_aseprite::AsepritePlugin,
            StateMachinePlugin,
            //IncandescentPlugin,
            //HanabiPlugin,
        ))

        // Startup
        .add_systems(Startup, set_window_icon) // Set the Window icon.

        //.add_systems(Startup, print_random_value)

        // Update
        .add_systems(Update, follow)
        // Debug plugin comes last, allowing to inspect the final app state.
        .add_plugins(plugins::debug::plugin);

    // Actually start the game now!
    app.run();
}

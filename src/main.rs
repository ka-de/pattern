// Turn clippy into a real bitch
#![warn(clippy::all, clippy::pedantic)]

// This changes the executable to a graphical application instead of a CLI one
// only for Release builds.
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rand::RngCore;

// Particle effects
// ⚠️ TODO: Move to plugin or something?
use bevy_hanabi::prelude::*;

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

// ⚠️ TODO: Move this with Game Settings
use components::settings::GameSettings;

use crate::plugins::ui::set_window_icon::set_window_icon;
use crate::plugins::get_backend::get_backend;
// ⚠️ TODO: Move audio stuff to its own thing

// 🎲 - Random Number Generation
fn print_random_value(mut rng: ResMut<GlobalEntropy<WyRand>>) {
    println!("Random value: {}", rng.next_u32());
}

// 🧠 - AI
//
// First, we define a "Thirst" component and associated system. This is NOT
// THE AI. It's a plain old system that just makes an entity "thirstier" over
// time. This is what the AI will later interact with.
//
// There's nothing special here. It's a plain old Bevy component.
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

pub fn thirst_system(time: Res<Time>, mut thirsts: Query<&mut Thirst>) {
    for mut thirst in &mut thirsts {
        thirst.thirst += thirst.per_second * ((time.delta().as_micros() as f32) / 1_000_000.0);
        if thirst.thirst >= 100.0 {
            debug!("Thirst >= {}", thirst.thirst);
            thirst.thirst = 100.0;
        }

        trace!("Thirst: {}", thirst.thirst);
    }
}

// The second step is to define an action. What can the AI do, and how does it
// do it? This is the first bit involving Big Brain itself, and there's a few
// pieces you need:
//
// 1. An Action Component. This is just a plain Component we will query
//    against later.
// 2. An ActionBuilder. This is anything that implements the ActionBuilder
//    trait.
// 3. A System that will run Action code.
//
// These actions will be spawned and queued by the game engine when their
// conditions trigger (we'll configure what these are later).
//
// In most cases, the ActionBuilder just attaches the Action component to the
// actor entity. In this case, you can use the derive macro `ActionBuilder`
// to make your Action Component implement the ActionBuilder trait.
// You need your type to implement Clone and Debug (necessary for ActionBuilder)
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct Drink {
    until: f32,
    per_second: f32,
}

// Action systems execute according to a state machine, where the states are
// labeled by ActionState.
fn drink_action_system(
    time: Res<Time>,
    mut thirsts: Query<&mut Thirst>,
    // We execute actions by querying for their associated Action Component
    // (Drink in this case). You'll always need both Actor and ActionState.
    mut query: Query<(&Actor, &mut ActionState, &Drink, &ActionSpan)>
) {
    for (Actor(actor), mut state, drink, span) in &mut query {
        // This sets up the tracing scope. Any `debug` calls here will be
        // spanned together in the output.
        let _guard = span.span().enter();

        // Use the drink_action's actor to look up the corresponding Thirst Component.
        if let Ok(mut thirst) = thirsts.get_mut(*actor) {
            match *state {
                ActionState::Requested => {
                    debug!("Time to drink some water!");
                    *state = ActionState::Executing;
                }
                ActionState::Executing => {
                    trace!("Drinking...");
                    thirst.thirst -=
                        drink.per_second * ((time.delta().as_micros() as f32) / 1_000_000.0);
                    if thirst.thirst <= drink.until {
                        // To "finish" an action, we set its state to Success or
                        // Failure.
                        debug!("Done drinking water");
                        *state = ActionState::Success;
                    }
                }
                // All Actions should make sure to handle cancellations!
                ActionState::Cancelled => {
                    debug!("Action was cancelled. Considering this a failure.");
                    *state = ActionState::Failure;
                }
                _ => {}
            }
        }
    }
}

// Then, we have something called "Scorers". These are special components that
// run in the background, calculating a "Score" value, which is what Big Brain
// will use to pick which Actions to execute.
//
// Just like with Actions, there is a distinction between Scorer components
// and the ScorerBuilder which will attach those components to the Actor entity.
//
// Again, in most cases, you can use the `ScorerBuilder` derive macro to make your
// Scorer Component act as a ScorerBuilder. You need it to implement Clone and Debug.
#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct Thirsty;

// Looks familiar? It's a lot like Actions!
pub fn thirsty_scorer_system(
    thirsts: Query<&Thirst>,
    // Same dance with the Actor here, but now we use look up Score instead of ActionState.
    mut query: Query<(&Actor, &mut Score, &ScorerSpan), With<Thirsty>>
) {
    for (Actor(actor), mut score, span) in &mut query {
        if let Ok(thirst) = thirsts.get(*actor) {
            // This is really what the job of a Scorer is. To calculate a
            // generic "Utility" score that the Big Brain engine will compare
            // against others, over time, and use to make decisions. This is
            // generally "the higher the better", and "first across the finish
            // line", but that's all configurable using Pickers!
            //
            // The score here must be between 0.0 and 1.0.
            score.set(thirst.thirst / 100.0);
            if thirst.thirst >= 80.0 {
                span.span().in_scope(|| {
                    debug!("Thirst above threshold! Score: {}", thirst.thirst / 100.0)
                });
            }
        }
    }
}

// Now that we have all that defined, it's time to add a Thinker to an entity!
// The Thinker is the actual "brain" behind all the AI. Every entity you want
// to have AI behavior should have one *or more* Thinkers attached to it.
pub fn init_entities(mut cmd: Commands) {
    // Create the entity and throw the Thirst component in there. Nothing special here.
    cmd.spawn((
        Thirst::new(75.0, 1.6),
        Thinker::build()
            .label("AIBrain")
            .picker(FirstToScore { threshold: 0.8 })
            // Technically these are supposed to be ActionBuilders and
            // ScorerBuilders, but our Clone impls simplify our code here.
            .when(Thirsty, Drink {
                until: 1.0,
                per_second: 5.0,
            }),
    ));
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
        .add_systems(Startup, init_entities)
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

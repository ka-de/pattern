// Turn clippy into a real bitch
#![warn(clippy::all, clippy::pedantic)]

// This changes the executable to a graphical application instead of a CLI one
// only for Release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rand::RngCore;

/*
// ⚠️ GraphViz
use graphviz_rust::{
    dot_generator::*,
    dot_structures::*,
    attributes::*,
    cmd::{ CommandArg, Format },
    exec,
    parse,
    printer::{ DotPrinter, PrinterContext },
};
*/

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

// Used for setting the Window icon
use bevy::winit::WinitWindows;
use winit::window::Icon;

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
            ActionState::Requested => {
                debug!("One-off action!");
                *state = ActionState::Success;
            }
            ActionState::Cancelled => {
                debug!("One-off action was cancelled. Considering this a failure.");
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
        Thinker::build().label("My Thinker").picker(FirstToScore { threshold: 0.8 }),
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

    /*

    // ⚠️ debugdump 🐛
    // ⚠️ TODO: OnEnter(…), OnExit(…), OnTransition(…)
	// ⚠️ TODO: FixedFirst etc?

    let mut _render_graph_settings = bevy_mod_debugdump::render_graph::Settings::default();
    let mut _schedule_graph_settings = bevy_mod_debugdump::schedule_graph::Settings::default();

    // Render Graph
    let _render_graph = bevy_mod_debugdump::render_graph_dot(&mut app, &_render_graph_settings);
    let _render_graph_g: graphviz_rust::dot_structures::Graph = parse(&_render_graph).unwrap();
    let _render_graph_dot = exec(
        _render_graph_g.clone(),
        &mut PrinterContext::default(),
        vec![Format::Dot.into(), CommandArg::Output("docs/render-graph.dot".to_string())]
    ).unwrap();
    let _render_graph_svg = exec(
        _render_graph_g.clone(),
        &mut PrinterContext::default(),
        vec![Format::Svg.into(), CommandArg::Output("docs/render-graph.svg".to_string())]
    ).unwrap();
    let _render_graph_png = exec(
        _render_graph_g.clone(),
        &mut PrinterContext::default(),
        vec![Format::Png.into(), CommandArg::Output("docs/render-graph.png".to_string())]
    ).unwrap();

    // First Schedule Graph
    let _first_schedule_graph = bevy_mod_debugdump::schedule_graph_dot(
        &mut app,
        First,
        &_schedule_graph_settings
    );
    let _first_schedule_g: graphviz_rust::dot_structures::Graph = parse(
        &_first_schedule_graph
    ).unwrap();
    let _first_schedule_graph_dot = exec(
        _first_schedule_g.clone(),
        &mut PrinterContext::default(),
        vec![Format::Dot.into(), CommandArg::Output("docs/first-schedule-graph.dot".to_string())]
    ).unwrap();
    let _first_schedule_graph_svg = exec(
        _first_schedule_g.clone(),
        &mut PrinterContext::default(),
        vec![Format::Svg.into(), CommandArg::Output("docs/first-schedule-graph.svg".to_string())]
    ).unwrap();
    let _first_schedule_graph_png = exec(
        _first_schedule_g.clone(),
        &mut PrinterContext::default(),
        vec![Format::Png.into(), CommandArg::Output("docs/first-schedule-graph.png".to_string())]
    ).unwrap();

    // Last Schedule Graph
    let _last_schedule_graph = bevy_mod_debugdump::schedule_graph_dot(
        &mut app,
        Last,
        &_schedule_graph_settings
    );
    let _last_schedule_g: graphviz_rust::dot_structures::Graph = parse(
        &_last_schedule_graph
    ).unwrap();
    let _last_schedule_graph_dot = exec(
        _last_schedule_g.clone(),
        &mut PrinterContext::default(),
        vec![Format::Dot.into(), CommandArg::Output("docs/last-schedule-graph.dot".to_string())]
    ).unwrap();
    let _last_schedule_graph_svg = exec(
        _last_schedule_g.clone(),
        &mut PrinterContext::default(),
        vec![Format::Svg.into(), CommandArg::Output("docs/last-schedule-graph.svg".to_string())]
    ).unwrap();
    let _last_schedule_graph_png = exec(
        _last_schedule_g.clone(),
        &mut PrinterContext::default(),
        vec![Format::Png.into(), CommandArg::Output("docs/last-schedule-graph.png".to_string())]
    ).unwrap();

    // StateTransition Schedule Graph
    let _statetransition_schedule_graph = bevy_mod_debugdump::schedule_graph_dot(
        &mut app,
        StateTransition,
        &_schedule_graph_settings
    );
    let _statetransition_schedule_g: graphviz_rust::dot_structures::Graph = parse(
        &_statetransition_schedule_graph
    ).unwrap();
    let _statetransition_schedule_graph_dot = exec(
        _statetransition_schedule_g.clone(),
        &mut PrinterContext::default(),
        vec![
            Format::Dot.into(),
            CommandArg::Output("docs/statetransition-schedule-graph.dot".to_string())
        ]
    ).unwrap();
    let _statetransition_schedule_graph_svg = exec(
        _statetransition_schedule_g.clone(),
        &mut PrinterContext::default(),
        vec![
            Format::Svg.into(),
            CommandArg::Output("docs/statetransition-schedule-graph.svg".to_string())
        ]
    ).unwrap();
    let _statetransition_schedule_graph_png = exec(
        _statetransition_schedule_g.clone(),
        &mut PrinterContext::default(),
        vec![
            Format::Png.into(),
            CommandArg::Output("docs/statetransition-schedule-graph.png".to_string())
        ]
    ).unwrap();

    // FixedMain
    let _fixedmain_schedule_graph = bevy_mod_debugdump::schedule_graph_dot(
        &mut app,
        FixedMain,
        &_schedule_graph_settings
    );
    let _fixedmain_schedule_g: graphviz_rust::dot_structures::Graph = parse(
        &_fixedmain_schedule_graph
    ).unwrap();
    let _fixedmain_schedule_graph_dot = exec(
        _fixedmain_schedule_g.clone(),
        &mut PrinterContext::default(),
        vec![
            Format::Dot.into(),
            CommandArg::Output("docs/fixedmain-schedule-graph.dot".to_string())
        ]
    ).unwrap();
    let _fixedmain_schedule_graph_svg = exec(
        _fixedmain_schedule_g.clone(),
        &mut PrinterContext::default(),
        vec![
            Format::Svg.into(),
            CommandArg::Output("docs/fixedmain-schedule-graph.svg".to_string())
        ]
    ).unwrap();
    let _fixedmain_schedule_graph_png = exec(
        _fixedmain_schedule_g.clone(),
        &mut PrinterContext::default(),
        vec![
            Format::Png.into(),
            CommandArg::Output("docs/fixedmain-schedule-graph.png".to_string())
        ]
    ).unwrap();

    // RunFixedMainLoop Schedule Graph
    let _runfixedmainloop_schedule_graph = bevy_mod_debugdump::schedule_graph_dot(
        &mut app,
        RunFixedMainLoop,
        &_schedule_graph_settings
    );
    let _runfixedmainloop_schedule_g: graphviz_rust::dot_structures::Graph = parse(
        &_runfixedmainloop_schedule_graph
    ).unwrap();
    let _runfixedmainloop_schedule_graph_dot = exec(
        _runfixedmainloop_schedule_g.clone(),
        &mut PrinterContext::default(),
        vec![
            Format::Dot.into(),
            CommandArg::Output("docs/runfixedmainloop-schedule-graph.dot".to_string())
        ]
    ).unwrap();
    let _runfixedmainloop_schedule_graph_svg = exec(
        _runfixedmainloop_schedule_g.clone(),
        &mut PrinterContext::default(),
        vec![
            Format::Svg.into(),
            CommandArg::Output("docs/runfixedmainloop-schedule-graph.svg".to_string())
        ]
    ).unwrap();
    let _runfixedmainloop_schedule_graph_png = exec(
        _runfixedmainloop_schedule_g.clone(),
        &mut PrinterContext::default(),
        vec![
            Format::Png.into(),
            CommandArg::Output("docs/runfixedmainloop-schedule-graph.png".to_string())
        ]
    ).unwrap();

    // Startup Schedule Graph
    let _startup_schedule_graph = bevy_mod_debugdump::schedule_graph_dot(
        &mut app,
        Startup,
        &_schedule_graph_settings
    );
    let _startup_schedule_g: graphviz_rust::dot_structures::Graph = parse(
        &_startup_schedule_graph
    ).unwrap();
    let _startup_schedule_graph_dot = exec(
        _startup_schedule_g.clone(),
        &mut PrinterContext::default(),
        vec![Format::Dot.into(), CommandArg::Output("docs/startup-schedule-graph.dot".to_string())]
    ).unwrap();
    let _startup_schedule_graph_svg = exec(
        _startup_schedule_g.clone(),
        &mut PrinterContext::default(),
        vec![Format::Svg.into(), CommandArg::Output("docs/startup-schedule-graph.svg".to_string())]
    ).unwrap();
    let _startup_schedule_graph_png = exec(
        _startup_schedule_g.clone(),
        &mut PrinterContext::default(),
        vec![Format::Png.into(), CommandArg::Output("docs/startup-schedule-graph.png".to_string())]
    ).unwrap();

    // PreStartup Schedule Graph
    let _prestartup_schedule_graph = bevy_mod_debugdump::schedule_graph_dot(
        &mut app,
        PreStartup,
        &_schedule_graph_settings
    );
    let _prestartup_schedule_g: graphviz_rust::dot_structures::Graph = parse(
        &_prestartup_schedule_graph
    ).unwrap();
    let _prestartup_schedule_graph_dot = exec(
        _prestartup_schedule_g.clone(),
        &mut PrinterContext::default(),
        vec![
            Format::Dot.into(),
            CommandArg::Output("docs/prestartup-schedule-graph.dot".to_string())
        ]
    ).unwrap();
    let _prestartup_schedule_graph_svg = exec(
        _prestartup_schedule_g.clone(),
        &mut PrinterContext::default(),
        vec![
            Format::Svg.into(),
            CommandArg::Output("docs/prestartup-schedule-graph.svg".to_string())
        ]
    ).unwrap();
    let _prestartup_schedule_graph_png = exec(
        _prestartup_schedule_g.clone(),
        &mut PrinterContext::default(),
        vec![
            Format::Png.into(),
            CommandArg::Output("docs/prestartup-schedule-graph.png".to_string())
        ]
    ).unwrap();

    // PostStartup Schedule Graph
    let _poststartup_schedule_graph = bevy_mod_debugdump::schedule_graph_dot(
        &mut app,
        PostStartup,
        &_schedule_graph_settings
    );
    let _poststartup_schedule_g: graphviz_rust::dot_structures::Graph = parse(
        &_poststartup_schedule_graph
    ).unwrap();
    let _poststartup_schedule_graph_dot = exec(
        _poststartup_schedule_g.clone(),
        &mut PrinterContext::default(),
        vec![
            Format::Dot.into(),
            CommandArg::Output("docs/poststartup-schedule-graph.dot".to_string())
        ]
    ).unwrap();
    let _poststartup_schedule_graph_svg = exec(
        _poststartup_schedule_g.clone(),
        &mut PrinterContext::default(),
        vec![
            Format::Svg.into(),
            CommandArg::Output("docs/poststartup-schedule-graph.svg".to_string())
        ]
    ).unwrap();
    let _poststartup_schedule_graph_png = exec(
        _poststartup_schedule_g.clone(),
        &mut PrinterContext::default(),
        vec![
            Format::Png.into(),
            CommandArg::Output("docs/poststartup-schedule-graph.png".to_string())
        ]
    ).unwrap();

    // PreUpdate Schedule Graph
    let _preupdate_schedule_graph = bevy_mod_debugdump::schedule_graph_dot(
        &mut app,
        PreUpdate,
        &_schedule_graph_settings
    );
    let _preupdate_schedule_g: graphviz_rust::dot_structures::Graph = parse(
        &_preupdate_schedule_graph
    ).unwrap();
    let _preupdate_schedule_graph_dot = exec(
        _preupdate_schedule_g.clone(),
        &mut PrinterContext::default(),
        vec![
            Format::Dot.into(),
            CommandArg::Output("docs/preupdate-schedule-graph.dot".to_string())
        ]
    ).unwrap();
    let _preupdate_schedule_graph_svg = exec(
        _preupdate_schedule_g.clone(),
        &mut PrinterContext::default(),
        vec![
            Format::Svg.into(),
            CommandArg::Output("docs/preupdate-schedule-graph.svg".to_string())
        ]
    ).unwrap();
    let _preupdate_schedule_graph_png = exec(
        _preupdate_schedule_g.clone(),
        &mut PrinterContext::default(),
        vec![
            Format::Png.into(),
            CommandArg::Output("docs/preupdate-schedule-graph.png".to_string())
        ]
    ).unwrap();

    // Update Schedule Graph
    let _update_schedule_graph = bevy_mod_debugdump::schedule_graph_dot(
        &mut app,
        Update,
        &_schedule_graph_settings
    );
    let _update_schedule_g: graphviz_rust::dot_structures::Graph = parse(
        &_update_schedule_graph
    ).unwrap();
    let _update_schedule_graph_dot = exec(
        _update_schedule_g.clone(),
        &mut PrinterContext::default(),
        vec![Format::Dot.into(), CommandArg::Output("docs/update-schedule-graph.dot".to_string())]
    ).unwrap();
    let _update_schedule_graph_svg = exec(
        _update_schedule_g.clone(),
        &mut PrinterContext::default(),
        vec![Format::Svg.into(), CommandArg::Output("docs/update-schedule-graph.svg".to_string())]
    ).unwrap();
    let _update_schedule_graph_png = exec(
        _update_schedule_g.clone(),
        &mut PrinterContext::default(),
        vec![Format::Png.into(), CommandArg::Output("docs/update-schedule-graph.png".to_string())]
    ).unwrap();

    // PostUpdate Schedule Graph
    let _postupdate_schedule_graph = bevy_mod_debugdump::schedule_graph_dot(
        &mut app,
        PostUpdate,
        &_schedule_graph_settings
    );
    let _postupdate_schedule_g: graphviz_rust::dot_structures::Graph = parse(
        &_postupdate_schedule_graph
    ).unwrap();
    let _postupdate_schedule_graph_dot = exec(
        _postupdate_schedule_g.clone(),
        &mut PrinterContext::default(),
        vec![
            Format::Dot.into(),
            CommandArg::Output("docs/postupdate-schedule-graph.dot".to_string())
        ]
    ).unwrap();
    let _postupdate_schedule_graph_svg = exec(
        _postupdate_schedule_g.clone(),
        &mut PrinterContext::default(),
        vec![
            Format::Svg.into(),
            CommandArg::Output("docs/postupdate-schedule-graph.svg".to_string())
        ]
    ).unwrap();
    let _postupdate_schedule_graph_png = exec(
        _postupdate_schedule_g.clone(),
        &mut PrinterContext::default(),
        vec![
            Format::Png.into(),
            CommandArg::Output("docs/postupdate-schedule-graph.png".to_string())
        ]
    ).unwrap();
*/
    // Actually start the game now!
    app.run();
}

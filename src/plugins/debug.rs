/// this code is compiled only if debug assertions are enabled (debug mode)

use graphviz_rust::{ cmd::{ CommandArg, Format }, exec_dot };

use bevy::{
    prelude::*,
    utils::intern::Interned,
    ecs::schedule::ScheduleLabel,
    input::common_conditions::input_toggle_active,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::{ prelude::RapierDebugRenderPlugin, render::DebugRenderContext };

use super::{ gamestate::GameState, ui::fps_widget::{ spawn_fps_widget, FpsWidget } };
use crate::components::childof::debug_children;

// Adds L key as debug KeyCode for toggling physics wireframes.
pub fn toggle_physics_wireframes(
    mut ctx: ResMut<DebugRenderContext>,
    input: Res<ButtonInput<KeyCode>>
) {
    if input.just_pressed(KeyCode::F9) {
        ctx.enabled = !ctx.enabled;
    }
}

pub fn disable_physics_wireframes(mut ctx: ResMut<DebugRenderContext>) {
    ctx.enabled = false;
}

pub(crate) fn plugin(app: &mut App) {
    // Plugins
    app.add_plugins((
        // FpsWidget
        super::ui::fps_widget::plugin,
        // WorldInspectorPlugin
        WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F11)),
        // RapierDebugRenderPlugin
        RapierDebugRenderPlugin::default(),
        // StateInspectorPlugin
        bevy_inspector_egui::quick::StateInspectorPlugin::<GameState>
            ::default()
            .run_if(bevy::input::common_conditions::input_toggle_active(false, KeyCode::F10)),
    ));

    // Startup
    app.add_systems(Startup, (
        // Debug all parent and child elements with the Name component.
        debug_children,
        // Disable wireframes by default. I've seen enough collision boxes for now.
        disable_physics_wireframes,
    ));

    // Update
    // FpsWidget
    app.add_systems(Update, spawn_fps_widget.run_if(not(any_with_component::<FpsWidget>)));
    // DebugRenderContext - Rapier
    app.add_systems(Update, toggle_physics_wireframes);

    if let Err(err) = render_graphs(app) {
        error!("Error rendering graph: {}", err);
    }
}

pub(crate) fn make_log_plugin() -> impl Plugin {
    bevy::log::LogPlugin {
        level: bevy::log::Level::DEBUG,
        filter: "info,big_brain=debug,one_off=debug,separated=debug,wgpu_core=warn,wgpu_hal=warn".into(),
        update_subscriber: None,
    }
}

type IoResult = std::io::Result<()>;

/// ⚠️ debugdump 🐛 render system and render graphs when the RENDER_GRAPHS environment variable is set
fn render_graphs(app: &mut App) -> IoResult {
    if std::env::var_os("RENDER_GRAPHS") == None {
        return Ok(());
    }
    if let Some(schedules) = app.world.get_resource::<Schedules>() {
        render_schedules_graphs(
            app,
            schedules
                .iter()
                .map(|(_, s)| s.label())
                .collect::<Vec<_>>()
        )?;
    }
    render_render_graph(app)
}

fn render_render_graph(app: &mut App) -> IoResult {
    let settings = bevy_mod_debugdump::render_graph::Settings::default();

    let graph_str = bevy_mod_debugdump::render_graph_dot(app, &settings);
    render_graph(graph_str, "docs/graph/render")?;
    Ok(())
}

/// Renders the system graphs for a list of schedules
fn render_schedules_graphs(
    app: &mut App,
    schedules: impl AsRef<[Interned<dyn ScheduleLabel>]>
) -> IoResult {
    let settings = bevy_mod_debugdump::schedule_graph::Settings::default();

    for &label in schedules.as_ref() {
        let stem = format!("docs/graph/{:?}-schedule", label);
        let graph_str = bevy_mod_debugdump::schedule_graph_dot(app, label, &settings);
        render_graph(graph_str, stem)?;
    }
    Ok(())
}

/// Given a graphviz string, render it into dot, svg and png files
pub fn render_graph(graph_str: String, output_stem: impl AsRef<str>) -> std::io::Result<()> {
    exec_dot(
        graph_str.clone(),
        vec![Format::Dot.into(), CommandArg::Output(format!("{}.dot", output_stem.as_ref()))]
    )?;
    exec_dot(
        graph_str.clone(),
        vec![Format::Svg.into(), CommandArg::Output(format!("{}.svg", output_stem.as_ref()))]
    )?;
    exec_dot(
        graph_str,
        vec![Format::Png.into(), CommandArg::Output(format!("{}.png", output_stem.as_ref()))]
    )?;
    Ok(())
}

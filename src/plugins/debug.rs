/// this code is compiled only if debug assertions are enabled (debug mode)

use bevy::{
    app::{ Plugin, Update },
    input::{ common_conditions::input_toggle_active, ButtonInput },
    prelude::{ any_with_component, not, IntoSystemConfigs, KeyCode, Res, ResMut },
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::render::{ DebugRenderContext, RapierDebugRenderPlugin };

use super::ui::fps_widget::{ spawn_fps_widget, FpsWidget };

// Adds L key as debug KeyCode for toggling physics wireframes.
pub fn toggle_physics_wireframes(
    mut ctx: ResMut<DebugRenderContext>,
    input: Res<ButtonInput<KeyCode>>
) {
    if input.just_pressed(KeyCode::KeyL) {
        ctx.enabled = !ctx.enabled;
    }
}

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_plugins((
        // FpsWidget
        super::ui::fps_widget::plugin,
        // WorldInspectorPlugin
        WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F11)),
        RapierDebugRenderPlugin::default(),
    ));
    // FpsWidget
    app.add_systems(Update, spawn_fps_widget.run_if(not(any_with_component::<FpsWidget>)));
    // DebugRenderContext - Rapier
    app.add_systems(Update, toggle_physics_wireframes);
}

pub(crate) fn make_log_plugin() -> impl Plugin {
    bevy::log::LogPlugin {
        level: bevy::log::Level::DEBUG,
        filter: "info,separated=debug,wgpu_core=warn,wgpu_hal=warn".into(),
        update_subscriber: None,
    }
}

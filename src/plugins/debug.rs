/// this code is compiled only if debug assertions are enabled (debug mode)
use bevy::prelude::*;

use bevy::input::common_conditions::input_toggle_active;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use super::ui::fps_widget::{ spawn_fps_widget, FpsWidget };

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_plugins((
        // FpsWidget
        super::ui::fps_widget::plugin,
        // WorldInspectorPlugin
        WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F11)),
    ));

    // FpsWidget
    app.add_systems(Update, spawn_fps_widget.run_if(not(any_with_component::<FpsWidget>)));
}

pub(crate) fn make_log_plugin() -> impl Plugin {
    bevy::log::LogPlugin {
        level: bevy::log::Level::DEBUG,
        filter: "info,pattern=debug,wgpu_core=warn,wgpu_hal=warn,pattern=debug".into(),
        update_subscriber: None,
    }
}

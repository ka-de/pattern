// Turn clippy into a real bitch
#![warn(clippy::all, clippy::pedantic)]

mod components;

use bevy::input::common_conditions::input_toggle_active;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_tweening::*;

fn set_framepace(mut settings: ResMut<bevy_framepace::FramepaceSettings>) {
    use bevy_framepace::Limiter;
    settings.limiter = Limiter::Off;
}

fn toggle_framepace(
    mut settings: ResMut<bevy_framepace::FramepaceSettings>,
    input: Res<ButtonInput<KeyCode>>
) {
    if input.just_pressed(KeyCode::F9) {
        use bevy_framepace::Limiter;
        settings.limiter = match settings.limiter {
            Limiter::Auto => Limiter::Off,
            Limiter::Off => Limiter::from_framerate(60.0),
            Limiter::Manual(_) => Limiter::Auto,
        };
    }
}

fn main() {
    // this code is compiled only if debug assertions are enabled (debug mode)
    #[cfg(debug_assertions)]
    let log_plugin = LogPlugin {
        level: bevy::log::Level::DEBUG,
        filter: "info,pattern=debug,wgpu_core=warn,wgpu_hal=warn,pattern=debug".into(),
        update_subscriber: None,
    };

    // this code is compiled only if debug assertions are disabled (release mode)
    #[cfg(not(debug_assertions))]
    let log_plugin = LogPlugin {
        level: bevy::log::Level::INFO,
        filter: "warning,pattern=info".into(),
        update_subscriber: None,
    };

    #[cfg(target_arch = "wasm32")]
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            canvas: Some("#pattern-canvas".into()),
            ..default()
        }),
        ..default()
    };

    #[cfg(not(target_arch = "wasm32"))]
    let window_plugin = WindowPlugin::default();

    /*
     * Debugging keyboard shortcuts:
     *
     * F9  - Toggle Framepacing
     * F10 - StateInspector (GameState)
     * F11 - WorldInspector
     * F12 - PerformanceUI (Not yet implemented)
     */

    let mut app = App::new();

    app.insert_resource(Msaa::Off) // Disable Multi-Sample Anti-Aliasing
        // DefaultPlugins
        .add_plugins((
            DefaultPlugins.set(window_plugin).set(ImagePlugin::default_nearest()).set(log_plugin),
            // Tweening
            TweeningPlugin,
            // Frame Pacing
            bevy_framepace::FramepacePlugin,
            components::gamestate::game_state_plugin,
            // PerformanceUI
            components::perfui::setup_perf_ui,
            components::ui::setup_ui,
            components::systems::setup_ldtk,
        ))
        .add_systems(Update, toggle_framepace)
        .add_systems(Startup, set_framepace);

    #[cfg(debug_assertions)]
    app.add_plugins(bevy_framepace::debug::DiagnosticsPlugin)
        // FrameTimeDiagnosticsPlugin
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        // WorldInspectorPlugin
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F11))
        );

    app.run();
}

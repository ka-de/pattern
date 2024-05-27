// Turn clippy into a real bitch
#![warn(clippy::all, clippy::pedantic)]

mod components;

use bevy::render::camera::ScalingMode;
use bevy_ecs_ldtk::{ LdtkPlugin, LdtkSettings, LevelSelection, LevelSpawnBehavior, SetClearColor };
use bevy_rapier2d::plugin::{ NoUserData, RapierConfiguration, RapierPhysicsPlugin, TimestepMode };
use components::gamestate::{
    set_state_loading,
    set_state_mainmenu,
    set_state_playing,
    set_state_splashscreen,
    GameState,
};
use components::{ CustomPerfUiAppExt as _, CustomSystemsAppExt as _ };

use bevy::input::common_conditions::input_toggle_active;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_inspector_egui::quick::StateInspectorPlugin;
use bevy_splashscreen::prelude::*;

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
    // Splash Screen Configuration
    let config = SplashScreenConfiguration {
        run_state: GameState::SplashScreen,
        next_state: GameState::Playing,
        // TODO: Implement this motherfucker!
        //next_state: GameState::MainMenu,
        path: String::from("splash_screens"),
        custom_size: Some(Vec2::ONE),
        splash_timer: 3.0,
        clear_color: Color::BLACK,
        camera_scaling_mode: ScalingMode::Fixed { width: 1.0, height: 1.0 },
    };

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

    App::new()
        // Disable Multi-Sample Anti-Aliasing
        .insert_resource(Msaa::Off)
        // DefaultPlugins
        .add_plugins(
            DefaultPlugins.set(window_plugin).set(ImagePlugin::default_nearest()).set(log_plugin)
        )
        // Frame Pacing
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_plugins(bevy_framepace::debug::DiagnosticsPlugin)
        .add_systems(Update, toggle_framepace)
        // GameState
        .init_state::<GameState>()
        .register_type::<GameState>()
        // Splash Screen
        .add_plugins(SplashScreenPlugin(config))
        .init_resource::<bevy_progress::ProgressCounter>()
        //.init_resource::<bevy_splashscreen::resources::splash_screen_images::SplashScreenImages>()
        .add_systems(OnEnter(GameState::SplashScreen), set_state_splashscreen)
        .add_systems(OnEnter(GameState::Loading), set_state_loading)
        .add_systems(OnEnter(GameState::MainMenu), set_state_mainmenu)
        .add_systems(OnEnter(GameState::Playing), set_state_playing)
        // FrameTimeDiagnosticsPlugin
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        // WorldInspectorPlugin
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F11))
        )
        // StateInspectorPlugin
        .add_plugins(
            StateInspectorPlugin::<GameState>
                ::default()
                .run_if(input_toggle_active(false, KeyCode::F10))
        )
        // PerformanceUI
        .add_custom_perf_ui()
        // Custom Systems
        .add_custom_systems()
        //
        .add_plugins((LdtkPlugin, RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0)))
        .run();
}

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_asset_loader::prelude::*;
use crate::plugins::splashscreen::{ SplashScreenPlugin, SplashScreenConfiguration };

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum GameState {
    #[default]
    SplashScreen,
    Playing,
    Loading,
    MainMenu,
}

pub fn set_state_splashscreen() {
    info!("Set GameState: SplashScreen");
}

pub fn set_state_loading() {
    info!("Set GameState: Loading");
}

pub fn set_state_mainmenu() {
    info!("Set GameState: MainMenu");
}

pub fn set_state_playing() {
    info!("Set GameState: Playing");
}

pub fn game_state_plugin(app: &mut App) {
    // Splash Screen Configuration
    let config = SplashScreenConfiguration {
        run_state: GameState::SplashScreen,
        next_state: GameState::Playing,
        // TODO: Implement this motherfucker!
        //next_state: GameState::MainMenu,
        images: StandardDynamicAsset::Files { paths: vec!["splash_screens/splash.png".to_owned()] },
        custom_size: Some(Vec2::ONE),
        splash_timer: 3.0,
        clear_color: Color::BLACK,
        camera_scaling_mode: ScalingMode::Fixed { width: 1.0, height: 1.0 },
    };

    // GameState
    app.init_state::<GameState>()
        .register_type::<GameState>()
        .add_loading_state(LoadingState::new(GameState::SplashScreen))
        .add_systems(OnEnter(GameState::SplashScreen), set_state_splashscreen)
        .add_systems(OnEnter(GameState::Loading), set_state_loading)
        .add_systems(OnEnter(GameState::MainMenu), set_state_mainmenu)
        .add_systems(OnEnter(GameState::Playing), set_state_playing)
        // Splash Screen
        .add_plugins(SplashScreenPlugin(config))
        .init_resource::<bevy_progress::ProgressCounter>();

    // StateInspectorPlugin
    #[cfg(debug_assertions)]
    app.add_plugins(
        bevy_inspector_egui::quick::StateInspectorPlugin::<GameState>
            ::default()
            .run_if(bevy::input::common_conditions::input_toggle_active(false, KeyCode::F10))
    );
}

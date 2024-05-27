use bevy::{
    app::{ App, Startup, Update },
    asset::{ AssetApp, AssetServer },
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{ schedule::States, system::{ Commands, Res } },
    log::info,
    math::Vec2,
    reflect::Reflect,
};
use bevy_ecs_ldtk::{
    app::{ LdtkEntityAppExt, LdtkIntCellAppExt },
    LdtkPlugin,
    LdtkSettings,
    LevelSelection,
    LevelSpawnBehavior,
    SetClearColor,
};
use bevy_rapier2d::plugin::{ NoUserData, RapierConfiguration, RapierPhysicsPlugin, TimestepMode };

/**
 * The GameState
 *
 * TODO: Write the doc lmao
 */
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

pub fn set_state_playing(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Set GameState: Playing");
    // 🎥
    let camera = Camera2dBundle::default();
    commands.spawn(camera);
    super::systems::spawn_ldtk_world(commands, asset_server);
}

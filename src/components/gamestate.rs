use bevy::{
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{ schedule::States, system::Commands },
    log::info,
    reflect::Reflect,
};

/*
 * The GameState
 *
 *
 */

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum GameState {
    #[default]
    SplashScreen,
    Loading,
    MainMenu,
    Playing,
}

pub fn set_state_splashscreen() {
    info!("Set State: SplashScreen");
}

pub fn set_state_loading() {
    info!("Set State: Loading");
}

pub fn set_state_mainmenu(mut commands: Commands) {
    let camera = Camera2dBundle::default();
    commands.spawn(camera);
    info!("Set State: MainMenu");
}

pub fn set_state_playing() {
    info!("Set State: Playing");
}

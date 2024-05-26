use bevy::{ ecs::schedule::States, reflect::Reflect };

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
    println!("SplashScreen...")
}

pub fn set_state_loading() {
    println!("Loading...")
}

pub fn set_state_mainmenu() {
    println!("MainMenu...")
}

pub fn set_state_playing() {
    println!("Playing...")
}

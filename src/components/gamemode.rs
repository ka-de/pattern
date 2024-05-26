use bevy::{ ecs::schedule::States, reflect::Reflect };

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum GameMode {
    #[default]
    WolfMode,
    OtterMode,
}
pub fn set_gamemode_wolfmode() {
    println!("WolfMode Selected...")
}
pub fn set_gamemode_ottermode() {
    println!("OtterMode Selected...")
}

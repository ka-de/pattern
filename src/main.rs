use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_cam)
        .run()
}

fn setup_cam(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default());
}

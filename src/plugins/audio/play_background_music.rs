use bevy::prelude::*;

#[derive(Component)]
struct BackgroundMusic;

// ⚠️ TODO: Currently very dumb, just plays one music on repeat!
pub fn play_background_music(asset_server: Res<AssetServer>, mut commands: Commands) {
    // Create an entity dedicated to playing our background music
    commands.spawn((
        AudioBundle {
            source: asset_server.load("music/garam_masala_wip.ogg"),
            settings: PlaybackSettings::LOOP,
        },
        BackgroundMusic,
    ));
}

pub fn reset_speed(music_controller: Query<&AudioSink, With<BackgroundMusic>>) {
    if let Ok(sink) = music_controller.get_single() {
        sink.set_speed(1.0);
    }
}

fn pause(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    music_controller: Query<&AudioSink, With<BackgroundMusic>>
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        if let Ok(sink) = music_controller.get_single() {
            sink.toggle();
        }
    }
}

use bevy::{
    ecs::system::{ Res, Commands },
    audio::{ AudioBundle, PlaybackSettings },
    asset::AssetServer,
};

// ⚠️ TODO: Currently very dumb, just plays one music on repeat!
pub fn play_background_music(asset_server: Res<AssetServer>, mut commands: Commands) {
    // Create an entity dedicated to playing our background music
    commands.spawn(AudioBundle {
        source: asset_server.load("music/garam_masala_wip.ogg"),
        settings: PlaybackSettings::LOOP,
    });
}

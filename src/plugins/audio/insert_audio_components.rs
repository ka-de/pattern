use bevy::{
    asset::AssetServer,
    audio::{ AudioBundle, PlaybackSettings, SpatialListener, SpatialScale },
    core::Name,
    ecs::{ entity::Entity, query::Added, system::{ Commands, Query, Res } },
    log::info,
};
use bevy_ecs_ldtk::{ prelude::LdtkFields, EntityInstance };

use crate::components::player::Player;

/// Insert SpatialListener into the Player entity
/// FIXME ⚠️ remove debug code and Name query
pub fn insert_spatial_listener(
    mut commands: Commands,
    added_player: Query<(Entity, &Name), Added<Player>>
) {
    for (player_entity, name) in &added_player {
        info!("Inserted spatial listener for {}", name);
        commands.entity(player_entity).insert(SpatialListener::default());
    }
}

/// Insert SFX AudioBundle into the LDtk EntityInstance with a `sfx` field
/// FIXME ⚠️ remove debug code and Name query
pub fn insert_audio_sources(
    mut commands: Commands,
    added_player: Query<(Entity, &EntityInstance, &Name), Added<bevy_ecs_ldtk::EntityInstance>>,
    asset_server: Res<AssetServer>
) {
    for (entity, ldtk_instance, name) in &added_player {
        if let Ok(sfx) = ldtk_instance.get_string_field("sfx") {
            let sfx_path = format!("sfx/{}", sfx);
            let spatial_scale = ldtk_instance
                .get_float_field("spatial_scale")
                .ok()
                .map(|s| SpatialScale::new(super::AUDIO_SCALE * s));
            info!("Inserted SFX {} for {}", sfx, name);

            commands.entity(entity).insert(AudioBundle {
                source: asset_server.load(sfx_path),
                settings: PlaybackSettings {
                    spatial: true,
                    spatial_scale,
                    ..PlaybackSettings::LOOP
                },
            });
        }
    }
}

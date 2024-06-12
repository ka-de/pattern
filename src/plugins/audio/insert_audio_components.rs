use std::time::Duration;

use bevy::{
    asset::AssetServer,
    audio::{ PlaybackSettings, SpatialListener, Volume, SpatialScale },
    core::Name,
    ecs::{ entity::Entity, query::Added, system::{ Commands, Query, Res } },
    log::info,
};
use bevy_ecs_ldtk::{ prelude::LdtkFields, EntityInstance };
use rand::{ Rng, thread_rng };

use super::delayed_audio_source::DelayedPlaybackSettings;
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
pub fn insert_audio_sources(
    mut commands: Commands,
    added_player: Query<(Entity, &EntityInstance), Added<bevy_ecs_ldtk::EntityInstance>>,
    asset_server: Res<AssetServer>
) {
    if added_player.is_empty() {
        return;
    }
    let mut rng = thread_rng();
    for (entity, ldtk_instance) in &added_player {
        if let Ok(sfx) = ldtk_instance.get_string_field("sfx") {
            let sfx_path = format!("sfx/{}", sfx);
            let volume = match ldtk_instance.get_float_field("volume") {
                Ok(&v) => Volume::new(v),
                Err(_) => Volume::default(),
            };
            let spatial_scale = ldtk_instance
                .get_float_field("spatial_scale")
                .ok()
                .map(|s| SpatialScale::new(super::AUDIO_SCALE * s));
            let source = asset_server.load(sfx_path);
            let delay: Duration = Duration::from_secs_f32(rng.gen_range(0.0..3.0));

            commands.entity(entity).insert(DelayedPlaybackSettings {
                source,
                delay,
                settings: PlaybackSettings {
                    spatial: true,
                    volume,
                    spatial_scale,
                    ..PlaybackSettings::LOOP
                },
            });
        }
    }
}

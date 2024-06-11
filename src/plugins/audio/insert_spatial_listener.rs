use bevy::{
    log::info,
    core::Name,
    audio::SpatialListener,
    ecs::{ entity::Entity, query::Added, system::{ Query, Commands } },
};

use crate::components::player::Player;

pub fn insert_spatial_listener(
    mut commands: Commands,
    added_player: Query<(Entity, &Name), Added<Player>>
) {
    for (player_entity, name) in &added_player {
        info!("Inserted spatial listener for {}", name);
        commands.entity(player_entity).insert(SpatialListener::default());
    }
}

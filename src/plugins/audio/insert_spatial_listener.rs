use bevy::ecs;
use bevy::log::info;

pub fn insert_spatial_listener(
    mut commands: Commands,
    added_player: ecs::system::Query<
        (ecs::entity::Entity, &bevy::core::Name),
        ecs::query::Added<components::player::Player>
    >
) {
    for (player_entity, name) in &added_player {
        info!("Inserted spatial listener for {}", name);
        commands.entity(player_entity).insert(SpatialListener::default());
    }
}

use bevy::{
    core::Name,
    ecs::{ component::Component, entity::Entity, query::With, system::{ Query, Res } },
    log,
    math::Vec2,
    transform::components::GlobalTransform,
};
use bevy_rapier2d::{ pipeline::QueryFilter, plugin::RapierContext };

use crate::entities::player::Player;

/// Component applied to entities that should detect line of sight to the player
#[derive(Component, Clone)]
pub(crate) struct LineOfSight {
    max_distance: f32,
    last_sighted: Option<Vec2>,
    in_sight: bool,
}

/// Component applied to entities that seek the player
impl Default for LineOfSight {
    fn default() -> Self {
        Self { max_distance: 250.0, last_sighted: None, in_sight: false }
    }
}

pub(crate) fn line_of_sight(
    player: Query<(Entity, &GlobalTransform), With<Player>>,
    mut observers: Query<(&mut LineOfSight, &GlobalTransform, Entity, Option<&Name>)>,
    rapier_context: Res<RapierContext>
) {
    let Ok((player_entity, player_transform)) = player.get_single() else {
        return;
    };
    let player_pos = player_transform.translation().truncate();

    // Iterate over entities having a LineOfSight component
    for (mut line_of_sight, observer_transform, observer_entity, observer_name) in &mut observers {
        let observer_pos = observer_transform.translation().truncate();
        let vector = player_pos - observer_pos;
        let max_distance = line_of_sight.max_distance;
        let distance = vector.length();
        if distance > max_distance {
            continue;
        }
        let ray_dir = vector.normalize();

        // Cast ray from the observer toward the player
        let stalked = if
            let Some((collided_entity, toi)) = rapier_context.cast_ray(
                observer_pos,
                ray_dir,
                max_distance,
                false,
                // FIXME: make sight obstacles (wall etc) into a group and use .groups() to only hit player and obstacles
                QueryFilter::new().exclude_sensors().exclude_collider(observer_entity)
            )
        {
            // TODO: remove debug
            if collided_entity == player_entity && !line_of_sight.in_sight {
                log::debug!(
                    "{:?}({:?}) sees the player! toi={}",
                    observer_name,
                    observer_entity,
                    toi
                );
            }
            collided_entity == player_entity
        } else {
            false
        };

        // Minimal updates
        if stalked {
            let last_sighted = Some(player_pos);
            if !line_of_sight.in_sight || line_of_sight.last_sighted != last_sighted {
                let los_mut = line_of_sight.as_mut();
                los_mut.last_sighted = last_sighted;
                los_mut.in_sight = true;
            }
        } else {
            if line_of_sight.in_sight {
                line_of_sight.in_sight = false;
                // TODO: remove debug
                log::debug!("{:?}({:?}) lost the player.", observer_name, observer_entity);
            }
        }
    }
}

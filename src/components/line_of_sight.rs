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
    last_sigthed: Option<Vec2>,
    in_sight: bool,
}

/// Component applied to entities that seek the player
impl Default for LineOfSight {
    fn default() -> Self {
        Self { max_distance: 250.0, last_sigthed: None, in_sight: false }
    }
}

pub(crate) fn line_of_sight(
    player: Query<(Entity, &GlobalTransform), With<Player>>,
    mut stalkers: Query<(&mut LineOfSight, &GlobalTransform, Entity, Option<&Name>)>,
    rapier_context: Res<RapierContext>
) {
    let Ok((player_entity, player_transofrm)) = player.get_single() else {
        return;
    };
    let player_pos = player_transofrm.translation().truncate();

    // Iterate over entities having a LineOfSight component
    for (mut line_of_sight, stalker_tranform, stalker_entity, stalker_name) in &mut stalkers {
        let stalker_pos = stalker_tranform.translation().truncate();
        let vector = player_pos - stalker_pos;
        let max_distance = line_of_sight.max_distance;
        let distance = vector.length();
        if distance > max_distance {
            continue;
        }
        let ray_dir = vector.normalize();

        // Cast ray from the stalker toward the player
        let stalked = if
            let Some((collided_entity, toi)) = rapier_context.cast_ray(
                stalker_pos,
                ray_dir,
                max_distance,
                false,
                // FIXME: make sight obstacles (wall etc) into a group and use .groups() to only hit player and obstacles
                QueryFilter::new().exclude_sensors().exclude_collider(stalker_entity)
            )
        {
            // TODO: remove debug
            if collided_entity == player_entity && !line_of_sight.in_sight {
                log::debug!("{:?}({:?}) sees the player! toi={}", stalker_name, stalker_entity, toi);
            }
            collided_entity == player_entity
        } else {
            false
        };

        // Minimal updates
        if stalked {
            let last_sighted = Some(player_pos);
            if !line_of_sight.in_sight || line_of_sight.last_sigthed != last_sighted {
                let los_mut = line_of_sight.as_mut();
                los_mut.last_sigthed = last_sighted;
                los_mut.in_sight = true;
            }
        } else {
            if line_of_sight.in_sight {
                line_of_sight.in_sight = false;
                // TODO: remove debug
                log::debug!("{:?}({:?}) lost the player.", stalker_name, stalker_entity);
            }
        }
    }
}

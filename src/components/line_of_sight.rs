use bevy::{
    core::Name,
    ecs::{ component::Component, entity::Entity, query::With, system::{ Query, Res } },
    transform::components::GlobalTransform,
    log,
};
use bevy_rapier2d::{ pipeline::QueryFilter, plugin::RapierContext };

use crate::entities::player::Player;

/// Component applied to entities that should detect line of sight to the player
#[derive(Component, Clone)]
pub(crate) struct LineOfSight {
    max_distance: f32,
}

/// Component applied to entities that seek the player
impl Default for LineOfSight {
    fn default() -> Self {
        Self { max_distance: 250.0 }
    }
}

pub(crate) fn line_of_sight(
    player: Query<(Entity, &GlobalTransform), With<Player>>,
    stalkers: Query<(&LineOfSight, &GlobalTransform, Entity, Option<&Name>)>,
    rapier_context: Res<RapierContext>
) {
    let Ok((player_entity, player_transofrm)) = player.get_single() else {
        return;
    };
    let player_pos = player_transofrm.translation().truncate();

    for (
        LineOfSight { max_distance },
        stalker_tranform,
        stalker_entity,
        stalker_name,
    ) in &stalkers {
        let stalker_pos = stalker_tranform.translation().truncate();
        let vector = player_pos - stalker_pos;
        let ray_dir = vector.normalize();

        if
            let Some((collided_entity, toi)) = rapier_context.cast_ray(
                stalker_pos,
                ray_dir,
                *max_distance,
                false,
                // FIXME: make sight obstacles (wall etc) into a group and use .groups() to only hit player and obstacles
                QueryFilter::new().exclude_sensors().exclude_collider(stalker_entity)
            )
        {
            if collided_entity == player_entity {
                // FIXME do something here
                log::info!("{:?}({:?}) sees the player! toi={}", stalker_name, stalker_entity, toi);
            }
        }
    }
}

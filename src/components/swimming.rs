use std::collections::HashSet;
use bevy::ecs::{
    component::Component,
    entity::Entity,
    event::EventReader,
    query::With,
    system::Query,
};
use bevy_rapier2d::pipeline::CollisionEvent;

use crate::plugins::rapier_utils::reciprocal_collisions;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Swimmable;

// Attach this component to any entity to allow them to swim
#[derive(Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Swimmer {
    pub swimming: bool,
    pub intersecting_swimmables: HashSet<Entity>,
}

pub fn detect_swim_range(
    mut swimmers: Query<&mut Swimmer>,
    swimmables: Query<Entity, With<Swimmable>>,
    mut collisions: EventReader<CollisionEvent>
) {
    reciprocal_collisions(&mut collisions, move |collider_a, collider_b, _, start| {
        if
            let (Ok(mut swimmer), Ok(swimmable)) = (
                swimmers.get_mut(*collider_a),
                swimmables.get(*collider_b),
            )
        {
            if start {
                swimmer.intersecting_swimmables.insert(swimmable);
            } else {
                swimmer.intersecting_swimmables.remove(&swimmable);
            }
            true
        } else {
            false
        }
    });
}

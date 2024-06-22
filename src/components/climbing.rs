use std::collections::HashSet;
use bevy::prelude::*;
use bevy_rapier2d::{ dynamics::GravityScale, pipeline::CollisionEvent };

use crate::plugins::rapier_utils::reciprocal_collisions;

// Attach this to any component to allow the player (or any climber entity) to climb up and
// down on it.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Climbable;

// Attach this component to any entity to allow them to climb up ladders.
#[derive(Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Climber {
    pub climbing: bool,
    pub intersecting_climbables: HashSet<Entity>,
}

// Checks for collision events between climbers and climbable entities.
// If a collision starts, the climbable entity is added to the climber’s set of intersecting climbables.
// If a collision stops, the climbable entity is removed from the set.
pub fn detect_climb_range(
    mut climbers: Query<&mut Climber>,
    climbables: Query<Entity, With<Climbable>>,
    mut collisions: EventReader<CollisionEvent>
) {
    reciprocal_collisions(&mut collisions, move |collider_a, collider_b, _, start| {
        if
            let (Ok(mut climber), Ok(climbable)) = (
                climbers.get_mut(*collider_a),
                climbables.get(*collider_b),
            )
        {
            if start {
                climber.intersecting_climbables.insert(climbable);
            } else {
                climber.intersecting_climbables.remove(&climbable);
            }
            true
        } else {
            false
        }
    });
}

// Checks if a climber entity is climbing.
// If it is, the gravity scale is set to 0.0, effectively ignoring gravity.
// If the climber is not climbing, the gravity scale is set back to 1.0.
pub fn ignore_gravity_if_climbing(
    mut query: Query<(&Climber, &mut GravityScale), Changed<Climber>>
) {
    for (climber, mut gravity_scale) in &mut query {
        if climber.climbing {
            gravity_scale.0 = 0.0;
        } else {
            gravity_scale.0 = 1.0;
        }
    }
}

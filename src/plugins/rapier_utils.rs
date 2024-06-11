use bevy::ecs::{ entity::Entity, event::EventReader };
use bevy_rapier2d::{ pipeline::CollisionEvent, rapier::geometry::CollisionEventFlags };

/// Apply the function to the permutations of colliding entities
#[inline]
pub fn reciprocal_collisions<Handler: FnMut(&Entity, &Entity, &CollisionEventFlags, bool) -> bool>(
    collisions: &mut EventReader<CollisionEvent>,
    mut handler: Handler
) {
    for collision in collisions.read() {
        let (collider_a, collider_b, flags, start) = match collision {
            CollisionEvent::Started(collider_a, collider_b, flags) =>
                (collider_a, collider_b, flags, true),
            CollisionEvent::Stopped(collider_a, collider_b, flags) =>
                (collider_a, collider_b, flags, false),
        };
        if !handler(collider_a, collider_b, flags, start) {
            handler(collider_b, collider_a, flags, start);
        }
    }
}

use std::collections::HashSet;

use bevy::{
    core::Name,
    ecs::{
        component::Component,
        entity::Entity,
        event::EventReader,
        query::{ Added, Changed, With, Without },
        system::{ Commands, Query },
    },
    hierarchy::BuildChildren,
    math::{ Vec2, Vec3 },
    transform::components::{ GlobalTransform, Transform },
};
use bevy_rapier2d::{ geometry::{ ActiveEvents, Collider, Sensor }, pipeline::CollisionEvent };

use crate::plugins::rapier_utils::reciprocal_collisions;

#[derive(Clone, Default, Component)]
pub struct GroundDetection {
    pub on_ground: bool,
}

#[derive(Component)]
pub struct GroundSensor {
    pub ground_detection_entity: Entity,
    pub intersecting_ground_entities: HashSet<Entity>,
}

pub fn spawn_ground_sensor(
    mut commands: Commands,
    detect_ground_for: Query<(Entity, &Collider), Added<GroundDetection>>
) {
    for (entity, shape) in &detect_ground_for {
        if let Some(cuboid) = shape.as_cuboid() {
            let Vec2 { x: half_extents_x, y: half_extents_y } = cuboid.half_extents();

            let detector_shape = Collider::cuboid(half_extents_x / 2.0, 2.0);

            let sensor_translation = Vec3::new(0.0, -half_extents_y, 0.0);

            commands.entity(entity).with_children(|builder| {
                builder
                    .spawn_empty()
                    .insert(Name::new("ground_sensor"))
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(detector_shape)
                    .insert(Sensor)
                    .insert(Transform::from_translation(sensor_translation))
                    .insert(GlobalTransform::default())
                    .insert(GroundSensor {
                        ground_detection_entity: entity,
                        intersecting_ground_entities: HashSet::new(),
                    });
            });
        }
    }
}

pub fn ground_detection(
    mut ground_sensors: Query<&mut GroundSensor>,
    mut collisions: EventReader<CollisionEvent>,
    collidables: Query<Entity, (With<Collider>, Without<Sensor>)>
) {
    reciprocal_collisions(&mut collisions, move |e1, e2, _, start| {
        if let (true, Ok(mut sensor)) = (collidables.contains(*e1), ground_sensors.get_mut(*e2)) {
            if start {
                sensor.intersecting_ground_entities.insert(*e1);
            } else {
                sensor.intersecting_ground_entities.remove(e1);
            }
            true
        } else {
            false
        }
    });
}

pub fn update_on_ground(
    mut ground_detectors: Query<&mut GroundDetection>,
    ground_sensors: Query<&GroundSensor, Changed<GroundSensor>>
) {
    for sensor in &ground_sensors {
        if let Ok(mut ground_detection) = ground_detectors.get_mut(sensor.ground_detection_entity) {
            ground_detection.on_ground = !sensor.intersecting_ground_entities.is_empty();
        }
    }
}

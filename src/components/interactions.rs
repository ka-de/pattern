use std::collections::HashSet;

use bevy::{
    core::Name,
    ecs::{
        component::Component,
        entity::Entity,
        event::EventReader,
        query::{ Added, Changed, With },
        system::{ Commands, Query },
    },
    hierarchy::BuildChildren,
    log::info,
    math::Vec2,
    transform::components::Transform,
};
use bevy_ecs_ldtk::{ ldtk::ldtk_fields::LdtkFields, EntityInstance };
use bevy_rapier2d::{ geometry::{ ActiveEvents, Collider, Sensor }, pipeline::CollisionEvent };

use super::player::Player;

#[derive(Clone, Component)]
pub struct Interactive {
    pub name: String,
}

pub fn setup_interactive_entity(
    mut commands: Commands,
    query: Query<(Entity, &EntityInstance), Added<EntityInstance>>
) {
    for (entity, ldtk_entity) in query.iter() {
        if
            let (Ok(name), Ok(true)) = (
                ldtk_entity.get_string_field("name"),
                ldtk_entity.get_bool_field("hasDialogue"),
            )
        {
            info!("New interactive {}: {}", ldtk_entity.identifier, name);
            commands.entity(entity).insert(Interactive { name: name.into() });
        }
    }
}

#[derive(Component)]
pub struct InteractionSensor {
    pub interaction_detection_entity: Entity,
    pub intersecting_entities: HashSet<Entity>,
}

pub fn spawn_interaction_sensor(
    mut commands: Commands,
    detect_interaction_for: Query<(Entity, &Collider), Added<Player>>
) {
    for (entity, shape) in &detect_interaction_for {
        if let Some(cuboid) = shape.as_cuboid() {
            let Vec2 { x: half_extents_x, y: half_extents_y } = cuboid.half_extents();
            let detector_shape = Collider::cuboid(half_extents_x * 4.0, half_extents_y * 1.5);
            commands.entity(entity).with_children(|builder| {
                builder
                    .spawn_empty()
                    .insert(Name::new("interaction_sensor"))
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(detector_shape)
                    .insert(Sensor)
                    .insert(InteractionSensor {
                        interaction_detection_entity: entity,
                        intersecting_entities: HashSet::new(),
                    });
            });
        }
    }
}

pub fn interaction_detection(
    mut interactors: Query<&mut InteractionSensor>,
    interactives: Query<Entity, With<Interactive>>,
    mut collisions: EventReader<CollisionEvent>
) {
    crate::rapier_utils::reciprocal_collisions(
        &mut collisions,
        move |interactor_entity, interactive_entity, _, start| {
            if
                let (Ok(mut interactor), true) = (
                    interactors.get_mut(*interactor_entity),
                    interactives.contains(*interactive_entity),
                )
            {
                if start {
                    info!(
                        "{:?} starts interacting with {:?}",
                        interactor_entity,
                        interactive_entity
                    );
                    interactor.intersecting_entities.insert(*interactive_entity);
                } else {
                    interactor.intersecting_entities.remove(interactive_entity);
                }
                true
            } else {
                false
            }
        }
    );
}

// pub fn update_interaction(
//     mut interactive: Query<(&Interactive, &Transform, &EntityInstance)>,
//     interaction_sensors: Query<(&InteractionSensor, &Name, &Transform), Changed<InteractionSensor>>
// ) {
//     for (sensor, sensor_name, sensor_transform) in &interaction_sensors {
//         for (interactive_component, interactive_tranform, ei) in interactive.iter_many(
//             &sensor.intersecting_entities
//         ) {
//             let distance = sensor_transform.translation.distance(interactive_tranform.translation);
//             info!(
//                 "{} interacting with a {} named {} at {}px",
//                 sensor_name,
//                 ei.identifier,
//                 interactive_component.name,
//                 distance
//             );
//         }
//     }
// }

pub fn update_interaction(
    mut interactive: Query<(&Interactive, Entity)>,
    interaction_sensors: Query<(&InteractionSensor, Entity), Changed<InteractionSensor>>
) {
    for (sensor, sensor_entity) in &interaction_sensors {
        for (interactive_component, interactive_entity) in interactive.iter_many(
            &sensor.intersecting_entities
        ) {
            info!(
                "{:?} interacting with {:?}:{}",
                sensor_entity,
                interactive_entity,
                interactive_component.name
            );
        }
    }
}

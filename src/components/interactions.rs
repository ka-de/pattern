use std::collections::HashSet;

use bevy::{
    core::Name,
    render::color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        event::EventReader,
        query::{ Added, Changed, With },
        schedule::{ IntoSystemConfigs, SystemConfigs },
        system::{ Commands, Query },
    },
    hierarchy::{ Parent, PushChild },
    log::info,
    math::Vec2,
    transform::components::GlobalTransform,
};
use bevy_ecs_ldtk::{ ldtk::ldtk_fields::LdtkFields, EntityInstance };
use bevy_rapier2d::{
    geometry::{ ActiveEvents, Collider, Sensor },
    pipeline::CollisionEvent,
    render::ColliderDebugColor,
};

use super::player::Player;

#[derive(Component, Default)]
pub struct InteractionSensor {
    pub intersecting_entities: HashSet<Entity>,
    pub closest_entity: Option<Entity>,
}

/// Spawn a sensing region around the Player.
///
/// The sensor is spawn as a children to the Player. It contains an
/// [`InteractionSensor`] component that tracks interactive entities in range
/// and the closest one.
fn spawn_interaction_sensor(
    mut commands: Commands,
    mut detect_interaction_for: Query<(Entity, &Collider), Added<Player>>
) {
    for (parent, shape) in &mut detect_interaction_for {
        if let Some(cuboid) = shape.as_cuboid() {
            let Vec2 { x: half_extents_x, y: half_extents_y } = cuboid.half_extents();
            let mut sensor_cmds = commands.spawn((
                ActiveEvents::COLLISION_EVENTS,
                Collider::cuboid(half_extents_x * 4.0, half_extents_y * 1.5),
                Sensor,
                InteractionSensor::default(),
            ));
            #[cfg(debug_assertions)]
            sensor_cmds.insert((
                Name::new("interaction_sensor"),
                ColliderDebugColor(Color::rgb(0.0, 1.0, 0.0)),
            ));
            let child = sensor_cmds.id();
            commands.add(PushChild { parent, child });
        }
    }
}

#[derive(Component)]
pub struct Interactive {
    pub name: String,
}

/// Adds the [`Interactive`] component to LDtk entities that have a name and the
/// `hasDialogue` field set to true
fn setup_interactive_entity(
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

/// System collecting collision events of the interaction sensor with
/// interactive entities
fn interaction_detection(
    mut interaction_sensors: Query<&mut InteractionSensor>,
    interactive_entities: Query<Entity, With<Interactive>>,
    mut collisions: EventReader<CollisionEvent>
) {
    crate::rapier_utils::reciprocal_collisions(
        &mut collisions,
        move |interactor_entity, interactive_entity, _, start| {
            if
                let (Ok(mut interactor), true) = (
                    interaction_sensors.get_mut(*interactor_entity),
                    interactive_entities.contains(*interactive_entity),
                )
            {
                let set = &mut interactor.intersecting_entities;
                if start {
                    set.insert(*interactive_entity);
                } else {
                    set.remove(interactive_entity);
                }
                true
            } else {
                false
            }
        }
    );
}

/// System that tracks distances between interactive entities and the sensor, in
/// order to elect the closest interactive entity.
fn update_interactions(
    mut interaction_sensors: Query<(&mut InteractionSensor, &Parent)>,
    player_query: Query<&GlobalTransform, With<Player>>,
    interactive: Query<(&GlobalTransform, &Collider), With<Interactive>>
) {
    for (mut sensor, parent) in &mut interaction_sensors {
        // Bypass the player transform query if the is no interactive entities
        // in range
        if sensor.intersecting_entities.is_empty() {
            if sensor.closest_entity != None {
                sensor.closest_entity = None;
            }
            continue;
        }
        let player_transform = player_query.get(**parent).unwrap();
        // Find the closest entity.
        let mut closest_dist = f32::INFINITY;
        let mut closest_entity = None;
        for interactive_entity in &sensor.intersecting_entities {
            let (interactive_transform, interactive_collider) = interactive
                .get(*interactive_entity)
                .unwrap();

            let distance = interactive_collider.distance_to_local_point(
                player_transform.reparented_to(interactive_transform).translation.truncate(),
                false
            );
            if distance < closest_dist {
                closest_dist = distance;
                closest_entity = Some(*interactive_entity);
            }
        }
        // Gate the mutation such that only real change are detected when using
        // `Changed<InteractionSensor>` (See `Mut<>` mutation detection)
        if sensor.closest_entity != closest_entity {
            sensor.closest_entity = closest_entity;
        }
    }
}

fn test_interaction(
    players: Query<&Name, With<Player>>,
    sensors: Query<(&Parent, &InteractionSensor), Changed<InteractionSensor>>,
    interactives: Query<&Interactive>
) {
    for (parent, sensor) in sensors.iter() {
        info!(
            "{} interacting with {:?}",
            players.get(**parent).unwrap().as_str(),
            sensor.closest_entity.and_then(|e| interactives.get(e).ok()).map(|i| &i.name)
        );
    }
}

pub fn make_interaction_systems() -> SystemConfigs {
    (
        spawn_interaction_sensor,
        setup_interactive_entity,
        interaction_detection,
        update_interactions,
        test_interaction,
    ).chain()
}

use bevy::{
    asset::AssetServer,
    core::Name,
    ecs::{
        component::Component,
        entity::Entity,
        query::{ Changed, With },
        system::{ Commands, Query },
    },
    hierarchy::{ BuildChildren as _, DespawnRecursiveExt },
    math::Vec3,
    prelude::Res,
    render::primitives::Aabb,
    sprite::Anchor,
    text::{ Text, Text2dBundle, TextStyle },
    transform::components::Transform,
    utils::default,
};

/// Marker component for removing tooltips
#[derive(Component)]
pub(super) struct Tooltip;

use crate::components::interactions::{ InteractionSensor, Interactive };
pub(super) fn spawn_tooltip(
    mut commands: Commands,
    sensors: Query<&InteractionSensor, Changed<InteractionSensor>>,
    interactives: Query<&Aabb, With<Interactive>>,
    tooltips: Query<Entity, With<Tooltip>>,
    asset_server: Res<AssetServer>
) {
    if sensors.is_empty() {
        return; // Don't do anything if the InteractionSensor didn't change
    }
    // Remove old tooltips
    for tooltip in &tooltips {
        commands.entity(tooltip).despawn_recursive();
    }
    for sensor in sensors.iter() {
        let Some(entity) = sensor.closest_entity else {
            continue;
        };
        let Ok(aabb) = interactives.get(entity) else {
            continue;
        };
        let transform = Transform::from_translation(Vec3::Y * (aabb.half_extents.y + 5.0));
        let text = Text::from_section("E to talk", TextStyle {
            font: asset_server.load("fonts/bahnschrift.ttf"),
            font_size: 12.0,
            ..default()
        });
        commands.entity(entity).with_children(|builder| {
            builder.spawn((
                Tooltip,
                Text2dBundle {
                    text,
                    text_anchor: Anchor::BottomCenter,
                    transform,
                    ..Default::default()
                },
                #[cfg(debug_assertions)]
                Name::new("Tooltip")
            ));
        });
    }
}

use bevy::{
    asset::{ AssetServer, Assets, Handle },
    ecs::{ component::Component, system::Query },
    math::{ IVec2, Vec2 },
    render::texture::Image,
    sprite::TextureAtlasLayout,
    transform::components::Transform,
};
use bevy_ecs_ldtk::{
    ldtk::{ LayerInstance, TilesetDefinition },
    utils::ldtk_pixel_coords_to_translation_pivoted,
    EntityInstance,
    prelude::LdtkEntity,
    prelude::LdtkFields,
};
use bevy_rapier2d::dynamics::Velocity;

pub fn patrol(mut query: Query<(&mut Transform, &mut Velocity, &mut Patrol)>) {
    for (mut transform, mut velocity, mut patrol) in &mut query {
        if patrol.points.len() <= 1 {
            continue;
        }

        let mut new_velocity =
            (patrol.points[patrol.index] - transform.translation.truncate()).normalize() * 75.0;

        if new_velocity.dot(velocity.linvel) < 0.0 {
            if patrol.index == 0 {
                patrol.forward = true;
            } else if patrol.index == patrol.points.len() - 1 {
                patrol.forward = false;
            }

            transform.translation.x = patrol.points[patrol.index].x;
            transform.translation.y = patrol.points[patrol.index].y;

            if patrol.forward {
                patrol.index += 1;
            } else {
                patrol.index -= 1;
            }

            new_velocity =
                (patrol.points[patrol.index] - transform.translation.truncate()).normalize() * 75.0;
        }

        velocity.linvel = new_velocity;
    }
}

#[derive(Clone, PartialEq, Debug, Default, Component)]
pub struct Patrol {
    pub points: Vec<Vec2>,
    pub index: usize,
    pub forward: bool,
}

impl LdtkEntity for Patrol {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        layer_instance: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        _: &AssetServer,
        _: &mut Assets<TextureAtlasLayout>
    ) -> Patrol {
        let mut points = Vec::new();
        points.push(
            ldtk_pixel_coords_to_translation_pivoted(
                entity_instance.px,
                layer_instance.c_hei * layer_instance.grid_size,
                IVec2::new(entity_instance.width, entity_instance.height),
                entity_instance.pivot
            )
        );

        let ldtk_patrol_points = entity_instance
            .iter_points_field("patrol")
            .expect("patrol field should be correctly typed");

        for ldtk_point in ldtk_patrol_points {
            // The +1 is necessary here due to the pivot of the entities in the sample
            // file.
            // The patrols set up in the file look flat and grounded,
            // but technically they're not if you consider the pivot,
            // which is at the bottom-center for the skulls.
            let pixel_coords =
                (ldtk_point.as_vec2() + Vec2::new(0.5, 1.0)) *
                Vec2::splat(layer_instance.grid_size as f32);

            points.push(
                ldtk_pixel_coords_to_translation_pivoted(
                    pixel_coords.as_ivec2(),
                    layer_instance.c_hei * layer_instance.grid_size,
                    IVec2::new(entity_instance.width, entity_instance.height),
                    entity_instance.pivot
                )
            );
        }

        Patrol {
            points,
            index: 1,
            forward: true,
        }
    }
}
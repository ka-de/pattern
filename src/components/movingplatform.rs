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
pub fn moving_platform(mut query: Query<(&mut Transform, &mut Velocity, &mut MovingPlatform)>) {
    for (mut transform, mut velocity, mut movingplatform) in &mut query {
        if path.points.len() <= 1 {
            continue;
        }
        let mut new_velocity =
            (path.points[path.index] - transform.translation.truncate()).normalize() * 20.0;
        if new_velocity.dot(velocity.linvel) < 0.0 {
            if path.index == 0 {
                path.forward = true;
            } else if path.index == path.points.len() - 1 {
                path.forward = false;
            }
            transform.translation.x = path.points[path.index].x;
            transform.translation.y = path.points[path.index].y;
            if path.forward {
                path.index += 1;
            } else {
                path.index -= 1;
            }
            new_velocity =
                (path.points[path.index] - transform.translation.truncate()).normalize() * 20.0;
        }
        velocity.linvel = new_velocity;
    }
}
#[derive(Clone, PartialEq, Debug, Default, Component)]
pub struct MovingPlatform {
    pub points: Vec<Vec2>,
    pub index: usize,
    pub forward: bool,
}
impl LdtkEntity for MovingPlatform {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        layer_instance: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        _: &AssetServer,
        _: &mut Assets<TextureAtlasLayout>
    ) -> MovingPlatform {
        let mut points = Vec::new();
        points.push(
            ldtk_pixel_coords_to_translation_pivoted(
                entity_instance.px,
                layer_instance.c_hei * layer_instance.grid_size,
                IVec2::new(entity_instance.width, entity_instance.height),
                entity_instance.pivot
            )
        );
        let ldtk_path_points = entity_instance
            .iter_points_field("Path")
            .expect("Path field should be correctly typed");
        for ldtk_point in ldtk_path_points {
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
        MovingPlatform {
            points,
            index: 1,
            forward: true,
        }
    }
}

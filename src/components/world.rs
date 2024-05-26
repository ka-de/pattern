use bevy::math::bounding::Aabb2d;
use bevy::math::bounding::IntersectsVolume;
use bevy::prelude::*;

use crate::components::animals::FacingDirection;
use crate::components::{ GravityScale, Velocity };

const GRAVITY: f32 = 19.61;
const TILE_SIZE: f32 = 32.0;

// 💀 Zone
#[derive(Component)]
struct DeathZone {
    size: Vec2,
}

// Component representing a tile in the game
#[derive(Component)]
struct Tile {
    size: Vec2,
    ground: bool,
}

// Default implementation for Tile
impl Default for Tile {
    fn default() -> Self {
        Self {
            size: Vec2::new(TILE_SIZE, TILE_SIZE),
            ground: true,
        }
    }
}

// Function to create a bundle with the DeathZone component and a SpriteBundle
pub fn death_zone_bundle() -> impl Bundle {
    (
        DeathZone {
            size: Vec2::new(800.0, 50.0),
        },
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0), // red color for debug
                custom_size: Some(Vec2::new(800.0, 50.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, -200.0, 0.0)),
            ..default()
        },
    )
}

// Function to create a bundle with the Tile component and a SpriteBundle
pub fn tile_bundle(tile_position: Vec2) -> impl Bundle {
    (
        Tile::default(),
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 0.5),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            transform: Transform::from_translation(tile_position.extend(0.0)),
            ..default()
        },
    )
}

// System to handle collisions between entities and the death zone
fn handle_death_zone_collisions(
    mut commands: Commands,
    death_zone_query: Query<(&DeathZone, &Transform)>,
    entity_query: Query<(Entity, &Transform, &Sprite, &Velocity, &Name)>
) {
    for (death_zone, death_zone_transform) in death_zone_query.iter() {
        let death_zone_aabb = Aabb2d {
            min: death_zone_transform.translation.truncate() - death_zone.size / 2.0,
            max: death_zone_transform.translation.truncate() + death_zone.size / 2.0,
        };

        for (entity, transform, sprite, _, name) in entity_query.iter() {
            let entity_size = sprite.custom_size.unwrap_or(Vec2::splat(1.0));
            let entity_aabb = Aabb2d {
                min: transform.translation.truncate() - entity_size / 2.0,
                max: transform.translation.truncate() + entity_size / 2.0,
            };

            if death_zone_aabb.intersects(&entity_aabb) {
                info!("Collision detected between entity {} and death zone", name);
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

// System to apply gravity to all entities with a Velocity and GravityScale component
fn apply_gravity(mut query: Query<(&mut Velocity, &GravityScale)>, time: Res<Time>) {
    for (mut velocity, gravity_scale) in &mut query {
        velocity.y -= GRAVITY * gravity_scale.0 * time.delta_seconds();
    }
}

// System to handle collisions between "animal" entities and tiles
fn handle_collisions(
    mut animal_query: Query<
        (&mut Velocity, &mut Transform, &Sprite, &GravityScale, &mut FacingDirection, &Name)
    >,
    tile_query: Query<(&Tile, &Transform), Without<Velocity>>,
    time: Res<Time>
) {
    for (
        mut animal_velocity,
        mut animal_transform,
        animal_sprite,
        gravity_scale,
        facing_direction,
        name,
    ) in &mut animal_query {
        let animal_size = animal_sprite.custom_size.unwrap_or(Vec2::splat(1.0));
        let mut max_penetration_depth: f32 = 0.0;

        for (tile, tile_transform) in tile_query.iter() {
            let tile_position = tile_transform.translation.truncate();
            let tile_size = tile.size;

            if
                tile.ground &&
                is_colliding(
                    animal_transform.translation.truncate(),
                    animal_size,
                    tile_position,
                    tile_size
                )
            {
                let tile_top = tile_position.y + tile_size.y;
                let animal_bottom = animal_transform.translation.y - animal_size.y / 2.0;
                let penetration_depth = animal_bottom - tile_top;

                if penetration_depth < 0.0 {
                    max_penetration_depth = max_penetration_depth.max(-penetration_depth);
                }
            }
        }

        if max_penetration_depth > 0.0 {
            animal_velocity.y = 0.0;
            animal_transform.translation.y += max_penetration_depth * gravity_scale.0;
        } else {
            animal_velocity.y -= GRAVITY * gravity_scale.0 * time.delta_seconds();
        }
    }
}

/**
 * This function uses the Axis-Aligned Bounding Box (AABB) collision detection method,
 * which is a common and simple method used to check if two rectangles are colliding.
 * It takes four parameters: the positions and sizes of two rectangles (a and b).
 */
fn is_colliding(a_pos: Vec2, a_size: Vec2, b_pos: Vec2, b_size: Vec2) -> bool {
    let a_aabb = Aabb2d::new(a_pos, a_size);
    let b_aabb = Aabb2d::new(b_pos, b_size);
    a_aabb.intersects(&b_aabb)
}

// Function to add the gravity, collision, and death zone collision systems to the Bevy app
pub fn setup_world_systems(app: &mut App) -> &mut App {
    app.add_systems(Update, apply_gravity)
        .add_systems(Update, handle_collisions)
        .add_systems(Update, handle_death_zone_collisions)
}

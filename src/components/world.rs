use bevy::prelude::*;

use crate::components::{GravityScale, Velocity};

#[derive(Component)]
struct DeathZone {
    size: Vec2,
}

pub fn death_zone_bundle() -> impl Bundle {
    (
        DeathZone {
            size: Vec2::new(800.0, 50.0), // adjust as needed
        },
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0),          // red color for debug
                custom_size: Some(Vec2::new(800.0, 50.0)), // adjust as needed
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, -200.0, 0.0)), // adjust as needed
            ..default()
        },
    )
}

#[derive(Component)]
struct Tile {
    size: Vec2,
    ground: bool,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            size: Vec2::new(32.0, 16.0),
            ground: true,
        }
    }
}

pub fn tile_bundle(tile_position: Vec2) -> impl Bundle {
    (
        Tile::default(),
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 0.5),
                custom_size: Some(Vec2::new(32.0, 16.0)),
                ..default()
            },
            transform: Transform::from_translation(tile_position.extend(0.0)),
            ..default()
        },
    )
}

fn handle_death_zone_collisions(
    mut commands: Commands,
    death_zone_query: Query<(&DeathZone, &Transform)>,
    entity_query: Query<(Entity, &Transform, &Sprite, &Velocity, &Name)>,
) {
    for (death_zone, death_zone_transform) in death_zone_query.iter() {
        let death_zone_position = death_zone_transform.translation.truncate();
        let death_zone_size = death_zone.size;
        //println!("Death zone position: {:?}, size: {:?}", death_zone_position, death_zone_size);

        for (entity, transform, sprite, _, name) in entity_query.iter() {
            let entity_position = transform.translation.truncate();
            let entity_size = sprite.custom_size.unwrap_or(Vec2::splat(1.0));
            //println!("Entity {} position: {:?}, size: {:?}", name, entity_position, entity_size);

            if is_colliding(
                entity_position,
                entity_size,
                death_zone_position,
                death_zone_size,
            ) {
                println!("Collision detected between entity {} and death zone", name);
                commands.entity(entity).despawn();
            }
        }
    }
}

fn apply_gravity(mut query: Query<(&mut Velocity, &GravityScale)>, time: Res<Time>) {
    const GRAVITY: f32 = 19.61;

    for (mut velocity, gravity_scale) in query.iter_mut() {
        velocity.y -= GRAVITY * gravity_scale.0 * time.delta_seconds();
    }
}

fn handle_collisions(
    mut animal_query: Query<(&mut Velocity, &mut Transform, &Sprite)>,
    tile_query: Query<(&Tile, &Transform), Without<Velocity>>,
) {
    for (mut animal_velocity, mut animal_transform, animal_sprite) in animal_query.iter_mut() {
        let animal_size = animal_sprite.custom_size.unwrap_or(Vec2::splat(1.0));

        for (tile, tile_transform) in tile_query.iter() {
            let tile_position = tile_transform.translation.truncate();
            let tile_size = tile.size;

            // Check for collision between the animal and the tile
            if tile.ground
                && is_colliding(
                    animal_transform.translation.truncate(),
                    animal_size,
                    tile_position,
                    tile_size,
                )
            {
                // Resolve the collision for ground tiles
                // For example, set the animal's vertical velocity to 0 when landing on a ground tile
                animal_velocity.y = 0.0;
                animal_transform.translation.y =
                    tile_position.y + tile_size.y / 2.0 + animal_size.y / 2.0;
            }
        }
    }
}

fn is_colliding(a_pos: Vec2, a_size: Vec2, b_pos: Vec2, b_size: Vec2) -> bool {
    let a_min = a_pos - a_size / 2.0;
    let a_max = a_pos + a_size / 2.0;
    let b_min = b_pos - b_size / 2.0;
    let b_max = b_pos + b_size / 2.0;

    //println!("a_min: {:?}, a_max: {:?}, b_min: {:?}, b_max: {:?}", a_min, a_max, b_min, b_max);

    a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y
}

pub fn setup_world_systems(app: &mut App) -> &mut App {
    app.add_systems(Update, apply_gravity)
        .add_systems(Update, handle_collisions)
        .add_systems(Update, handle_death_zone_collisions)
}

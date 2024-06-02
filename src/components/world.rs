use bevy::prelude::*;

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

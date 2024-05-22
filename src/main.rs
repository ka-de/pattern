mod components;

use components::{Dog, Cat, CustomPerfUiAppExt as _, CustomResourcesAppExt as _, CustomSystemsAppExt as _};

use bevy::prelude::*;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((Camera2dBundle::default(), components::MainCamera));

    // The 💀 zone.
    commands.spawn((
        components::DeathZone {
            size: Vec2::new(800.0, 50.0), // adjust as needed
            border_width: 5.0,
        },
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0),          // red color for debug
                custom_size: Some(Vec2::new(800.0, 50.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, -200.0, 0.0)), // adjust as needed
            ..default()
        },
    ));

    // Tiles
    for x in -5..5 {
        let tile_position = Vec2::new(x as f32 * 32.0, -100.0);
        commands.spawn((
            components::Tile::default(),
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.5, 0.5, 0.5),
                    custom_size: Some(Vec2::new(32.0, 16.0)),
                    ..default()
                },
                transform: Transform::from_translation(tile_position.extend(0.0)),
                ..default()
            },
        ));
    }

    // 🐈‍⬛
    components::spawn_cat(&mut commands, &asset_server, texture_atlas_layouts.as_mut());

    // 🐕
    components::spawn_dog(&mut commands, &asset_server, texture_atlas_layouts.as_mut())
}

fn main() {
    App::new()
        // The ImagePlugin::default_nearest() prevents blurry sprites
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_custom_perf_ui()
        .add_animal_perf_ui::<Cat>()
        .add_animal_perf_ui::<Dog>()
        .add_custom_resources()
        .add_systems(Startup, setup)
        .add_custom_systems()
        .run();
}

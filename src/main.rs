// Turn clippy into a real bitch
#![warn(clippy::all, clippy::pedantic)]

mod components;

use components::{ CustomPerfUiAppExt as _, CustomSystemsAppExt as _ };

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

/*
 * Player
 */

#[derive(Default, Component)]
struct PlayerChild;

#[derive(Default, Component)]
struct Player;

#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    player: Player,
    #[sprite_bundle]
    sprite_bundle: SpriteBundle,
}

fn process_player(
    mut commands: Commands,
    new_players: Query<Entity, Added<Player>>,
    assets: Res<AssetServer>
) {
    for player_entity in new_players.iter() {
        commands.spawn(PlayerChild).set_parent(player_entity);
    }
}

/*
 * GameState
 */

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum GameState {
    #[default]
    Loading,
    MainMenu,
    Playing,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    commands.spawn((Camera2dBundle::default(), components::MainCamera));

    // The 💀 zone.
    commands.spawn(components::death_zone_bundle());

    // Tiles
    for x in -5..5 {
        let tile_position = Vec2::new((x as f32) * 32.0, -100.0);
        commands.spawn(components::tile_bundle(tile_position));
    }

    // 🐈‍⬛
    components::spawn_cat(&mut commands, &asset_server, texture_atlas_layouts.as_mut());

    // 🐕
    components::spawn_dog(&mut commands, &asset_server, texture_atlas_layouts.as_mut());
    components::spawn_dog(&mut commands, &asset_server, texture_atlas_layouts.as_mut());
}

fn main() {
    // this code is compiled only if debug assertions are enabled (debug mode)
    #[cfg(debug_assertions)]
    let log_plugin = LogPlugin {
        level: bevy::log::Level::DEBUG,
        filter: "info,pattern=debug,wgpu_core=warn,wgpu_hal=warn,pattern=debug".into(),
        update_subscriber: None,
    };

    // this code is compiled only if debug assertions are disabled (release mode)
    #[cfg(not(debug_assertions))]
    let log_plugin = LogPlugin {
        level: bevy::log::Level::INFO,
        filter: "warning,pattern=info".into(),
        update_subscriber: None,
    };

    #[cfg(target_arch = "wasm32")]
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            canvas: Some("#pattern-canvas".into()),
            ..default()
        }),
        ..default()
    };

    #[cfg(not(target_arch = "wasm32"))]
    let window_plugin = WindowPlugin::default();

    // The ImagePlugin::default_nearest() prevents blurry sprites
    App::new()
        .add_plugins(
            DefaultPlugins.set(window_plugin).set(ImagePlugin::default_nearest()).set(log_plugin)
        )
        .register_ldtk_entity::<PlayerBundle>("Player")
        .add_systems(Update, process_player)
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_custom_perf_ui()
        .add_systems(Startup, setup)
        .add_custom_systems()
        .run();
}

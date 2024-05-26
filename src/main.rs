// Turn clippy into a real bitch
#![warn(clippy::all, clippy::pedantic)]

mod components;

use components::{ CustomPerfUiAppExt as _, CustomSystemsAppExt as _ };

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

/*
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
*/

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

    App::new()
        .add_plugins(
            DefaultPlugins.set(window_plugin).set(ImagePlugin::default_nearest()).set(log_plugin)
        )
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_custom_perf_ui()
        // Start of LDTK
        .add_plugins((LdtkPlugin, RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0)))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, -2000.0),
            physics_pipeline_active: true,
            query_pipeline_active: true,
            timestep_mode: TimestepMode::Variable {
                max_dt: 1.0 / 60.0,
                time_scale: 1.0,
                substeps: 1,
            },
            scaled_shape_subdivision: 10,
            force_update_from_transform_changes: false,
        })
        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .add_systems(Startup, components::systems::setup)
        .add_systems(Update, components::systems::spawn_wall_collision)
        .add_systems(Update, components::systems::movement)
        .add_systems(Update, components::systems::detect_climb_range)
        .add_systems(Update, components::systems::ignore_gravity_if_climbing)
        .add_systems(Update, components::systems::patrol)
        .add_systems(Update, components::systems::camera_fit_inside_current_level)
        .add_systems(Update, components::systems::update_level_selection)
        .add_systems(Update, components::systems::dbg_player_items)
        .add_systems(Update, components::systems::spawn_ground_sensor)
        .add_systems(Update, components::systems::ground_detection)
        .add_systems(Update, components::systems::update_on_ground)
        .add_systems(Update, components::systems::restart_level)
        .register_ldtk_int_cell::<components::misc::WallBundle>(1)
        .register_ldtk_int_cell::<components::misc::LadderBundle>(2)
        .register_ldtk_int_cell::<components::misc::WallBundle>(3)
        .register_ldtk_entity::<components::misc::PlayerBundle>("Player")
        .register_ldtk_entity::<components::misc::MobBundle>("Mob")
        .register_ldtk_entity::<components::misc::ChestBundle>("Chest")
        .register_ldtk_entity::<components::misc::PumpkinsBundle>("Pumpkins")
        // End of LDTK
        .add_custom_systems()
        .run();
}

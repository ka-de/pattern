use bevy::{
    asset::{ Assets, Handle },
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        entity::Entity,
        query::{ With, Without },
        schedule::SystemConfigs,
        system::{ Commands, Query, Res, ResMut, Resource },
    },
    input::{ keyboard::KeyCode, ButtonInput },
    log,
    math::{ Rect, Vec2 },
    prelude::IntoSystemConfigs,
    transform::components::Transform,
};
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_ecs_ldtk::{
    assets::{ LdtkProject, LevelIndices, LevelMetadataAccessor },
    LdtkSettings,
    LdtkWorldBundle,
    LevelIid,
    LevelSelection,
    LevelSpawnBehavior,
    Respawn,
    SetClearColor,
};
use bevy_rapier2d::plugin::{ RapierConfiguration, TimestepMode };

use crate::entities::Player;

pub(crate) fn level_selection_systems() -> SystemConfigs {
    (update_level_selection, restart_level, respawn_world).into_configs()
}

#[derive(AssetCollection, Resource)]
pub(crate) struct LdtkAssets {
    #[asset(path = "biomes.ldtk")]
    first_level: Handle<LdtkProject>,
}

// Loads the first level of the game from an LDTK file and spawns the game world.
// It also sets up the physics configuration and the level selection resource.
pub(crate) fn spawn_ldtk_world(mut commands: Commands, ldtk_assets: Res<LdtkAssets>) {
    commands.insert_resource(RapierConfiguration {
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
    });
    commands.insert_resource(LevelSelection::Uid(0));
    commands.insert_resource(LdtkSettings {
        level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
            load_level_neighbors: true,
        },
        set_clear_color: SetClearColor::FromLevelBackground,
        ..Default::default()
    });

    // 🎥
    commands.spawn(Camera2dBundle::default());
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: ldtk_assets.first_level.clone(),
        ..Default::default()
    });
    log::info!("Spawned ldtk world");
}

// Updates the current level selection based on the player’s position.
// If the player is within the bounds of a level, that level is set as the current level.
pub(crate) fn update_level_selection(
    level_query: Query<(&LevelIid, &Transform), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    mut level_selection: ResMut<LevelSelection>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>
) {
    for (level_iid, level_transform) in &level_query {
        let ldtk_project = ldtk_project_assets
            .get(ldtk_projects.single())
            .expect("Project should be loaded if level has spawned");

        let level = ldtk_project
            .get_raw_level_by_iid(&level_iid.to_string())
            .expect("Spawned level should exist in LDtk project");

        let level_bounds = Rect {
            min: Vec2::new(level_transform.translation.x, level_transform.translation.y),
            max: Vec2::new(
                level_transform.translation.x + (level.px_wid as f32),
                level_transform.translation.y + (level.px_hei as f32)
            ),
        };

        for player_transform in &player_query {
            if
                player_transform.translation.x < level_bounds.max.x &&
                player_transform.translation.x > level_bounds.min.x &&
                player_transform.translation.y < level_bounds.max.y &&
                player_transform.translation.y > level_bounds.min.y &&
                !level_selection.is_match(&LevelIndices::default(), level)
            {
                log::debug!("Updating level selection {:?} -> {:?}", level_selection, level.iid);
                *level_selection = LevelSelection::iid(level.iid.clone());
            }
        }
    }
}

// Respawns the game world when the ‘T’ key is pressed.
// It does this by inserting a Respawn component into the entity that holds the LDtk project.
pub(crate) fn respawn_world(
    mut commands: Commands,
    ldtk_projects: Query<Entity, With<Handle<LdtkProject>>>,
    input: Res<ButtonInput<KeyCode>>
) {
    if input.just_pressed(KeyCode::KeyT) {
        commands.entity(ldtk_projects.single()).insert(Respawn);
    }
}

// This function restarts the current level when the ‘R’ key is pressed.
// It does this by inserting a Respawn component into all entities that are part of the current level.
pub(crate) fn restart_level(
    mut commands: Commands,
    level_query: Query<Entity, With<LevelIid>>,
    input: Res<ButtonInput<KeyCode>>
) {
    if input.just_pressed(KeyCode::KeyR) {
        for level_entity in &level_query {
            commands.entity(level_entity).insert(Respawn);
        }
    }
}

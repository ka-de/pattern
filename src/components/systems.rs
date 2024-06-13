use bevy::{
    app::{ App, Update },
    asset::{ Assets, Handle },
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        entity::Entity,
        event::EventReader,
        query::{ Changed, With, Without },
        schedule::{ common_conditions::in_state, OnEnter },
        system::{ Commands, Query, Res, ResMut, Resource },
    },
    input::{ keyboard::KeyCode, ButtonInput },
    math::{ Rect, Vec2 },
    prelude::IntoSystemConfigs,
    transform::components::Transform,
};
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{ config::{ ConfigureLoadingState, LoadingStateConfig }, LoadingStateAppExt },
};
use bevy_ecs_ldtk::{
    app::{ LdtkEntityAppExt, LdtkIntCellAppExt },
    assets::{ LdtkProject, LevelIndices, LevelMetadataAccessor },
    LdtkPlugin,
    LdtkSettings,
    LdtkWorldBundle,
    LevelIid,
    LevelSelection,
    LevelSpawnBehavior,
    Respawn,
    SetClearColor,
};
use bevy_rapier2d::{
    dynamics::GravityScale,
    pipeline::CollisionEvent,
    plugin::{ NoUserData, RapierConfiguration, RapierPhysicsPlugin, TimestepMode },
};

use super::{
    ladders::{ Climbable, Climber },
    water::Swimmer,
    predefinedpath::move_on_path,
    player::Player,
    wall::spawn_wall_collision,
    water::Swimmable,
};
use crate::plugins::{ gamestate::GameState, rapier_utils::reciprocal_collisions };

// Sets up the game world using the LDTK plugin.
//
// It registers different entities and their corresponding bundles to be used in the game world.
// It also adds various systems to the game update loop, which will be run if the game state is Playing.
pub fn setup_ldtk(app: &mut App) {
    app.register_ldtk_int_cell::<super::wall::WallBundle>(1)
        .register_ldtk_int_cell::<super::ladders::LadderBundle>(2)
        .register_ldtk_int_cell::<super::wall::WallBundle>(3)
        .register_ldtk_int_cell::<super::water::WaterBundle>(4)
        .register_ldtk_entity::<super::torch::TorchBundle>("Torch")
        .register_ldtk_entity::<super::player::PlayerBundle>("Player")
        .register_ldtk_entity::<super::dog::DogBundle>("Dog")
        .register_ldtk_entity::<super::dog::DogPatrolBundle>("DogPatrol")
        .register_ldtk_entity::<super::cat::CatBundle>("Cat")
        .register_ldtk_entity::<super::cat::CatPatrolBundle>("CatPatrol")
        .register_ldtk_entity::<super::cauldron::CauldronBundle>("Cauldron")
        .register_ldtk_entity::<super::kade::KadeBundle>("Kade")
        .register_ldtk_entity::<super::npc::NpcBundle>("Npc")
        .register_ldtk_entity::<super::npc::NpcPatrolBundle>("NpcPatrol")
        .register_ldtk_entity::<super::movingplatform::MovingPlatformBundle>("MovingPlatform")
        .register_ldtk_entity::<super::enemy::EnemyBundle>("Enemy")
        .register_ldtk_entity::<super::chest::ChestBundle>("Chest")
        .register_ldtk_entity::<super::pumpkin::PumpkinBundle>("Pumpkins")
        .configure_loading_state(
            LoadingStateConfig::new(GameState::SplashScreen).load_collection::<LdtkAssets>()
        )
        .add_systems(OnEnter(GameState::Playing), spawn_ldtk_world)
        .add_systems(
            Update,
            (
                spawn_wall_collision,
                detect_climb_range,
                detect_swim_range,
                ignore_gravity_if_climbing,
                move_on_path,
                super::camera::fit_inside_current_level,
                update_level_selection,
                super::items::dbg_player_items,
                super::ground::spawn_ground_sensor,
                super::ground::ground_detection,
                super::ground::update_on_ground,
                super::interactions::make_interaction_systems(),
                restart_level,
                respawn_world,
            ).run_if(in_state(GameState::Playing))
        )
        // RapierPhysicsPlugin
        .add_plugins((LdtkPlugin, RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0)));
}

#[derive(AssetCollection, Resource)]
struct LdtkAssets {
    #[asset(path = "first_level.ldtk")]
    first_level: Handle<LdtkProject>,
}

// Loads the first level of the game from an LDTK file and spawns the game world.
// It also sets up the physics configuration and the level selection resource.
fn spawn_ldtk_world(mut commands: Commands, ldtk_assets: Res<LdtkAssets>) {
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
}

// Checks for collision events between climbers and climbable entities.
// If a collision starts, the climbable entity is added to the climber’s set of intersecting climbables.
// If a collision stops, the climbable entity is removed from the set.
pub fn detect_climb_range(
    mut climbers: Query<&mut Climber>,
    climbables: Query<Entity, With<Climbable>>,
    mut collisions: EventReader<CollisionEvent>
) {
    reciprocal_collisions(&mut collisions, move |collider_a, collider_b, _, start| {
        if
            let (Ok(mut climber), Ok(climbable)) = (
                climbers.get_mut(*collider_a),
                climbables.get(*collider_b),
            )
        {
            if start {
                climber.intersecting_climbables.insert(climbable);
            } else {
                climber.intersecting_climbables.remove(&climbable);
            }
            true
        } else {
            false
        }
    });
}

pub fn detect_swim_range(
    mut swimmers: Query<&mut Swimmer>,
    swimmables: Query<Entity, With<Swimmable>>,
    mut collisions: EventReader<CollisionEvent>
) {
    reciprocal_collisions(&mut collisions, move |collider_a, collider_b, _, start| {
        if
            let (Ok(mut swimmer), Ok(swimmable)) = (
                swimmers.get_mut(*collider_a),
                swimmables.get(*collider_b),
            )
        {
            if start {
                swimmer.intersecting_swimmables.insert(swimmable);
            } else {
                swimmer.intersecting_swimmables.remove(&swimmable);
            }
            true
        } else {
            false
        }
    });
}

// Checks if a climber entity is climbing.
// If it is, the gravity scale is set to 0.0, effectively ignoring gravity.
// If the climber is not climbing, the gravity scale is set back to 1.0.
pub fn ignore_gravity_if_climbing(
    mut query: Query<(&Climber, &mut GravityScale), Changed<Climber>>
) {
    for (climber, mut gravity_scale) in &mut query {
        if climber.climbing {
            gravity_scale.0 = 0.0;
        } else {
            gravity_scale.0 = 1.0;
        }
    }
}

// Updates the current level selection based on the player’s position.
// If the player is within the bounds of a level, that level is set as the current level.
pub fn update_level_selection(
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
                *level_selection = LevelSelection::iid(level.iid.clone());
            }
        }
    }
}

// Respawns the game world when the ‘T’ key is pressed.
// It does this by inserting a Respawn component into the entity that holds the LDtk project.
fn respawn_world(
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
pub fn restart_level(
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

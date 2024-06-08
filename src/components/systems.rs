use super::{
    items::Items,
    ladders::{ Climbable, Climber },
    patrol::patrol,
    player::Player,
    wall::spawn_wall_collision,
};
use crate::plugins::gamestate::GameState;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn setup_ldtk(app: &mut App) {
    app.register_ldtk_int_cell::<super::wall::WallBundle>(1)
        .register_ldtk_int_cell::<super::ladders::LadderBundle>(2)
        .register_ldtk_int_cell::<super::wall::WallBundle>(3)
        .register_ldtk_entity::<super::torch::TorchBundle>("Torch")
        .register_ldtk_entity::<super::player::PlayerBundle>("Player")
        .register_ldtk_entity::<super::npc::NpcBundle>("Npc")
        .register_ldtk_entity::<super::npc::NpcPatrolBundle>("NpcPatrol")
        .register_ldtk_entity::<super::enemy::MobBundle>("Mob")
        .register_ldtk_entity::<super::chest::ChestBundle>("Chest")
        .register_ldtk_entity::<super::pumpkins::PumpkinsBundle>("Pumpkins")
        .add_systems(
            Update,
            (
                spawn_wall_collision,
                detect_climb_range,
                ignore_gravity_if_climbing,
                patrol,
                super::camera::fit_inside_current_level,
                update_level_selection,
                super::items::dbg_player_items,
                super::ground::spawn_ground_sensor,
                super::ground::ground_detection,
                super::ground::update_on_ground,
                restart_level,
                respawn_world,
                super::npc::print_npc_info,
                super::npc::print_npcpatrol_info,
            ).run_if(in_state(GameState::Playing))
        )
        // RapierPhysicsPlugin
        .add_plugins((LdtkPlugin, RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0)));
}

pub fn spawn_ldtk_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    // TODO: use bevy_asset_loader states
    let ldtk_handle = asset_server.load("first_level.ldtk");
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
    commands.spawn(LdtkWorldBundle {
        ldtk_handle,
        ..Default::default()
    });
}

pub fn detect_climb_range(
    mut climbers: Query<&mut Climber>,
    climbables: Query<Entity, With<Climbable>>,
    mut collisions: EventReader<CollisionEvent>
) {
    for collision in collisions.read() {
        match collision {
            CollisionEvent::Started(collider_a, collider_b, _) => {
                if
                    let (Ok(mut climber), Ok(climbable)) = (
                        climbers.get_mut(*collider_a),
                        climbables.get(*collider_b),
                    )
                {
                    climber.intersecting_climbables.insert(climbable);
                }
                if
                    let (Ok(mut climber), Ok(climbable)) = (
                        climbers.get_mut(*collider_b),
                        climbables.get(*collider_a),
                    )
                {
                    climber.intersecting_climbables.insert(climbable);
                };
            }
            CollisionEvent::Stopped(collider_a, collider_b, _) => {
                if
                    let (Ok(mut climber), Ok(climbable)) = (
                        climbers.get_mut(*collider_a),
                        climbables.get(*collider_b),
                    )
                {
                    climber.intersecting_climbables.remove(&climbable);
                }

                if
                    let (Ok(mut climber), Ok(climbable)) = (
                        climbers.get_mut(*collider_b),
                        climbables.get(*collider_a),
                    )
                {
                    climber.intersecting_climbables.remove(&climbable);
                }
            }
        }
    }
}

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

fn respawn_world(
    mut commands: Commands,
    ldtk_projects: Query<Entity, With<Handle<LdtkProject>>>,
    input: Res<ButtonInput<KeyCode>>
) {
    if input.just_pressed(KeyCode::KeyT) {
        commands.entity(ldtk_projects.single()).insert(Respawn);
    }
}

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

use bevy::{
    app::{ App, PreUpdate, Update },
    ecs::schedule::{ common_conditions::in_state, OnEnter },
    prelude::IntoSystemConfigs,
};
use bevy_asset_loader::loading_state::{
    config::{ ConfigureLoadingState, LoadingStateConfig },
    LoadingStateAppExt,
};
use bevy_ecs_ldtk::LdtkPlugin;
use bevy_rapier2d::plugin::{ NoUserData, RapierPhysicsPlugin };

use crate::{ components, entities, plugins::{ gamestate::GameState, ldtk } };

/// Sets up the game world using the LDTK plugin.
///
/// It registers different entities and their corresponding bundles to be used in the game world.
/// It also adds various systems to the game update loop, which will be run if the game state is Playing.
pub fn setup_world_systems(app: &mut App) {
    // ⚠️ FIXME: some parts should be moved to the ldtk plugin once we figure the execution order of the systems.
    app.add_plugins((
        crate::entities::plugin,
        LdtkPlugin,
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
    ))
        .configure_loading_state(
            LoadingStateConfig::new(GameState::SplashScreen).load_collection::<ldtk::LdtkAssets>()
        )
        .add_systems(
            PreUpdate,
            ldtk::level_selection_systems().run_if(in_state(GameState::Playing))
        )
        .add_systems(OnEnter(GameState::Playing), crate::plugins::ldtk::spawn_ldtk_world)
        .add_systems(
            Update,
            (
                ldtk::update_level_selection,
                ldtk::restart_level,
                ldtk::respawn_world,
                components::collision::spawn_wall_collision,
                (
                    components::interactions::spawn_interaction_sensor,
                    components::interactions::setup_interactive_entity,
                    components::interactions::interaction_detection,
                    components::interactions::update_interactions,
                ).chain(),
                (
                    components::ground::spawn_ground_sensor,
                    components::ground::ground_detection,
                    components::ground::update_on_ground,
                ).chain(),
                (
                    components::climbing::detect_climb_range,
                    components::climbing::ignore_gravity_if_climbing,
                ).chain(),
                components::swimming::detect_swim_range,
                components::predefinedpath::move_on_path,
                components::camera::fit_inside_current_level,
                components::items::dbg_player_items,
                components::line_of_sight::line_of_sight::<entities::Player>,
            ).run_if(in_state(GameState::Playing))
        );
    // RapierPhysicsPlugin
}

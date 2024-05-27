use bevy::{
    app::{ App, Startup, Update },
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{ schedule::States, system::Commands },
    log::info,
    math::Vec2,
    reflect::Reflect,
};
use bevy_ecs_ldtk::{
    app::{ LdtkEntityAppExt, LdtkIntCellAppExt },
    LdtkPlugin,
    LdtkSettings,
    LevelSelection,
    LevelSpawnBehavior,
    SetClearColor,
};
use bevy_rapier2d::plugin::{ NoUserData, RapierConfiguration, RapierPhysicsPlugin, TimestepMode };

/*
 * The GameState
 *
 *
 */

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum GameState {
    #[default]
    SplashScreen,
    Loading,
    MainMenu,
    Playing,
}

pub fn set_state_splashscreen(mut commands: Commands) {
    info!("Set GameState: SplashScreen");
    let camera = Camera2dBundle::default();
    commands.spawn(camera);
    info!("Camera spawned during GameState SplashScreen");
}

pub fn set_state_loading() {
    info!("Set GameState: Loading");
}

pub fn set_state_mainmenu() {
    info!("Set GameState: MainMenu");
}

pub fn set_state_playing() {
    info!("Set GameState: Playing");
    let mut app = App::new();

    app.add_plugins((LdtkPlugin, RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0)))
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
        .add_systems(Startup, super::systems::setup)
        .register_ldtk_int_cell::<super::misc::WallBundle>(1)
        .register_ldtk_int_cell::<super::misc::LadderBundle>(2)
        .register_ldtk_int_cell::<super::misc::WallBundle>(3)
        .register_ldtk_entity::<super::player::PlayerBundle>("Player")
        .register_ldtk_entity::<super::misc::MobBundle>("Mob")
        .register_ldtk_entity::<super::misc::ChestBundle>("Chest")
        .register_ldtk_entity::<super::misc::PumpkinsBundle>("Pumpkins")
        .add_systems(Update, super::systems::spawn_wall_collision)
        .add_systems(Update, super::systems::movement)
        .add_systems(Update, super::systems::detect_climb_range)
        .add_systems(Update, super::systems::ignore_gravity_if_climbing)
        .add_systems(Update, super::systems::patrol)
        .add_systems(Update, super::camera::fit_inside_current_level)
        .add_systems(Update, super::systems::update_level_selection)
        .add_systems(Update, super::systems::dbg_player_items)
        .add_systems(Update, super::ground::spawn_ground_sensor)
        .add_systems(Update, super::ground::ground_detection)
        .add_systems(Update, super::ground::update_on_ground)
        .add_systems(Update, super::systems::restart_level);
}

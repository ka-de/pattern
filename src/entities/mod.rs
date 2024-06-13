pub(crate) mod player;
pub(crate) mod torch;
pub(crate) mod enemy;
pub(crate) mod npc;
pub(crate) mod pumpkin;
pub(crate) mod chest;
pub(crate) mod movingplatform;
pub(crate) mod dog;
pub(crate) mod cat;
pub(crate) mod kade;
pub(crate) mod cauldron;
pub(crate) mod intcells;

use bevy::app::App;
use bevy_ecs_ldtk::app::{ LdtkEntityAppExt, LdtkIntCellAppExt };

// Component commonly used by entities bundles in sub-modules
use crate::components::{ collision::ColliderBundle, predefinedpath::PredefinedPath };
pub(super) use player::Player;

pub(super) fn plugin(app: &mut App) {
    app.register_ldtk_int_cell::<intcells::WallBundle>(1)
        .register_ldtk_int_cell::<intcells::LadderBundle>(2)
        .register_ldtk_int_cell::<intcells::WallBundle>(3)
        .register_ldtk_int_cell::<intcells::WaterBundle>(4)
        .register_ldtk_entity::<torch::TorchBundle>("Torch")
        .register_ldtk_entity::<player::PlayerBundle>("Player")
        .register_ldtk_entity::<dog::DogBundle>("Dog")
        .register_ldtk_entity::<dog::DogPatrolBundle>("DogPatrol")
        .register_ldtk_entity::<cat::CatBundle>("Cat")
        .register_ldtk_entity::<cat::CatPatrolBundle>("CatPatrol")
        .register_ldtk_entity::<cauldron::CauldronBundle>("Cauldron")
        .register_ldtk_entity::<kade::KadeBundle>("Kade")
        .register_ldtk_entity::<npc::NpcBundle>("Npc")
        .register_ldtk_entity::<npc::NpcPatrolBundle>("NpcPatrol")
        .register_ldtk_entity::<movingplatform::MovingPlatformBundle>("MovingPlatform")
        .register_ldtk_entity::<enemy::EnemyBundle>("Enemy")
        .register_ldtk_entity::<chest::ChestBundle>("Chest")
        .register_ldtk_entity::<pumpkin::PumpkinBundle>("Pumpkins");
}

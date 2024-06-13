use bevy::ecs::{ bundle::Bundle, component::Component };
use bevy_ecs_ldtk::LdtkIntCell;

use crate::components::{ sensorbundle::SensorBundle, swimming::Swimmable, climbing::Climbable };

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

/// LdtkIntCell representing a ladder in the game world.
#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct LadderBundle {
    #[from_int_grid_cell]
    pub sensor_bundle: SensorBundle,
    pub climbable: Climbable,
}

/// LdtkIntCell representing a water in the game world.
#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct WaterBundle {
    #[from_int_grid_cell]
    pub sensor_bundle: SensorBundle,
    pub swimmable: Swimmable,
}

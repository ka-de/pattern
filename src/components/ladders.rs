use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkIntCell;
use std::collections::HashSet;

use super::sensorbundle::SensorBundle;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Climbable;

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct LadderBundle {
    #[from_int_grid_cell]
    pub sensor_bundle: SensorBundle,
    pub climbable: Climbable,
}

// Attach this component to any entity to allow them to climb up ladders
#[derive(Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Climber {
    pub climbing: bool,
    pub intersecting_climbables: HashSet<Entity>,
}

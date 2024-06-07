use bevy::{
    ecs::{ bundle::Bundle, component::Component },
    prelude::{ Or, Query, With },
    sprite::SpriteBundle,
    transform::components::Transform,
};
use bevy_ecs_ldtk::prelude::{ LdtkEntity, LdtkFields };

use super::{ collision::ColliderBundle, patrol::Patrol };

// Npc
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Npc;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct NpcBundle {
    #[sprite_bundle("npc.png")]
    pub sprite_bundle: SpriteBundle,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    pub npc: Npc,
}

// NpcPatrol
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct NpcPatrol;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct NpcPatrolBundle {
    #[sprite_bundle("npc.png")]
    pub sprite_bundle: SpriteBundle,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    pub npc: NpcPatrol,
    #[ldtk_entity]
    pub patrol: Patrol,
}

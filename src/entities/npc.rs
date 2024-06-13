use bevy::{ ecs::{ bundle::Bundle, component::Component }, sprite::SpriteBundle };
use bevy_ecs_ldtk::{ prelude::LdtkEntity, EntityInstance };

use super::{ ColliderBundle, PredefinedPath };

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
    #[from_entity_instance]
    pub entity: EntityInstance,
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct NpcPatrolBundle {
    #[ldtk_entity]
    pub npc: NpcBundle,
    #[ldtk_entity]
    pub predefined_path: PredefinedPath,
}

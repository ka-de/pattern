use bevy::{ ecs::{ bundle::Bundle, component::Component }, sprite::SpriteBundle };
use bevy_ecs_ldtk::{ prelude::LdtkEntity, EntityInstance };

use super::ColliderBundle;

// Kade
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Kade;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct KadeBundle {
    #[sprite_bundle("kade.png")]
    pub sprite_bundle: SpriteBundle,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    pub kade: Kade,
    #[from_entity_instance]
    pub entity: EntityInstance,
}

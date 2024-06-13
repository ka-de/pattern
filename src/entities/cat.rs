use bevy::{ ecs::{ bundle::Bundle, component::Component }, sprite::SpriteSheetBundle };
use bevy_ecs_ldtk::prelude::LdtkEntity;

use super::{ColliderBundle, PredefinedPath};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Cat;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct CatBundle {
    #[sprite_sheet_bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    pub cat: Cat,
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct CatPatrolBundle {
    #[ldtk_entity]
    pub cat: CatBundle,
    #[ldtk_entity]
    pub predefined_path: PredefinedPath,
}

use bevy::{ ecs::{ bundle::Bundle, component::Component }, sprite::SpriteSheetBundle };
use bevy_ecs_ldtk::prelude::LdtkEntity;

use super::{ collision::ColliderBundle, predefinedpath::PredefinedPath };

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct MovingPlatform;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct MovingPlatformBundle {
    #[sprite_sheet_bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    #[ldtk_entity]
    pub predefined_path: PredefinedPath,
}

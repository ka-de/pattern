use bevy::{ ecs::bundle::Bundle, sprite::SpriteSheetBundle };
use bevy_ecs_ldtk::LdtkEntity;

use super::ColliderBundle;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct ChestBundle {
    #[sprite_sheet_bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
}

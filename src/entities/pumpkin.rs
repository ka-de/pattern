use bevy::{ ecs::component::Component, prelude::Bundle, sprite::SpriteSheetBundle };
use bevy_ecs_ldtk::LdtkEntity;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Pumpkin;

// 🎃
#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PumpkinBundle {
    #[sprite_sheet_bundle(no_grid)]
    pub sprite_sheet_bundle: SpriteSheetBundle,
}

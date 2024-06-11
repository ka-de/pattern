use bevy::{ prelude::Bundle, sprite::SpriteSheetBundle };
use bevy_ecs_ldtk::LdtkEntity;

// 🎃
#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PumpkinsBundle {
    #[sprite_sheet_bundle(no_grid)]
    pub sprite_sheet_bundle: SpriteSheetBundle,
}

use bevy::{ ecs::{ bundle::Bundle, component::Component }, sprite::SpriteSheetBundle };
use bevy_ecs_ldtk::LdtkEntity;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Torch;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct TorchBundle {
    #[sprite_sheet_bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub torch: Torch,
}

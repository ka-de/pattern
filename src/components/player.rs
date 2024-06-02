use bevy::ecs::bundle::Bundle;
use bevy::ecs::component::Component;
use bevy::sprite::SpriteBundle;
use bevy_ecs_ldtk::{ EntityInstance, LdtkEntity, Worldly };

use super::armor::Armor;
use super::Health;

use super::{
    climber::Climber,
    collision::ColliderBundle,
    ground::GroundDetection,
    items::Items,
};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[sprite_bundle("player.png")]
    pub sprite_bundle: SpriteBundle,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    pub player: Player,
    #[worldly]
    pub worldly: Worldly,
    pub climber: Climber,
    pub ground_detection: GroundDetection,
    pub health: Health,
    pub armor: Armor,

    // Build Items Component manually by using `impl From<&EntityInstance>`
    #[from_entity_instance]
    items: Items,

    // The whole EntityInstance can be stored directly as an EntityInstance component
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

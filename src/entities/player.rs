use std::collections::HashMap;

use bevy::prelude::*;
use bevy_ecs_ldtk::{ EntityInstance, LdtkEntity, Worldly };

use crate::{
    components::{
        armor::Armor,
        health::Health,
        collision::ColliderBundle,
        ground::GroundDetection,
        items::Items,
        climbing::Climber,
        swimming::Swimmer,
    },
    plugins::input,
    //plugins::input::{ ActionState, Slot, Ability, AbilitySlotMap },
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
    pub swimmer: Swimmer,
    pub ground_detection: GroundDetection,
    pub health: Health,
    pub armor: Armor,

    // Build Items Component manually by using `impl From<&EntityInstance>`
    #[from_entity_instance]
    items: Items,

    // The whole EntityInstance can be stored directly as an EntityInstance component
    #[from_entity_instance]
    entity_instance: EntityInstance,

    // Input manager components
    #[with(make_action_map)]
    input_bundle: input::InputBundle<Ability>,
}

// The list of possible abilities is typically longer than the list of slots
#[derive(input::Actionlike, PartialEq, Eq, Hash, Clone, Debug, Copy, Reflect)]
pub enum Ability {
    Slash,
    Shoot,
    LightningBolt,
    Fireball,
    Dash,
    Heal,
    FrozenOrb,
    PolymorphSheep,
}

fn make_action_map(_: &EntityInstance) -> input::InputBundle<Ability> {
    input::make_action_map(
        HashMap::from([
            (0, Ability::Slash),
            (1, Ability::Shoot),
            (2, Ability::FrozenOrb),
            // Some slots may be empty!
            (4, Ability::Dash),
            (5, Ability::PolymorphSheep),
        ])
    )
}

pub fn draw_health_bar(mut gizmos: Gizmos, query: Query<(&GlobalTransform, &Player, &Health)>) {
    for (transform, _, health) in query.iter() {
        let health_ratio = (health.current as f32) / (health.max as f32);
        let bar_width = 20.0f32; // Adjust as needed
        let bar_height = 0.1f32; // Adjust as needed
        let offset = Vec3::new(-10.0, 20.0, 0.0); // Offset above the player

        let start = transform.translation() + offset;
        let end = start + Vec3::new(bar_width, 0.0, 0.0);

        // Draw the background (red) bar
        gizmos.line_2d(start.truncate(), end.truncate(), Color::RED);

        // Draw the foreground (green) bar based on health
        gizmos.line_2d(
            start.truncate(),
            (start + Vec3::new(bar_width * health_ratio, 0.0, 0.0)).truncate(),
            Color::GREEN
        );
    }
}

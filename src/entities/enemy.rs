use bevy::{
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        system::{ In, Query, Res },
        query::With,
    },
    sprite::SpriteSheetBundle,
    time::Time,
    transform::components::Transform,
};
use bevy_ecs_ldtk::{ prelude::LdtkEntity, EntityInstance };
use seldom_state::{ prelude::StateMachine, trigger::IntoTrigger as _ };

use super::{ ColliderBundle, PredefinedPath };
use crate::components::line_of_sight::LineOfSight;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Enemy;

#[derive(Default, Bundle, LdtkEntity)]
pub struct EnemyBundle {
    #[sprite_sheet_bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    pub enemy: Enemy,
    #[ldtk_entity]
    pub predefined_path: PredefinedPath,
    pub line_of_sight: LineOfSight<super::player::Player>,
    #[with(make_state_machine)]
    pub state_machine: StateMachine,
    pub state: Idle,
}

//////////////////////////////////////////////////////////////////////////////////////////
/// STATE

fn make_state_machine(_: &EntityInstance) -> StateMachine {
    // FIXME: should be adapted to use LineOfSight
    let near_player = move |
        In(entity): In<Entity>,
        player_query: Query<(Entity, &Transform), With<super::Player>>,
        transforms: Query<&Transform>
    | {
        let Ok((player, player_transform)) = player_query.get_single() else {
            return Err(f32::INFINITY);
        };
        let distance = player_transform.translation
            .truncate()
            .distance(transforms.get(entity).unwrap().translation.truncate());

        // Check whether the target is within range. If it is, return `Ok` to trigger!
        match distance <= 300.0 {
            true => Ok((distance, player)),
            false => Err(distance),
        }
    };

    // This state machine handles the enemy's transitions. Transitions defined earlier have
    // priority, but triggers after the first accepted one may still be checked.
    StateMachine::default()
        // Add a transition. When they're in `Idle` state, and the `near_player` trigger occurs,
        // switch to this instance of the `Follow` state
        .trans_builder(
            near_player,
            // Transitions accept specific instances of states
            |_old_state: &Idle, (_distance, target)| {
                Some(Follow {
                    target: target,
                    speed: 100.0,
                })
            }
        )
        // Add a second transition. When they're in the `Follow` state, and the `near_player`
        // trigger does not occur, switch to the `Idle` state. `.not()` is a combinator that
        // negates the trigger. `.and(other)` and `.or(other)` also exist.
        .trans::<Follow, _>(near_player.not(), Idle)
        // Enable transition logging
        .set_trans_logging(true)
}

// Entities in the `Idle` state do nothing
#[derive(Clone, Component, Default)]
#[component(storage = "SparseSet")]
struct Idle;

// Entities in the `Follow` state move toward the given entity at the given speed
#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
struct Follow {
    target: Entity,
    speed: f32,
}

// Let's define some behavior for entities in the follow state
// FIXME: should be adapted to switch between patrol mode Idle state, and pathfinding follow during ()
fn follow(
    mut transforms: Query<&mut Transform>,
    follows: Query<(Entity, &Follow)>,
    time: Res<Time>
) {
    for (entity, follow) in &follows {
        // Get the positions of the follower and target
        let target_translation = transforms.get(follow.target).unwrap().translation;
        let follow_transform = &mut transforms.get_mut(entity).unwrap();
        let follow_translation = follow_transform.translation;

        // Find the direction from the follower to the target and go that way
        follow_transform.translation +=
            (target_translation - follow_translation).normalize_or_zero() *
            follow.speed *
            time.delta_seconds();
    }
}

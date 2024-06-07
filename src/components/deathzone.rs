use bevy::prelude::*;
use bevy_ecs_ldtk::{ assets::LdtkProject, LdtkEntity, Respawn };
use bevy_rapier2d::prelude::*;

use super::player::Player;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct DeathZone;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct DeathZoneBundle {
    pub death_zone: DeathZone,
    pub collider: Collider,
    pub sensor: Sensor,
    pub transform: Transform,
}

impl DeathZoneBundle {
    pub fn new(width: f32, height: f32, position: Vec3) -> Self {
        DeathZoneBundle {
            death_zone: DeathZone,
            collider: Collider::cuboid(width / 2.0, height / 2.0),
            sensor: Sensor,
            transform: Transform::from_translation(position),
        }
    }
}

// ⚠️ TODO: Fade out the screen.
pub fn detect_death_zone_collision(
    mut commands: Commands,
    ldtk_projects: Query<Entity, With<Handle<LdtkProject>>>,
    mut collision_events: EventReader<CollisionEvent>,
    death_zones: Query<Entity, With<DeathZone>>,
    players: Query<Entity, With<Player>>
) {
    for collision in collision_events.read() {
        match collision {
            CollisionEvent::Started(collider_a, collider_b, _) => {
                println!("Collision detected between {:?} and {:?}", collider_a, collider_b);
                let is_death_zone_collision =
                    (death_zones.contains(*collider_a) && players.contains(*collider_b)) ||
                    (death_zones.contains(*collider_b) && players.contains(*collider_a));

                if is_death_zone_collision {
                    println!("DeathZone collision detected!");
                    commands.entity(ldtk_projects.single()).insert(Respawn);
                }
            }
            _ => {}
        }
    }
}

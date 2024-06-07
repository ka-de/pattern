use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::{ prelude::*, rapier::geometry::InteractionGroups };

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub gravity_scale: GravityScale,
    pub friction: Friction,
    pub density: ColliderMassProperties,
}

impl From<&EntityInstance> for ColliderBundle {
    fn from(entity_instance: &EntityInstance) -> ColliderBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        match entity_instance.identifier.as_ref() {
            "Player" =>
                ColliderBundle {
                    collider: Collider::cuboid(6.0, 14.0),
                    friction: Friction {
                        coefficient: 0.0,
                        combine_rule: CoefficientCombineRule::Min,
                    },
                    rotation_constraints,
                    ..Default::default()
                },
            "Mob" =>
                ColliderBundle {
                    collider: Collider::cuboid(5.0, 5.0),
                    rigid_body: RigidBody::KinematicVelocityBased,
                    rotation_constraints,
                    ..Default::default()
                },
            "Npc" =>
                ColliderBundle {
                    collider: Collider::cuboid(5.0, 5.0),
                    rigid_body: RigidBody::KinematicVelocityBased,
                    rotation_constraints,
                    density: ColliderMassProperties::Density(50.0),
                    ..Default::default()
                },
            "NpcPatrol" =>
                ColliderBundle {
                    collider: Collider::cuboid(5.0, 5.0),
                    rigid_body: RigidBody::KinematicVelocityBased,
                    rotation_constraints,
                    density: ColliderMassProperties::Density(50.0),
                    ..Default::default()
                },
            "Chest" =>
                ColliderBundle {
                    collider: Collider::cuboid(8.0, 8.0),
                    rotation_constraints,
                    density: ColliderMassProperties::Density(50.0),
                    ..default()
                },
            _ => {
                debug!("default ColliderBundle used for: {}", entity_instance.identifier);
                ColliderBundle {
                    collider: Collider::cuboid(
                        (entity_instance.width as f32) / 2.0,
                        (entity_instance.height as f32) / 2.0
                    ),
                    rotation_constraints,
                    ..default()
                }
            }
        }
    }
}

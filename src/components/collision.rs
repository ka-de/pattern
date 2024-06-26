use bevy::{ prelude::*, utils::HashMap, utils::HashSet, ecs::system::EntityCommands };
use bevy_ecs_ldtk::{
    assets::LdtkProject,
    ldtk::{ loaded_level::LoadedLevel, LayerInstance },
    EntityInstance,
    GridCoords,
    LdtkIntCell,
    LevelIid,
};
use bevy_rapier2d::{
    dynamics::{ CoefficientCombineRule, GravityScale, LockedAxes, RigidBody, Velocity },
    geometry::{ Collider, ColliderMassProperties, CollisionGroups, Friction, Group },
};

use crate::entities::intcells::Wall;

/// Spawns heron collisions for the walls that have just been spawned
///
/// Lookup the levels corresponding to the walls that have been spawned, and
/// associate to them the GridCoords of the walls.
///
/// See [`spawn_wall_collision_for_level`] for the actual collider generation
/// algorithm.
pub fn spawn_wall_collision(
    mut commands: Commands,
    wall_query: Query<(&GridCoords, &Parent), Added<Wall>>,
    parent_query: Query<&Parent, Without<Wall>>,
    level_query: Query<(Entity, &LevelIid)>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>
) {
    if wall_query.is_empty() {
        return;
    }

    let ldtk_project = ldtk_project_assets
        .get(ldtk_projects.single())
        .expect("Project should be loaded if level has spawned");

    // Consider where the walls are
    // storing them as GridCoords in a HashSet for quick, easy lookup
    //
    // The key of this map will be the entity of the level the wall belongs to.
    // This has two consequences in the resulting collision entities:
    // 1. it forces the walls to be split along level boundaries
    // 2. it lets us easily add the collision entities as children of the appropriate level entity
    let mut level_to_wall_locations: HashMap<Entity, HashSet<GridCoords>> = HashMap::new();

    wall_query.iter().for_each(|(&grid_coords, parent)| {
        // An intgrid tile's direct parent will be a layer entity, not the level entity
        // To get the level entity, you need the tile's grandparent.
        // This is where parent_query comes in.
        if let Ok(grandparent) = parent_query.get(parent.get()) {
            level_to_wall_locations.entry(grandparent.get()).or_default().insert(grid_coords);
        }
    });

    level_query.iter().for_each(|(level_entity, level_iid)| {
        if let Some(level_walls) = level_to_wall_locations.get(&level_entity) {
            let level = ldtk_project
                .as_standalone()
                .get_loaded_level_by_iid(&level_iid.to_string())
                .expect("Spawned level should exist in LDtk project");

            spawn_wall_collision_for_level(level, level_walls, commands.entity(level_entity));
        }
    });
}

/// Spawns heron collisions for the walls of a level
///
/// You could just insert a ColliderBundle in to the WallBundle,
/// but this spawns a different collider for EVERY wall tile.
/// This approach leads to bad performance.
///
/// Instead, by flagging the wall tiles and spawning the collisions later,
/// we can minimize the amount of colliding entities.
///
/// The algorithm used here is a nice compromise between simplicity, speed,
/// and a small number of rectangle colliders.
/// In basic terms, it will:
/// 1. consider where the walls are
/// 2. combine wall tiles into flat "plates" in each individual row
/// 3. combine the plates into rectangles across multiple rows wherever possible
/// 4. spawn colliders for each rectangle
fn spawn_wall_collision_for_level(
    level: LoadedLevel,
    level_walls: &bevy::utils::hashbrown::HashSet<GridCoords>,
    mut entity_commands: EntityCommands
) {
    /// Represents a wide wall that is 1 tile tall
    /// Used to spawn wall collisions
    #[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Plate {
        left: i32,
        right: i32,
    }

    /// A simple rectangle type representing a wall of any size
    struct Rect {
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
    }

    let LayerInstance {
        c_wid: width,
        c_hei: height,
        grid_size,
        ..
    } = *level
        .layer_instances()
        .iter()
        .filter(|level| level.identifier == "Collisions")
        .next()
        .expect("could not find the Collisions layer");

    // combine wall tiles into flat "plates" in each individual row
    let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

    for y in 0..height {
        let mut row_plates: Vec<Plate> = Vec::new();
        let mut plate_start = None;

        // + 1 to the width so the algorithm "terminates" plates that touch the right edge
        for x in 0..width + 1 {
            match (plate_start, level_walls.contains(&(GridCoords { x, y }))) {
                (Some(s), false) => {
                    row_plates.push(Plate {
                        left: s,
                        right: x - 1,
                    });
                    plate_start = None;
                }
                (None, true) => {
                    plate_start = Some(x);
                }
                _ => (),
            }
        }

        plate_stack.push(row_plates);
    }

    // combine "plates" into rectangles across multiple rows
    let mut rect_builder: HashMap<Plate, Rect> = HashMap::new();
    let mut prev_row: Vec<Plate> = Vec::new();
    let mut wall_rects: Vec<Rect> = Vec::new();

    // an extra empty row so the algorithm "finishes" the rects that touch the top edge
    plate_stack.push(Vec::new());

    for (y, current_row) in plate_stack.into_iter().enumerate() {
        for prev_plate in &prev_row {
            if !current_row.contains(prev_plate) {
                // remove the finished rect so that the same plate in the future starts a new rect
                if let Some(rect) = rect_builder.remove(prev_plate) {
                    wall_rects.push(rect);
                }
            }
        }
        for plate in &current_row {
            rect_builder
                .entry(plate.clone())
                .and_modify(|e| {
                    e.top += 1;
                })
                .or_insert(Rect {
                    bottom: y as i32,
                    top: y as i32,
                    left: plate.left,
                    right: plate.right,
                });
        }
        prev_row = current_row;
    }

    entity_commands.with_children(|level| {
        // Spawn colliders for every rectangle..
        // Making the collider a child of the level serves two purposes:
        // 1. Adjusts the transforms to be relative to the level for free
        // 2. the colliders will be despawned automatically when levels unload
        for wall_rect in wall_rects {
            level
                .spawn_empty()
                .insert(Name::new("wall_collision"))
                .insert(
                    Collider::cuboid(
                        (((wall_rect.right as f32) - (wall_rect.left as f32) + 1.0) *
                            (grid_size as f32)) /
                            2.0,
                        (((wall_rect.top as f32) - (wall_rect.bottom as f32) + 1.0) *
                            (grid_size as f32)) /
                            2.0
                    )
                )
                .insert(RigidBody::Fixed)
                .insert(Friction::new(1.0))
                .insert(
                    Transform::from_xyz(
                        (((wall_rect.left + wall_rect.right + 1) as f32) * (grid_size as f32)) /
                            2.0,
                        (((wall_rect.bottom + wall_rect.top + 1) as f32) * (grid_size as f32)) /
                            2.0,
                        0.0
                    )
                )
                .insert(GlobalTransform::default());
        }
    });
}

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub gravity_scale: GravityScale,
    pub friction: Friction,
    pub density: ColliderMassProperties,
    pub collision_group: CollisionGroups,
}

const PLAYER_GROUP: Group = Group::GROUP_1;
const NPC_GROUP: Group = Group::GROUP_2;

impl From<&EntityInstance> for ColliderBundle {
    fn from(entity_instance: &EntityInstance) -> ColliderBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        match entity_instance.identifier.as_ref() {
            "Player" =>
                ColliderBundle {
                    collider: Collider::cuboid(8.0, 12.0),
                    friction: Friction {
                        coefficient: 0.0,
                        combine_rule: CoefficientCombineRule::Min,
                    },
                    rotation_constraints,
                    collision_group: CollisionGroups::new(PLAYER_GROUP, Group::ALL - NPC_GROUP),
                    ..Default::default()
                },
            "Enemy" =>
                ColliderBundle {
                    collider: Collider::cuboid(12.0, 12.0),
                    rigid_body: RigidBody::KinematicVelocityBased,
                    rotation_constraints,
                    ..Default::default()
                },
            "Npc" =>
                ColliderBundle {
                    collider: Collider::cuboid(12.0, 12.0),
                    rigid_body: RigidBody::KinematicVelocityBased,
                    rotation_constraints,
                    density: ColliderMassProperties::Density(50.0),
                    collision_group: CollisionGroups::new(NPC_GROUP, Group::ALL - NPC_GROUP),
                    ..Default::default()
                },
            "NpcPatrol" =>
                ColliderBundle {
                    collider: Collider::cuboid(12.0, 12.0),
                    rigid_body: RigidBody::KinematicVelocityBased,
                    rotation_constraints,
                    density: ColliderMassProperties::Density(50.0),
                    collision_group: CollisionGroups::new(NPC_GROUP, Group::ALL),
                    ..Default::default()
                },
            "Cauldron" =>
                ColliderBundle {
                    collider: Collider::cuboid(12.0, 12.0),
                    rigid_body: RigidBody::KinematicVelocityBased,
                    rotation_constraints,
                    density: ColliderMassProperties::Density(50.0),
                    collision_group: CollisionGroups::new(NPC_GROUP, Group::ALL),
                    ..Default::default()
                },
            "Kade" =>
                ColliderBundle {
                    collider: Collider::cuboid(12.0, 12.0),
                    rigid_body: RigidBody::KinematicVelocityBased,
                    rotation_constraints,
                    density: ColliderMassProperties::Density(50.0),
                    collision_group: CollisionGroups::new(NPC_GROUP, Group::ALL),
                    ..Default::default()
                },
            "Dog" =>
                ColliderBundle {
                    collider: Collider::cuboid(12.0, 12.0),
                    rigid_body: RigidBody::KinematicVelocityBased,
                    rotation_constraints,
                    density: ColliderMassProperties::Density(50.0),
                    collision_group: CollisionGroups::new(NPC_GROUP, Group::ALL),
                    ..Default::default()
                },
            "DogPatrol" =>
                ColliderBundle {
                    collider: Collider::cuboid(12.0, 12.0),
                    rigid_body: RigidBody::KinematicVelocityBased,
                    rotation_constraints,
                    density: ColliderMassProperties::Density(50.0),
                    collision_group: CollisionGroups::new(NPC_GROUP, Group::ALL),
                    ..Default::default()
                },
            "Cat" =>
                ColliderBundle {
                    collider: Collider::cuboid(12.0, 12.0),
                    rigid_body: RigidBody::KinematicVelocityBased,
                    rotation_constraints,
                    density: ColliderMassProperties::Density(50.0),
                    collision_group: CollisionGroups::new(NPC_GROUP, Group::ALL),
                    ..Default::default()
                },
            "CatPatrol" =>
                ColliderBundle {
                    collider: Collider::cuboid(12.0, 12.0),
                    rigid_body: RigidBody::KinematicVelocityBased,
                    rotation_constraints,
                    density: ColliderMassProperties::Density(50.0),
                    collision_group: CollisionGroups::new(NPC_GROUP, Group::ALL),
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

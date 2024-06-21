use bevy::{ ecs::{ entity::Entity, system::Query }, log::debug };

use super::name::Name;

#[derive(Relation)]
pub struct ChildOf;

pub fn debug_children(
    tree: Query<(&Name, Relations<ChildOf>)>,
    roots: Query<Entity, Root<ChildOf>>
) {
    tree.traverse::<ChildOf>(roots.iter())
        .track_self()
        .for_each(|Name(parent), _, Name(child), _| {
            debug!("{} is the parent of {}", parent, child);
        });
}

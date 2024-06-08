use bevy::{
    ecs::{ component::Component, entity::Entity, query::With, system::{ Commands, Query, Res } },
    time::Time,
};

use super::banner_widget::BannerWidgetCommands;

#[derive(Component)]
pub struct Bouncing;

pub fn bouncing_labels(
    mut commands: Commands,
    bouncing_labels: Query<Entity, With<Bouncing>>,
    time: Res<Time>
) {
    for entity in bouncing_labels.iter() {
        commands
            .entity(entity)
            .set_position(700.0, 100.0 + (time.elapsed_seconds() * 100.0).sin() * 50.0);
    }
}

use bevy::{
    ecs::{ component::Component, entity::Entity, query::With, system::{ Commands, Query, Res } },
    time::Time,
};

use super::banner_widget::BannerWidgetCommands;

#[derive(Component)]
pub struct Rotating;

pub fn rotating_labels(
    mut commands: Commands,
    rotating_labels: Query<Entity, With<Rotating>>,
    time: Res<Time>
) {
    for entity in rotating_labels.iter() {
        let angle = time.elapsed_seconds() * 2.0 * std::f32::consts::PI;
        commands
            .entity(entity)
            .set_position(700.0 + angle.cos() * 100.0, 100.0 + angle.sin() * 100.0);
    }
}

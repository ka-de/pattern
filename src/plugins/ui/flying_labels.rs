use bevy::{ prelude::{ Commands, Entity, Query, Res, With }, time::Time };

use super::{ banner_widget::BannerWidgetCommands, flying::Flying };

pub fn flying_labels(
    mut commands: Commands,
    flying_labels: Query<Entity, With<Flying>>,
    time: Res<Time>
) {
    for entity in flying_labels.iter() {
        commands
            .entity(entity)
            .set_position(
                700.0 + time.elapsed_seconds().sin() * 100.0,
                100.0 + time.elapsed_seconds().cos() * 100.0
            );
    }
}

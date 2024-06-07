use bevy::app::{ Startup, Update };
use release_label::release_label;

pub(crate) mod fps_widget;
pub(crate) mod flying_labels;
pub(crate) mod release_label;
pub(crate) mod flying;
pub(crate) mod set;
pub(crate) mod banner_widget;

use crate::plugins::ui::flying_labels::flying_labels;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_systems(Startup, release_label).add_systems(Update, flying_labels);
}

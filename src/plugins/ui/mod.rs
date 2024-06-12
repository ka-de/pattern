use bevy::app::{ Startup, PreUpdate, Update };
use tooltip::spawn_tooltip;
use release_label::release_label;

pub(crate) mod bouncing;
pub(crate) mod flying;
pub(crate) mod rotating;
pub(crate) mod zooming;
pub(crate) mod set_window_icon;
pub(crate) mod fps_widget;
pub(crate) mod release_label;
pub(crate) mod set;
pub(crate) mod banner_widget;
pub(crate) mod settings_widget;
mod tooltip;

use crate::plugins::ui::flying::flying_labels;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_systems(Startup, release_label)
        .add_systems(Update, flying_labels)
        .add_systems(PreUpdate, spawn_tooltip);
}

use bevy::app::{ Startup, Update };
use release_label::release_label;
use keycap_demo::keycap_demo;

// release_label depends on it, this also shares a lot of code with keycap
// but keycap needs to be dynamically replaced with a gamepad button later! ⚠️
pub(crate) mod banner_widget;

pub(crate) mod set_window_icon;
pub(crate) mod fps_widget;
pub(crate) mod release_label;
pub(crate) mod set;
pub(crate) mod keycap;
pub(crate) mod keycap_demo;
/*
pub(crate) mod flying;
pub(crate) mod bouncing;
pub(crate) mod rotating;
pub(crate) mod zooming;

pub(crate) mod settings_widget;

use crate::plugins::ui::flying::flying_labels;
*/

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_systems(Startup, release_label); // 🐺
    //.add_systems(Startup, keycap_demo);
    //.add_systems(Update, flying_labels);
}

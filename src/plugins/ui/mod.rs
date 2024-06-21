use bevy::{ app::{ PreUpdate, Startup, Update }, render::{ camera::ClearColor, color::Color } };

use release_label::release_label;
use tooltip::spawn_tooltip;
use bevy_vector_shapes::prelude::*;
use aery::prelude::*;

//use keycap_demo::keycap_demo;

// release_label depends on it, this also shares a lot of code with keycap
// but keycap needs to be dynamically replaced with a gamepad button later! ⚠️
pub(crate) mod banner_widget;

pub(crate) mod fps_widget;
// Rewrite with shapes?
//pub(crate) mod keycap;
//pub(crate) mod keycap_demo;
pub(crate) mod release_label;
pub(crate) mod set;
pub(crate) mod set_window_icon;
mod tooltip;

/*
pub(crate) mod flying;
pub(crate) mod bouncing;
pub(crate) mod rotating;
pub(crate) mod zooming;

pub(crate) mod settings_widget;

use crate::plugins::ui::flying::flying_labels;
use crate::plugins::ui::keycap_demo::keycap_demo;
*/

pub(crate) fn plugin(app: &mut bevy::app::App) {
    // Resources
    app.insert_resource(ClearColor(Color::DARK_GRAY))
        // Plugins
        .add_plugins(ShapePlugin::default())
        // 🐺
        // Startup
        //.add_systems(Startup, keycap_demo)
        .add_systems(Startup, release_label)

        // Update
        //.add_systems(Update, flying_labels)

        // PreUpdate
        .add_systems(PreUpdate, spawn_tooltip);
}

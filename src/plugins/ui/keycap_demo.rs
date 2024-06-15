use bevy::{ prelude::Commands, ui::Val };
use sickle_ui::{
    ui_builder::{ UiBuilderExt, UiRoot },
    ui_style::{ SetNodeBottomExt, SetNodeRightExt },
};

use super::keycap::{ KeycapWidgetConfig, UiKeycapWidgetExt };

pub fn keycap_demo(mut commands: Commands) {
    commands
        .ui_builder(UiRoot)
        .keycap_widget(KeycapWidgetConfig::from("E", "fonts/dogicabold.ttf", 8.0))
        .style()
        .right(Val::Px(100.0))
        .bottom(Val::Px(100.0));
}

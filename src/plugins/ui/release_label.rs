use bevy::{ prelude::Commands, ui::Val };
use sickle_ui::{
    ui_builder::{ UiBuilderExt, UiRoot },
    ui_style::{ SetNodeBottomExt, SetNodeLeftExt },
};

use super::banner_widget::{ BannerWidgetConfig, UiBannerWidgetExt };

// release_label
//
// Prints out release or debug build on the UI.
pub fn release_label(mut commands: Commands) {
    // Print out "DEVELOPMENT BUILD" when not in release mode.
    #[cfg(debug_assertions)]
    commands
        .ui_builder(UiRoot)
        .banner_widget(BannerWidgetConfig::from("DEVELOPMENT BUILD", "fonts/bahnschrift.ttf", 8.0))
        .style()
        .left(Val::Px(100.0))
        .bottom(Val::Px(100.0));
    // ⚠️ TODO: This will have to go away from the actual release build
    // Print out "ALPHA RELEASE BUILD" when in release mode.
    #[cfg(not(debug_assertions))]
    commands
        .ui_builder(UiRoot)
        .banner_widget(
            BannerWidgetConfig::from("ALPHA RELEASE BUILD", "fonts/bahnschrift.ttf", 8.0)
        )
        .style()
        .left(Val::Px(100.0))
        .bottom(Val::Px(100.0));
}

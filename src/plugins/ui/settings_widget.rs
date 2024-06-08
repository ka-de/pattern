use bevy::prelude::*;
use sickle_ui::ui_builder::{ UiBuilder, UiRoot };
use sickle_ui::ui_style::{ SetNodeHeightExt, SetNodeLeftExt, SetNodeTopExt, SetNodeWidthExt };
use sickle_ui::widgets::button::{ ButtonConfig, UiButtonExt };

use crate::plugins::ui::banner_widget::{ BannerWidgetConfig, UiBannerWidgetExt };
use crate::settings::{ GameSettings, GameWindowLevel };

pub fn spawn_settings_ui(mut commands: Commands) {
    commands.spawn(NodeBundle::default()).with_children(|parent| {
        let mut ui = UiBuilder::new(parent);

        ui.banner_widget(BannerWidgetConfig::from("Window Level", "fonts/bahnschrift.ttf", 16.0))
            .style()
            .left(Val::Px(10.0))
            .top(Val::Px(50.0));

        ui.button(ButtonConfig::new("Normal"), |btn| {
            btn.style().left(Val::Px(10.0)).top(Val::Px(70.0));
            btn.on_click(|world: &mut World| {
                world.resource_mut::<GameSettings>().window_level = GameWindowLevel::Normal;
            });
        });

        ui.button(ButtonConfig::new("Always on Top"), |btn| {
            btn.style().left(Val::Px(110.0)).top(Val::Px(70.0));
            btn.on_click(|world: &mut World| {
                world.resource_mut::<GameSettings>().window_level = GameWindowLevel::AlwaysOnTop;
            });
        });
    });
}

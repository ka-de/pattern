use bevy::{
    ecs::system::EntityCommands,
    prelude::{ Component, Entity },
    render::color::Color,
    ui::{ node_bundles::ImageBundle, AlignSelf, JustifyContent, PositionType, Val },
};
use sickle_ui::{
    ui_builder::{ UiBuilder, UiRoot },
    ui_commands::SetTextExt,
    ui_style::{
        SetImageExt,
        SetNodeAlignSelfExt,
        SetNodeHeightExt,
        SetNodeJustifyContentsExt,
        SetNodePositionTypeExt,
        SetNodeTopExt,
        SetNodeWidthExt,
    },
    widgets::{ container::UiContainerExt, label::{ LabelConfig, UiLabelExt } },
};

use crate::plugins::ui::set::{ SetFont, SetPosition };

/// BannerWidget
#[derive(Component)]
pub(crate) struct BannerWidget;

/// BannerLabel
/// A marker component used internally to initialize the label font.
#[derive(Component)]
struct BannerLabel;

/// BannerWidgetConfig
pub struct BannerWidgetConfig {
    pub label: String,
    pub font: String,
    pub font_size: f32,
}

impl BannerWidgetConfig {
    pub fn from(
        label: impl Into<String>,
        font: impl Into<String>,
        font_size: impl Into<f32>
    ) -> Self {
        Self {
            label: label.into(),
            font: font.into(),
            font_size: font_size.into(),
        }
    }
}

pub trait UiBannerWidgetExt<'w, 's> {
    fn banner_widget<'a>(&'a mut self, config: BannerWidgetConfig) -> UiBuilder<'w, 's, 'a, Entity>;
}

impl<'w, 's> UiBannerWidgetExt<'w, 's> for UiBuilder<'w, 's, '_, UiRoot> {
    fn banner_widget<'a>(
        &'a mut self,
        config: BannerWidgetConfig
    ) -> UiBuilder<'w, 's, 'a, Entity> {
        self.container((ImageBundle::default(), BannerWidget), |banner| {
            banner
                .style()
                .position_type(PositionType::Absolute)
                // Center the children (the label) horizontally.
                .justify_content(JustifyContent::Center)
                .width(Val::Px(100.0))
                .height(Val::Px(12.0))
                // Add a nice looking background image to our widget.
                .image("ui/label_gradient_transparent.png");

            // And we'll want a customizable label on the banner.
            let mut label = banner.label(LabelConfig::default());

            label
                .style()
                // Align the label relative to the top of the banner.
                .align_self(AlignSelf::Start)
                // Move us a few pixels down so we look nice relative to our font.
                .top(Val::Px(3.0));

            // We would like to set a default text style without having to pass in the AssetServer.
            label
                .entity_commands()
                .insert(BannerLabel)
                .set_text(config.label, None)
                .font(config.font, config.font_size, Color::rgb(1.0, 1.0, 1.0));
        })
    }
}

/// BannerWidgetCommands
///
/// An extension trait that exposes the SetFont command.
pub trait BannerWidgetCommands<'a> {
    fn font(
        &'a mut self,
        font: impl Into<String>,
        size: f32,
        color: Color
    ) -> &mut EntityCommands<'a>;

    fn set_position(&'a mut self, x: f32, y: f32) -> &mut EntityCommands<'a>;
}

impl<'a> BannerWidgetCommands<'a> for EntityCommands<'a> {
    fn font(
        &'a mut self,
        font: impl Into<String>,
        size: f32,
        color: Color
    ) -> &mut EntityCommands<'a> {
        self.add(SetFont(font.into(), size, color))
    }

    fn set_position(&'a mut self, x: f32, y: f32) -> &mut EntityCommands<'a> {
        // We insert our custom command into the entity commands queue.
        self.add(SetPosition(x, y))
    }
}

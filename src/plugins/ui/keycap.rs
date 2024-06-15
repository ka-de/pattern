use crate::plugins::ui::set::{ SetFont, SetPosition };
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

#[derive(Component)]
pub(crate) struct KeycapWidget;

#[derive(Component)]
struct KeycapLabel;

pub struct KeycapWidgetConfig {
    pub label: String,
    pub font: String,
    pub font_size: f32,
}

impl KeycapWidgetConfig {
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

pub trait UiKeycapWidgetExt<'w, 's> {
    fn keycap_widget<'a>(&'a mut self, config: KeycapWidgetConfig) -> UiBuilder<'w, 's, 'a, Entity>;
}

impl<'w, 's> UiKeycapWidgetExt<'w, 's> for UiBuilder<'w, 's, '_, UiRoot> {
    fn keycap_widget<'a>(
        &'a mut self,
        config: KeycapWidgetConfig
    ) -> UiBuilder<'w, 's, 'a, Entity> {
        self.container((ImageBundle::default(), KeycapWidget), |keycap| {
            keycap
                .style()
                .position_type(PositionType::Absolute)
                .justify_content(JustifyContent::Center)
                .width(Val::Px(25.0))
                .height(Val::Px(25.0))
                .image("ui/keycap.png");
            let mut label = keycap.label(LabelConfig::default());
            label.style().align_self(AlignSelf::Start).top(Val::Px(3.0));
            label
                .entity_commands()
                .insert(KeycapLabel)
                .set_text(config.label, None)
                .font(config.font, config.font_size, Color::rgb(1.0, 1.0, 1.0));
        })
    }
}

pub trait KeycapWidgetCommands<'a> {
    fn font(
        &'a mut self,
        font: impl Into<String>,
        size: f32,
        color: Color
    ) -> &mut EntityCommands<'a>;
    fn set_position(&'a mut self, x: f32, y: f32) -> &mut EntityCommands<'a>;
}

impl<'a> KeycapWidgetCommands<'a> for EntityCommands<'a> {
    fn font(
        &'a mut self,
        font: impl Into<String>,
        size: f32,
        color: Color
    ) -> &mut EntityCommands<'a> {
        self.add(SetFont(font.into(), size, color))
    }
    fn set_position(&'a mut self, x: f32, y: f32) -> &mut EntityCommands<'a> {
        self.add(SetPosition(x, y))
    }
}

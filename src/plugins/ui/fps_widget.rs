use bevy::{
    app::{ App, Update },
    ecs::{ entity::Entity, component::Component, query::With, system::{ Commands, Query, Res } },
    render::color::Color,
    text::TextStyle,
    ui::{ node_bundles::NodeBundle, JustifyContent, PositionType, Val },
};

use bevy::{ asset::AssetServer, diagnostic::{ DiagnosticsStore, FrameTimeDiagnosticsPlugin } };
use sickle_ui::{
    ui_builder::{ UiBuilder, UiBuilderExt, UiRoot },
    ui_commands::SetTextExt,
    ui_style::{
        SetBackgroundColorExt,
        //SetNodeBottomExt,
        SetNodeJustifyContentsExt,
        SetNodePositionTypeExt,
        SetNodeRightExt,
        SetNodeTopExt,
    },
    widgets::{ container::UiContainerExt, label::{ LabelConfig, UiLabelExt } },
};

pub fn plugin(app: &mut App) {
    app.add_plugins(FrameTimeDiagnosticsPlugin).add_systems(Update, update_fps);
}

#[derive(Component)]
pub struct FpsWidget;

#[derive(Component, Default)]
struct FpsText;

pub trait UiFPSWidgetExt<'w, 's> {
    fn fps<'a>(&'a mut self) -> UiBuilder<'w, 's, 'a, Entity>;
}

impl<'w, 's> UiFPSWidgetExt<'w, 's> for UiBuilder<'w, 's, '_, UiRoot> {
    fn fps<'a>(&'a mut self) -> UiBuilder<'w, 's, 'a, Entity> {
        self.container((NodeBundle::default(), FpsWidget), |fps| {
            fps.style()
                .position_type(PositionType::Absolute)
                .right(Val::Px(10.0))
                .top(Val::Px(10.0))
                .justify_content(JustifyContent::Center)
                .background_color(Color::NONE);

            let mut label = fps.label(LabelConfig::default());

            label.entity_commands().insert(FpsText).set_text("0".to_string(), None);
        })
    }
}

pub fn spawn_fps_widget(mut commands: Commands) {
    commands.ui_builder(UiRoot).fps();
}

fn update_fps(
    mut commands: Commands,
    diagnostics: Res<DiagnosticsStore>,
    label: Query<Entity, With<FpsText>>,
    asset_server: Res<AssetServer>
) {
    for label in &label {
        let Some(fps_diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) else {
            continue;
        };

        let Some(smoothed_fps) = fps_diagnostic.smoothed() else {
            continue;
        };

        // Target frame rate for 60 Hz monitors is actually slightly less than 60,
        // so we round down slightly to avoid flickering under happy circumstances.
        let text_color = if smoothed_fps > 59.5 {
            Color::GREEN
        } else if smoothed_fps > 30.0 {
            Color::YELLOW
        } else {
            Color::RED
        };

        let text_style = TextStyle {
            font: asset_server.load("fonts/bahnschrift.ttf"),
            font_size: 12.0,
            color: text_color,
        };

        commands.entity(label).set_text(format!("{:3.0}", smoothed_fps), text_style.into());
    }
}

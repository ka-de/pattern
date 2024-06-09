use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;
use super::Assets;

/// Marker for the [`Node`] that is the root of the UI
#[derive(Debug, Default, Component)]
pub struct UiRootNode;

#[derive(Debug, Default, Component)]
pub(crate) struct DialogueNode;

#[derive(Debug, Default, Component)]
pub(crate) struct DialogueNameNode;

#[derive(Debug, Default, Component)]
pub(crate) struct DialogueContinueNode;

#[derive(Debug, Default, Component)]
pub(crate) struct OptionsNode;

#[derive(Debug, Component)]
pub(crate) struct OptionButton(pub OptionId);

pub(crate) fn setup(mut commands: Commands, assets: Res<Assets>) {
    // root node
    commands
        .spawn((
            fmt_name("root"),
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::FlexEnd,
                    align_items: AlignItems::Center,
                    padding: UiRect::bottom(Val::Px(30.0)),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                visibility: Visibility::Hidden,
                ..default()
            },
            UiRootNode,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    fmt_name("top"),
                    NodeBundle {
                        style: Style {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        fmt_name("top edge image"),
                        ImageBundle {
                            image: UiImage {
                                // 29 pixels high
                                texture: assets.edge.clone(),
                                ..default()
                            },
                            style: Style {
                                width: Val::Px(DIALOG_WIDTH),
                                ..default()
                            },
                            ..default()
                        },
                    ));
                })
                .with_children(|parent| {
                    parent.spawn((
                        fmt_name("name"),
                        TextBundle {
                            text: Text::from_section(String::new(), text_style::name(&assets)),
                            style: Style {
                                position_type: PositionType::Absolute,
                                left: Val::Px(TEXT_BORDER / 2.0),
                                top: Val::Px(-8.0),
                                ..default()
                            },
                            z_index: ZIndex::Local(1),
                            ..default()
                        },
                        DialogueNameNode,
                        Label,
                    ));
                });

            parent
                .spawn((
                    fmt_name("dialogue"),
                    NodeBundle {
                        style: Style {
                            width: Val::Px(DIALOG_WIDTH),
                            min_height: Val::Px(50.0),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::SpaceAround,
                            align_items: AlignItems::FlexStart,
                            padding: UiRect::horizontal(Val::Px(TEXT_BORDER)),
                            ..default()
                        },
                        background_color: Color::BLACK.with_a(0.8).into(),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    // Dialog itself
                    parent.spawn((
                        fmt_name("text"),
                        TextBundle::from_section(
                            String::new(),
                            text_style::standard(&assets)
                        ).with_style(style::standard()),
                        DialogueNode,
                        Label,
                    ));
                })
                .with_children(|parent| {
                    // Options
                    parent.spawn((
                        fmt_name("options"),
                        NodeBundle {
                            style: Style {
                                display: Display::None,
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::FlexEnd,
                                align_items: AlignItems::FlexStart,
                                margin: UiRect::top(Val::Px(20.0)),
                                ..default()
                            },
                            visibility: Visibility::Hidden,
                            ..default()
                        },
                        OptionsNode,
                    ));
                });

            parent
                .spawn((
                    fmt_name("bottom"),
                    NodeBundle {
                        style: Style {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        fmt_name("bottom edge image"),
                        ImageBundle {
                            image: UiImage {
                                // 29 pixels high
                                texture: assets.edge.clone(),
                                flip_y: true,
                                ..default()
                            },
                            style: Style {
                                width: Val::Px(DIALOG_WIDTH),
                                ..default()
                            },
                            ..default()
                        },
                    ));
                })
                .with_children(|parent| {
                    parent.spawn((
                        fmt_name("continue indicator image"),
                        ImageBundle {
                            image: UiImage {
                                // 27 x 27 pixels
                                texture: assets.continue_indicator.clone(),
                                ..default()
                            },
                            style: Style {
                                position_type: PositionType::Absolute,
                                bottom: Val::Px(INITIAL_DIALOGUE_CONTINUE_BOTTOM),
                                ..default()
                            },
                            z_index: ZIndex::Local(1),
                            visibility: Visibility::Hidden,
                            ..default()
                        },
                        DialogueContinueNode,
                    ));
                });
        });
}

fn fmt_name(name: &str) -> Name {
    Name::new(format!("Yarn Spinner  dialogue view {name} node"))
}

pub(crate) const INITIAL_DIALOGUE_CONTINUE_BOTTOM: f32 = -5.0;

pub(crate) fn create_dialog_text(
    text: impl Into<String>,
    invisible: impl Into<String>,
    assets: &Assets
) -> Text {
    Text::from_sections([
        TextSection {
            value: text.into(),
            style: text_style::standard(assets),
        },
        TextSection {
            value: invisible.into(),
            style: TextStyle {
                color: Color::NONE,
                ..text_style::standard(assets)
            },
        },
    ])
}

pub(crate) fn spawn_options<'a, T>(
    entity_commands: &mut EntityCommands,
    options: T,
    assets: &Assets
)
    where T: IntoIterator<Item = &'a DialogueOption>, <T as IntoIterator>::IntoIter: 'a
{
    entity_commands.with_children(|parent| {
        for (i, option) in options.into_iter().enumerate() {
            parent
                .spawn((
                    fmt_name("option button"),
                    ButtonBundle {
                        style: Style {
                            justify_content: JustifyContent::FlexStart,
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    },
                    OptionButton(option.id),
                ))
                .with_children(|parent| {
                    let sections = [
                        TextSection {
                            value: format!("{}: ", i + 1),
                            style: text_style::option_id(assets),
                        },
                        TextSection {
                            value: option.line.text.clone(),
                            style: text_style::option_text(assets),
                        },
                    ];

                    parent.spawn((
                        fmt_name("option text"),
                        TextBundle::from_sections(sections).with_style(style::options()),
                        Label,
                    ));
                });
        }
    });
}

const DIALOG_WIDTH: f32 = 800.0 * 0.8;
const TEXT_BORDER: f32 = 120.0;

mod style {
    use super::*;
    pub(crate) fn standard() -> Style {
        Style {
            max_width: Val::Px(DIALOG_WIDTH - 2.0 * TEXT_BORDER),
            ..default()
        }
    }
    pub(crate) fn options() -> Style {
        const INDENT_MODIFIER: f32 = 1.0;
        Style {
            margin: UiRect::horizontal(Val::Px((INDENT_MODIFIER - 1.0) * TEXT_BORDER)),
            max_width: Val::Px(DIALOG_WIDTH - 2.0 * INDENT_MODIFIER * TEXT_BORDER),
            ..default()
        }
    }
}

mod text_style {
    use super::*;
    pub(crate) fn standard(assets: &Assets) -> TextStyle {
        TextStyle {
            font: assets.font.clone(),
            font_size: 20.0,
            color: Color::WHITE,
        }
    }
    pub(crate) fn name(assets: &Assets) -> TextStyle {
        TextStyle {
            font_size: 18.0,
            ..standard(assets)
        }
    }

    pub(crate) fn option_id(assets: &Assets) -> TextStyle {
        TextStyle {
            color: Color::ALICE_BLUE,
            ..option_text(assets)
        }
    }

    pub(crate) fn option_text(assets: &Assets) -> TextStyle {
        TextStyle {
            font_size: 18.0,
            color: Color::TOMATO,
            ..standard(assets)
        }
    }
}

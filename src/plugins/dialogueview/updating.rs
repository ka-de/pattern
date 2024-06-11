use bevy::app::{ App, Update };
use bevy::input::ButtonInput;
use bevy::prelude::{
    on_event,
    Commands,
    Condition,
    Event,
    EventReader,
    EventWriter,
    IntoSystemConfigs,
    KeyCode,
    MouseButton,
    Query,
    Res,
    ResMut,
    Touches,
    With,
    Without,
};
use bevy::reflect::Reflect;
use bevy::render::view::Visibility;
use bevy::text::Text;
use bevy_yarnspinner::{ events::*, prelude::* };

use super::option_selection::OptionSelection;
use super::setup::{ DialogueContinueNode, DialogueNameNode, UiRootNode };
use super::typewriter::{ self, Typewriter, in_dialogue };
use super::YarnSpinnerDialogueViewSystemSet;

pub(crate) fn ui_updating_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            hide_dialog,
            show_dialog.run_if(on_event::<DialogueStartEvent>()),
            present_line.run_if(in_dialogue.and_then(on_event::<PresentLineEvent>())),
            present_options.run_if(on_event::<PresentOptionsEvent>()),
            continue_dialogue.run_if(in_dialogue),
        )
            .chain()
            .after(YarnSpinnerSystemSet)
            .after(typewriter::spawn)
            .in_set(YarnSpinnerDialogueViewSystemSet)
    )
        .add_event::<SpeakerChangeEvent>()
        .register_type::<SpeakerChangeEvent>();
}

/// Signals that a speaker has changed.
/// A speaker starts speaking when a new line is presented with a [`PresentLineEvent`] which has a character name.
/// A speaker stops speaking when the line is fully displayed on the screen, which happens over the course of a few seconds
#[derive(Debug, Eq, PartialEq, Hash, Reflect, Event)]
#[reflect(Debug, PartialEq, Hash)]
#[non_exhaustive]
pub struct SpeakerChangeEvent {
    /// The name of the character who is or was speaking.
    pub character_name: String,
    /// If `true`, the character just started speaking. Otherwise, they just stopped.
    pub speaking: bool,
}

fn show_dialog(mut visibility: Query<&mut Visibility, With<UiRootNode>>) {
    *visibility.single_mut() = Visibility::Inherited;
}

fn hide_dialog(
    mut root_visibility: Query<&mut Visibility, With<UiRootNode>>,
    mut dialogue_complete_events: EventReader<DialogueCompleteEvent>
) {
    if !dialogue_complete_events.is_empty() {
        *root_visibility.single_mut() = Visibility::Hidden;
        dialogue_complete_events.clear();
    }
}

fn present_line(
    mut line_events: EventReader<PresentLineEvent>,
    mut speaker_change_events: EventWriter<SpeakerChangeEvent>,
    mut typewriter: ResMut<Typewriter>,
    mut name_node: Query<&mut Text, With<DialogueNameNode>>
) {
    for event in line_events.read() {
        let name = if let Some(name) = event.line.character_name() {
            speaker_change_events.send(SpeakerChangeEvent {
                character_name: name.to_string(),
                speaking: true,
            });
            name.to_string()
        } else {
            String::new()
        };
        name_node.single_mut().sections[0].value = name;
        typewriter.set_line(&event.line);
    }
}

fn present_options(mut commands: Commands, mut events: EventReader<PresentOptionsEvent>) {
    for event in events.read() {
        let option_selection = OptionSelection::from_option_set(&event.options);
        commands.insert_resource(option_selection);
    }
}

fn continue_dialogue(
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut dialogue_runners: Query<&mut DialogueRunner>,
    mut typewriter: ResMut<Typewriter>,
    option_selection: Option<Res<OptionSelection>>,
    mut root_visibility: Query<&mut Visibility, With<UiRootNode>>,
    mut continue_visibility: Query<
        &mut Visibility,
        (With<DialogueContinueNode>, Without<UiRootNode>)
    >
) {
    let explicit_continue =
        keys.just_pressed(KeyCode::Space) ||
        keys.just_pressed(KeyCode::Enter) ||
        mouse_buttons.just_pressed(MouseButton::Left) ||
        touches.any_just_pressed();
    if explicit_continue && !typewriter.is_finished() {
        typewriter.fast_forward();
        return;
    }
    if (explicit_continue || typewriter.last_before_options) && option_selection.is_none() {
        for mut dialogue_runner in dialogue_runners.iter_mut() {
            if !dialogue_runner.is_waiting_for_option_selection() && dialogue_runner.is_running() {
                dialogue_runner.continue_in_next_update();
                *root_visibility.single_mut() = Visibility::Hidden;
                *continue_visibility.single_mut() = Visibility::Hidden;
            }
        }
    }
}

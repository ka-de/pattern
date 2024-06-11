use bevy::app::{ App, Update };
use bevy::log::info;
use bevy::prelude::{
    any_with_component,
    on_event,
    Commands,
    Event,
    EventReader,
    EventWriter,
    IntoSystemConfigs,
    Local,
    Query,
    Res,
    ResMut,
    Resource,
    With,
};
use bevy::reflect::Reflect;
use bevy::render::view::Visibility;
use bevy::text::Text;
use bevy::time::Time;
use bevy::ui::{ Style, Val };
use bevy::utils::{ default, Instant };
use bevy_yarnspinner::{ events::*, prelude::* };
use unicode_segmentation::UnicodeSegmentation;

use super::option_selection::OptionSelection;
use super::setup::{
    create_dialog_text,
    DialogueContinueNode,
    DialogueNode,
    UiRootNode,
    INITIAL_DIALOGUE_CONTINUE_BOTTOM,
};
use super::updating::SpeakerChangeEvent;
use super::YarnSpinnerDialogueViewSystemSet;

pub(crate) fn typewriter_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            send_finished_event.run_if(in_dialogue),
            despawn.run_if(on_event::<DialogueCompleteEvent>()),
            spawn.run_if(on_event::<DialogueStartEvent>()),
            write_text.run_if(in_dialogue),
            show_continue.run_if(in_dialogue),
            bob_continue.run_if(any_with_component::<UiRootNode>),
        )
            .chain()
            .after(YarnSpinnerSystemSet)
            .in_set(YarnSpinnerDialogueViewSystemSet)
    ).add_event::<TypewriterFinishedEvent>();
}

pub(crate) fn in_dialogue(res: Option<Res<Typewriter>>) -> bool {
    res.is_some()
}

pub(crate) fn not_in_dialogue(res: Option<Res<Typewriter>>) -> bool {
    res.is_none()
}

#[derive(Debug, Eq, PartialEq, Hash, Reflect, Event)]
pub(crate) struct TypewriterFinishedEvent;

#[derive(Debug, Clone, PartialEq, Resource)]
pub(crate) struct Typewriter {
    pub(crate) character_name: Option<String>,
    pub(crate) current_text: String,
    pub(crate) graphemes_left: Vec<String>,
    pub(crate) last_before_options: bool,
    elapsed: f32,
    start: Instant,
    fast_typing: bool,
}

impl Default for Typewriter {
    fn default() -> Self {
        Self {
            character_name: default(),
            current_text: default(),
            graphemes_left: default(),
            last_before_options: default(),
            elapsed: default(),
            start: Instant::now(),
            fast_typing: default(),
        }
    }
}

impl Typewriter {
    pub(crate) fn set_line(&mut self, line: &LocalizedLine) {
        *self = Self {
            character_name: line.character_name().map(|s| s.to_string()),
            current_text: String::new(),
            graphemes_left: line
                .text_without_character_name()
                .graphemes(true)
                .map(|s| s.to_string())
                .collect(),
            last_before_options: line.is_last_line_before_options(),
            ..default()
        };
    }

    pub(crate) fn is_finished(&self) -> bool {
        self.graphemes_left.is_empty() && !self.current_text.is_empty()
    }

    pub(crate) fn fast_forward(&mut self) {
        self.fast_typing = true;
    }

    fn update_current_text(&mut self) {
        if self.is_finished() {
            return;
        }
        self.elapsed += self.start.elapsed().as_secs_f32();
        self.start = Instant::now();
        let calculated_graphemes = (self.graphemes_per_second() * self.elapsed).floor() as usize;
        let graphemes_left = self.graphemes_left.len();
        let grapheme_length_to_take = calculated_graphemes.min(graphemes_left);
        self.elapsed -= (grapheme_length_to_take as f32) / self.graphemes_per_second();
        let graphemes_to_take = self.graphemes_left.drain(..grapheme_length_to_take);
        self.current_text.extend(graphemes_to_take);
    }

    fn graphemes_per_second(&self) -> f32 {
        if self.fast_typing { 120.0 } else { 40.0 }
    }
}

fn write_text(
    mut text: Query<&mut Text, With<DialogueNode>>,
    mut typewriter: ResMut<Typewriter>,
    option_selection: Option<Res<OptionSelection>>,
    mut speaker_change_events: EventWriter<SpeakerChangeEvent>,
    mut root_visibility: Query<&mut Visibility, With<UiRootNode>>,
    assets: Res<super::Assets>
) {
    let mut text = text.single_mut();
    if typewriter.last_before_options && option_selection.is_none() {
        *text = default();
        return;
    }
    if typewriter.is_finished() {
        return;
    }
    if !typewriter.last_before_options {
        *root_visibility.single_mut() = Visibility::Inherited;
        // If this is last before options, the `OptionSelection` will make the visibility inherited as soon as it's ready instead
    }
    typewriter.update_current_text();
    if typewriter.is_finished() {
        if let Some(name) = typewriter.character_name.as_deref() {
            speaker_change_events.send(SpeakerChangeEvent {
                character_name: name.to_string(),
                speaking: false,
            });
        }
    }

    let current_text = &typewriter.current_text;
    let rest = typewriter.graphemes_left.join("");
    *text = create_dialog_text(current_text, rest, &assets);
}

fn show_continue(
    typewriter: Res<Typewriter>,
    mut visibility: Query<&mut Visibility, With<DialogueContinueNode>>,
    mut typewriter_finished_event: EventReader<TypewriterFinishedEvent>
) {
    for _event in typewriter_finished_event.read() {
        if !typewriter.last_before_options {
            let mut visibility = visibility.single_mut();
            *visibility = Visibility::Inherited;
        }
    }
}

pub(crate) fn despawn(mut commands: Commands) {
    info!("Despawning dialogue!");
    commands.remove_resource::<Typewriter>();
}

pub(crate) fn spawn(mut commands: Commands) {
    commands.init_resource::<Typewriter>();
}

fn bob_continue(
    time: Res<Time>,
    visibility: Query<&Visibility, With<DialogueContinueNode>>,
    mut style: Query<&mut Style, With<DialogueContinueNode>>
) {
    let visibility = visibility.single();
    if *visibility == Visibility::Hidden {
        return;
    }
    let mut style = style.single_mut();
    let pixels =
        (time.elapsed_seconds() * 3.0).sin().powi(2) * 5.0 + INITIAL_DIALOGUE_CONTINUE_BOTTOM;
    style.bottom = Val::Px(pixels);
}

fn send_finished_event(
    mut events: EventWriter<TypewriterFinishedEvent>,
    typewriter: Res<Typewriter>,
    mut last_finished: Local<bool>
) {
    if !typewriter.is_finished() {
        *last_finished = false;
    } else if !*last_finished {
        events.send(TypewriterFinishedEvent);
        *last_finished = true;
    }
}

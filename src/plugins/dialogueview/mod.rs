//! A dialogue view for Yarn Spinner.
//!
//! A dialogue view is a plugin that handles presenting lines and options to the user and advances the dialogue on user input.
//!
//! This crate also exposes the [`SpeakerChangeEvent`] which you can use to animate characters while they are speaking,
//! as the text is written out over a few seconds.
//!
//! ## Inputs
//!
//! - Advance the dialogue: press the space bar, enter key, left click or tap the screen after the text is done typing.
//! - Type out the text faster: Same as above, but hold press before the text is done typing.
//! - Select an option: press the number key corresponding to the option you want to select or click/tap the option.
//!
//! ## Limitations
//!
//! This dialogue view expects only a single instance of [`DialogueRunner`](bevy_yarnspinner::prelude::DialogueRunner) to be running.
//! Its behavior is otherwise undefined.

#![allow(clippy::too_many_arguments, clippy::type_complexity)]
#![warn(missing_docs, missing_debug_implementations)]

use bevy::prelude::*;
use bevy_yarnspinner::prelude::{ YarnFileSource, YarnSpinnerPlugin, YarnProject };

pub use updating::SpeakerChangeEvent;
pub(crate) use typewriter::{ in_dialogue, not_in_dialogue };

/// The plugin registering all systems of the dialogue view.
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct YarnSpinnerDialogueViewPlugin;

/// The [`SystemSet`] containing all systems added by the [`YarnSpinnerDialogueViewPlugin`].
/// Is run after the [`YarnSpinnerSystemSet`](bevy_yarnspinner::prelude::YarnSpinnerSystemSet).
#[derive(Debug, Default, Clone, Copy, SystemSet, Eq, PartialEq, Hash)]
pub struct YarnSpinnerDialogueViewSystemSet;

mod assets;
mod option_selection;
mod setup;
mod typewriter;
mod updating;

impl Plugin for YarnSpinnerDialogueViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            YarnSpinnerPlugin::with_yarn_source(YarnFileSource::file("dialogues/test_dialog.yarn"))
        )
            .add_plugins(assets::ui_assets_plugin)
            .add_plugins(setup::ui_setup_plugin)
            .add_plugins(updating::ui_updating_plugin)
            .add_plugins(typewriter::typewriter_plugin)
            .add_plugins(option_selection::option_selection_plugin)
            .add_systems(
                Update,
                // Spawn the dialogue runner once the Yarn project has finished compiling
                spawn_dialogue_runner.run_if(
                    in_state(crate::plugins::gamestate::GameState::Playing).and_then(
                        resource_added::<YarnProject>
                    )
                )
            );
    }
}

fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
    info!("Starting dialogue");
    // Create a dialogue runner from the project.
    let mut dialogue_runner = project.create_dialogue_runner();
    commands.spawn(dialogue_runner);
}

use std::time::Duration;

use bevy::{
    app::{ App, PostUpdate },
    asset::{ Asset, Assets, Handle },
    audio::{ AddAudioSource as _, AudioSource, Decodable, PlaybackMode, PlaybackSettings },
    ecs::{ component::Component, entity::Entity, system::{ Commands, Query, Res, ResMut } },
    prelude::IntoSystemConfigs,
    reflect::TypePath,
    transform::TransformSystem,
    log::warn,
};
use rodio::source::{ Delay, Repeat, Source };

/// An asset type that warps an audio source for adding a delay and infinite
/// repeats (private to this module)
#[derive(TypePath, Asset)]
struct DelayedAudioSource {
    source: AudioSource,
    delay: Duration,
}

impl Decodable for DelayedAudioSource {
    type DecoderItem = <AudioSource as Decodable>::DecoderItem;
    type Decoder = Delay<Repeat<<AudioSource as Decodable>::Decoder>>;
    fn decoder(&self) -> Self::Decoder {
        self.source.decoder().repeat_infinite().delay(self.delay)
    }
}

/// Component that gets converted to an AudioBundle
#[derive(Component)]
pub(super) struct DelayedPlaybackSettings {
    pub source: Handle<AudioSource>,
    pub delay: Duration,
    pub settings: PlaybackSettings,
}

fn convert_delayed_settings_to_audio_bundle(
    mut commands: Commands,
    mut delayed_audio_sources: ResMut<Assets<DelayedAudioSource>>,
    audio_sources: Res<Assets<AudioSource>>,
    added_sources: Query<(Entity, &DelayedPlaybackSettings)>
) {
    for (entity, DelayedPlaybackSettings { source, settings, delay }) in &added_sources {
        let Some(source) = audio_sources.get(source) else {
            continue; // Wait for the asset to be loaded
        };

        // Force PlaybackMode::Once in the PlaybackSettings
        match settings.mode {
            PlaybackMode::Loop => {}
            _ => {
                warn!(
                    "Playback mode in DelayedPlaybackSettings on entity {:?} is not Loop. It will be looped anyway!",
                    entity
                );
            }
        }
        let settings = PlaybackSettings {
            // The source in delayed_handle is already looped
            mode: PlaybackMode::Once,
            ..*settings
        };

        // Convert the AudioSource asset to DelayedAudioSource, then register
        // the asset to get an Handle<DelayedAudioSource>
        let source = source.clone();
        let delayed_handle = delayed_audio_sources.add(DelayedAudioSource {
            source,
            delay: *delay,
        });

        // Replace the DelayedPlaybackSettings component with the
        // AudioBundle-like (Handle<DelayedAudioSource>, PlaybackSettings) so it
        // can be played by bevy_audio systems
        commands
            .entity(entity)
            .remove::<DelayedPlaybackSettings>()
            .insert((delayed_handle, settings));
    }
}

/// Setup an audio source type for DelayedAudioSource
pub(super) fn plugin(app: &mut App) {
    app.add_audio_source::<DelayedAudioSource>().add_systems(
        PostUpdate,
        // bevy_audios's AudioPlaySet (sadly private) runs after
        // TransformSystem::TransformPropagate, so we run before that to ensure
        // that we're before AudioPlaySet
        convert_delayed_settings_to_audio_bundle.before(TransformSystem::TransformPropagate)
    );
}

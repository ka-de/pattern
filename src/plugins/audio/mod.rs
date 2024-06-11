use bevy::{ app::{ App, PreUpdate }, audio::{ DefaultSpatialScale, GlobalVolume, SpatialScale } };

mod insert_audio_components;
mod play_background_music;
pub(crate) mod change_global_volume;

const AUDIO_SCALE: f32 = 1.0 / 25.0;

pub fn plugin(app: &mut App) {
    app.add_systems(PreUpdate, (
        insert_audio_components::insert_spatial_listener,
        insert_audio_components::insert_audio_sources,
    ))
        .insert_resource(GlobalVolume::new(1.0))
        .insert_resource(DefaultSpatialScale(SpatialScale::new_2d(AUDIO_SCALE)));
}

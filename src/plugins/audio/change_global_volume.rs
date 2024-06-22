use bevy::{ audio::Volume, prelude::* };

pub fn change_global_volume(mut volume: ResMut<GlobalVolume>) {
    volume.volume = Volume::new(1.0);
}

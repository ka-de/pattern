use bevy::{ ecs::system::ResMut, audio::{ Volume, GlobalVolume } };

pub fn change_global_volume(mut volume: ResMut<GlobalVolume>) {
    volume.volume = Volume::new(1.0);
}

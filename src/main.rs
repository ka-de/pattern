mod components;

use components::{Cat, CustomPerfUiAppExt as _, CustomSystemsAppExt as _, Dog};

use bevy::log::LogPlugin;
use bevy::prelude::*;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((Camera2dBundle::default(), components::MainCamera));

    // The 💀 zone.
    commands.spawn(components::death_zone_bundle());

    // Tiles
    for x in -5..5 {
        let tile_position = Vec2::new(x as f32 * 32.0, -100.0);
        commands.spawn(components::tile_bundle(tile_position));
    }

    // 🐈‍⬛
    components::spawn_cat(&mut commands, &asset_server, texture_atlas_layouts.as_mut());

    // 🐕
    components::spawn_dog(&mut commands, &asset_server, texture_atlas_layouts.as_mut());
    components::spawn_dog(&mut commands, &asset_server, texture_atlas_layouts.as_mut());
}

fn main() {
    // this code is compiled only if debug assertions are enabled (debug mode)
    #[cfg(debug_assertions)]
    let log_plugin = LogPlugin {
        level: bevy::log::Level::DEBUG,
        filter: "info,pattern=debug".into(),
        update_subscriber: None,
    };

    // this code is compiled only if debug assertions are disabled (release mode)
    #[cfg(not(debug_assertions))]
    let logplugin = LogPlugin {
        level: bevy::log::Level::INFO,
        filter: "warning,pattern=info".into(),
    };

    // The ImagePlugin::default_nearest() prevents blurry sprites
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(log_plugin),
        )
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_custom_perf_ui()
        .add_animal_perf_ui::<Cat>()
        .add_animal_perf_ui::<Dog>()
        .add_systems(Startup, setup)
        .add_custom_systems()
        .run();
}

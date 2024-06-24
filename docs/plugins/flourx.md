# `flurx`

---

## Spawn and Respawn `flurx` Reactor

---

```rust
//! This example shows how to toggle [`Reactor`] processing.
//!
//! When you press [`KeyCode::Escape`], the box stops rotating.
//! When you press [`KeyCode::Enter`], the box starts rotating again.
//!
//! [`Reactor`]: bevy_flurx::prelude::Reactor

use bevy::DefaultPlugins;
use bevy::prelude::*;

use bevy_flurx::prelude::*;

#[derive(Component)]
struct RotateBox;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FlurxPlugin))
        .add_systems(Startup, (setup_camera_and_box, spawn_reactor))
        .add_systems(Update, toggle)
        .run();
}

fn setup_camera_and_box(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::BLUE,
                ..default()
            }),
            ..default()
        },
        RotateBox,
    ));
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.0,
            range: 100.0,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 6.0),
        ..default()
    });
}

fn spawn_reactor(mut commands: Commands) {
    commands.spawn(
        Reactor::schedule(|task| async move {
            task.will(Update, wait::until(rotate_shape)).await;
        })
    );
}

fn rotate_shape(mut shape: Query<&mut Transform, With<RotateBox>>, time: Res<Time>) -> bool {
    for mut t in shape.iter_mut() {
        t.rotate_y(time.delta_seconds());
    }
    false
}

fn toggle(
    mut commands: Commands,
    reactor: Query<Entity, With<Reactor>>,
    input: Res<ButtonInput<KeyCode>>
) {
    if input.just_pressed(KeyCode::Escape) {
        if let Ok(entity) = reactor.get_single() {
            info!("Despawning reactor.");
            commands.entity(entity).remove::<Reactor>();
        }
    }

    if input.just_pressed(KeyCode::Enter) {
        if reactor.iter().next().is_none() {
            info!("Spawning reactor.");
            commands.spawn(
                Reactor::schedule(|task| async move {
                    task.will(Update, wait::until(rotate_shape)).await;
                })
            );
        }
    }
}
```

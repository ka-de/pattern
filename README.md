# SEPARATED

---

## Introduction

SEPARATED is a 2D platformer game where you can talk to NPCs.

## Table of Contents

- [SEPARATED](#separated)
  - [Introduction](#introduction)
  - [Table of Contents](#table-of-contents)
  - [Player Inputs ∆](#player-inputs-)
  - [Debugging Keyboard Shortcuts](#debugging-keyboard-shortcuts)
  - [Spawn and Respawn `flurx` Reactor](#spawn-and-respawn-flurx-reactor)
  - [Render Engine Graphs](#render-engine-graphs)
  - [Trace](#trace)
  - [Filter out DX12 spam with PowerShell](#filter-out-dx12-spam-with-powershell)
  - [TODO](#todo)
  - [`filesystem_watcher` and `asset_processor`](#filesystem_watcher-and-asset_processor)
  - [Default RapierConfiguration](#default-rapierconfiguration)
  - [Rust Things 🦀](#rust-things-)
    - [Pedantic linting](#pedantic-linting)
    - [Linting on all packages, treating warnings as errors](#linting-on-all-packages-treating-warnings-as-errors)
    - [Format code](#format-code)
    - [Test without default features](#test-without-default-features)
    - [Test with only the `bevy_ui` features](#test-with-only-the-bevy_ui-features)
    - [Test with all features enabled](#test-with-all-features-enabled)
    - [Test with all features enabled on nightly](#test-with-all-features-enabled-on-nightly)
    - [Generate documentation with all features enabled](#generate-documentation-with-all-features-enabled)

## Player Inputs ∆

| Input        |          KeyCode          |     Gamepad Button/Axis     |
| :----------- | :-----------------------: | :-------------------------: |
| **Run**      |         **Shift**         | Xbox: **X** PS5: **Square** |
| **Interact** |           **E**           |   Xbox: **B** PS5: **◯**    |
| **Attack1**  |           **Q**           |      Xbox/PS5: **L1**       |
| **Jump**     |         **Space**         |   Xbox: **A** PS5: **╳**    |
| **Move**     | **WASD** + **Arrow Keys** |    **Any Axis + D-Pad**     |

## Debugging Keyboard Shortcuts

| Action                         | KeyCode |
| :----------------------------- | :-----: |
| Toggle Physics Wireframes      |   F9    |
| StateInspector (**GameState**) |   F10   |
| WorldInspector                 |   F11   |

## Spawn and Respawn `flurx` Reactor

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

## Render Engine Graphs

---

```pwsh
$env:RENDER_GRAPHS = "true"
cargo run
```

## Trace

---

```pwsh
$env:RUST_LOG="trace"
cargo run --release --features bevy/trace_tracy
```

## Filter out DX12 spam with PowerShell

---

```pwsh
cargo run 2>&1 | Out-String -Stream | Where-Object { $_ -notmatch "ID3D12Device" -and $_ -notmatch "Live Object at" }
```

With tracing:

```pwsh
 cargo run --features bevy/trace_tracy 2>&1 | Out-String -Stream | Where-Object { $_ -notmatch "ID3D12Device" -and $_ -notmatch "Live Object at" }
 ```

## TODO

---

- **Use WyRand instead of `thread_rng()`**

```rust
fn print_random_value(mut rng: ResMut<GlobalEntropy<WyRand>>) {
    println!("Random value: {}", rng.next_u32());
}

use bevy_rand::WyRand;
use bevy_rand::prelude::{GlobalEntropy, ForkableRng};

#[derive(Component)]
struct Source;

fn setup_source(mut commands: Commands, mut global: ResMut<GlobalEntropy<WyRand>>) {
    commands
        .spawn((
            Source,
            global.fork_rng(),
        ));
}
```

---

```rust
if ( jumping || falling ) {

    if velocity.y.abs() < jumpHangTimeThreshold {
        // Increase acceleration for this duration also.

        // Reduce gravity.
    }
}

// If the player is moving downwards..
if velocity.y < 0 {
    // Increase gravity while falling.
    gravityScale *= fallGravityMultiplier;

    // Cap maximum fall speed, so when falling over large distances,
    // we don't accelerate to insanely high speeds.
}
```

- **`bevy_asepritesheet` + `bevy_ecs_ldtk` integration.**

- **Patrol**

  - Flip sprite when turning around!

- **Movement Improvements**
  - Movement animations.
  - Movement particle effects.
  - Coyote (Grace) Time after falling off a ledge.
    - Maybe needs a raycast in front of the player? Timer needs to start before falling off a ledge.
  - **Jump Improvements**
    - Jumping animations.
    - Jumping particle effects.
    - Wall Jumping
      - ~~Prevent player movement for a short duration during the wall jump.~~ Reduce run force? Maybe a lerp between the wall jump speed and running speed?
    - Air Time
    - Jump Height
      - Increase the player's jump height the longer the jump button is being held down.
    - Clamp maximum falling speed.
    - Coyote Time while jumping and pressing the jump button.
      - There is already some check for being in the air we just need the input part I think.
    - Bonus Air Time
    - Peak Control
    - Fast Fall
      - Increase Player's falling speed after the peak of their jump by adjusting gravity.
- **Game Feel Improvements**

  This is kinda broad but always iterate over every small mechanic towards more fun.

- **AI Stuff** ⚠️ Started work

  - Basic Timer with Action Scheduling
    - Thirst
    - Fatigue

- **Pathfinding** ⚠️ Started work
- Use something to copy `dxil.dll` and `dxcompiler.dll` to Windows builds.
- **YarnSpinner**
  - Begin YarnSpinner integration ✅
  - YarnSpinner+LDTK integration ⚠️ Started work
- **UI**
  - sickle_ui
    - labels ✅
    - keycap/gamepad button switching ⚠️

## `filesystem_watcher` and `asset_processor`

???

## Default RapierConfiguration

```rust
impl RapierConfiguration {
    /// Configures rapier with the specified length unit.
    ///
    /// See the documentation of [`IntegrationParameters::length_unit`] for additional details
    /// on that argument.
    ///
    /// The default gravity is automatically scaled by that length unit.
    pub fn new(length_unit: Real) -> Self {
        Self {
            gravity: Vect::Y * -9.81 * length_unit,
            physics_pipeline_active: true,
            query_pipeline_active: true,
            timestep_mode: TimestepMode::Variable {
                max_dt: 1.0 / 60.0,
                time_scale: 1.0,
                substeps: 1,
            },
            scaled_shape_subdivision: 10,
            force_update_from_transform_changes: false,
        }
    }
}
```

## Rust Things 🦀

---

### Pedantic linting

```bash
cargo clippy -- -W clippy::pedantic
```

### Linting on all packages, treating warnings as errors

```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

This command runs the `clippy` linter on all packages in the workspace, for all targets and features. The `-D warnings` option treats any warnings as errors.

### Format code

```bash
cargo fmt --all
```

This command formats the code in every package using the default formatting rules provided by `rustfmt`.

### Test without default features

```bash
cargo test --no-default-features
```

This command runs tests in the package, but disables the default features.

### Test with only the `bevy_ui` features

```bash
cargo test --no-default-features --features="bevy_ui"
```

This command runs tests with only the `bevy_ui` feature enabled.

### Test with all features enabled

```bash
cargo test --all-features
```

This command runs tests with all features enabled.

### Test with all features enabled on nightly

```bash
cargo +nightly build --all-features
```

This command builds the package with all features enabled using the nightly version of the Rust compiler. This is typically used for generating documentation on docs.rs.

### Generate documentation with all features enabled

```bash
cargo +nightly doc --all-features --no-deps
```

This command generates documentation for the package with all features enabled, without including dependencies, using the nightly version of the Rust compiler.

# SEPARATED

---

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

use bevy_prng::WyRand;
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

- AI Stuff ⚠️ Started work
  - Basic Timer with Action Scheduling
    - Thirst
    - Fatigue
- A* Pathfinding
- Use something to copy `dxil.dll` and `dxcompiler.dll` to Windows builds.
- begin YarnSpinner integration ✅
- YarnSpinner+LDTK integration ⚠️ Started work
- sickle_ui
  - labels ✅
  - keycap/gamepad button switching ⚠️

## Debugging Keyboard Shortcuts

| Action                         | KeyCode |
| :----------------------------- | :-----: |
| StateInspector (**GameState**) |   F10   |
| WorldInspector                 |   F11   |

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

```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

This command runs the `clippy` linter on all packages in the workspace, for all targets and features. The `-D warnings` option treats any warnings as errors.

```bash
cargo fmt --all
```

This command formats the code in every package using the default formatting rules provided by `rustfmt`.

```bash
cargo test --no-default-features
```

This command runs tests in the package, but disables the default features.

```bash
cargo test --no-default-features --features="bevy_ui"
```

This command runs tests with only the `bevy_ui` feature enabled.

```bash
cargo test --all-features
```

This command runs tests with all features enabled.

```bash
cargo +nightly build --all-features
```

This command builds the package with all features enabled using the nightly version of the Rust compiler. This is typically used for generating documentation on docs.rs.

```bash
cargo +nightly doc --all-features --no-deps
```

This command generates documentation for the package with all features enabled, without including dependencies, using the nightly version of the Rust compiler.

## A Mouthful of Errors

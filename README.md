# PATTERN OF DEATH

---

## `filesystem_watcher` and `asset_processor`

???

## wasm-server-runner

### Current Progress

⚠️ BAD ⚠️

```r
ERROR app: C:\Users\kade\.cargo\registry\src\index.crates.io-6f17d22bba15001f\bevy_asset-0.13.2\src\io\wasm.rs:124 Reading directories is not supported with the HttpWasmAssetReader
 INFO app: C:\Users\kade\.cargo\git\checkouts\bevy_asset_loader-3fb1a3d48c4110df\a1f2aa1\bevy_asset_loader\src\loading_state\systems.rs:142 Loading state 'bevy_splashscreen::state::splash_screen_state::SplashScreenState::Initialize' is done
 INFO app: src\components\gamestate.rs:33 Set GameState: Playing
```

### Set Custom HTML

Needed so the thing can find `#pattern-canvas`.

```bash
set WASM_SERVER_RUNNER_CUSTOM_INDEX_HTML=C:\Users\kade\Desktop\rust\pattern\pattern.html
```

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

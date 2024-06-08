# SEPARATED

---

## Filter out DX12 spam with PowerShell

```pwsh
cargo run 2>&1 | Out-String -Stream | Where-Object { $_ -notmatch "ID3D12Device::CreateCommittedResource:" -and $_ -notmatch "Live Object at" }
```

## TODO

- Use something to copy `dxil.dll` and `dxcompiler.dll` to Windows builds.
- begin YarnSpinner integration
- YarnSpinner+LDTK integration
- sickle_ui

## Debugging Keyboard Shortcuts

| Action                         | KeyCode |
| :----------------------------- | :-----: | ------------- |
| StateInspector (**GameState**) |   F10   |
| WorldInspector                 |   F11   |
| PerformanceUI                  |   F12   | <!-- TODO --> |

## `filesystem_watcher` and `asset_processor`

???

### Set Custom HTML

Needed so the thing can find `#separated-canvas`.

```bash
set WASM_SERVER_RUNNER_CUSTOM_INDEX_HTML=C:\Users\kade\Desktop\rust\separated\separated.html
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

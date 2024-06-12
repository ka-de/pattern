# SEPARATED

---

## Filter out DX12 spam with PowerShell

```pwsh
cargo run 2>&1 | Out-String -Stream | Where-Object { $_ -notmatch "ID3D12Device::CreateCommittedResource:" -and $_ -notmatch "Live Object at" }
```

## TODO

- Use something to copy `dxil.dll` and `dxcompiler.dll` to Windows builds.
- ~~begin YarnSpinner integration~~
- YarnSpinner+LDTK integration
- sickle_ui

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

### Glitched state Interaction Crash

```r
2024-06-12T21:20:43.981606Z  INFO separated::components::interactions: New interactive Npc: Just an NPC
2024-06-12T21:20:43.981741Z  INFO separated::components::interactions: New interactive Npc: Bob Marley
2024-06-12T21:20:43.981811Z  INFO separated::components::interactions: New interactive NpcPatrol: Jeff
thread 'Compute Task Pool (3)' panicked at src\components\interactions.rs:138:18:
called `Result::unwrap()` on an `Err` value: NoSuchEntity(4772v42)
stack backtrace:
   0: std::panicking::begin_panic_handler
             at /rustc/b5b13568fb5da4ac988bde370008d6134d3dfe6c/library\std\src\panicking.rs:652
   1: core::panicking::panic_fmt
             at /rustc/b5b13568fb5da4ac988bde370008d6134d3dfe6c/library\core\src\panicking.rs:72
   2: core::result::unwrap_failed
             at /rustc/b5b13568fb5da4ac988bde370008d6134d3dfe6c/library\core\src\result.rs:1679
   3: enum2$<core::result::Result<tuple$<ref$<bevy_transform::components::global_transform::GlobalTransform>,ref$<bevy_rapier2d::geometry::collider::Collider> >,enum2$<bevy_ecs::query::error::QueryEntityError> > >::unwrap
             at /rustc/b5b13568fb5da4ac988bde370008d6134d3dfe6c\library\core\src\result.rs:1102
   4: separated::components::interactions::update_interactions
             at .\src\components\interactions.rs:136
   5: core::ops::function::FnMut::call_mut
             at /rustc/b5b13568fb5da4ac988bde370008d6134d3dfe6c\library\core\src\ops\function.rs:166
   6: core::ops::function::impls::impl$3::call_mut
             at /rustc/b5b13568fb5da4ac988bde370008d6134d3dfe6c\library\core\src\ops\function.rs:294
   7: bevy_ecs::system::function_system::impl$15::run::call_inner
             at C:\Users\kade\.cargo\registry\src\index.crates.io-6f17d22bba15001f\bevy_ecs-0.13.2\src\system\function_system.rs:656
   8: bevy_ecs::system::function_system::impl$15::run
             at C:\Users\kade\.cargo\registry\src\index.crates.io-6f17d22bba15001f\bevy_ecs-0.13.2\src\system\function_system.rs:659
   9: bevy_ecs::system::function_system::impl$6::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<separated::components::interactions::InteractionSensor>,ref$<bevy_hierarchy::components::parent::Parent> >,tuple$<> >,bevy_ecs::system::query::Que
             at C:\Users\kade\.cargo\registry\src\index.crates.io-6f17d22bba15001f\bevy_ecs-0.13.2\src\system\function_system.rs:499
  10: core::panic::unwind_safe::impl$28::poll
             at /rustc/b5b13568fb5da4ac988bde370008d6134d3dfe6c\library\core\src\panic\unwind_safe.rs:297
  11: futures_lite::future::impl$9::poll::closure$0
             at C:\Users\kade\.cargo\registry\src\index.crates.io-6f17d22bba15001f\futures-lite-2.3.0\src\future.rs:588
  12: core::panic::unwind_safe::impl$25::call_once
             at /rustc/b5b13568fb5da4ac988bde370008d6134d3dfe6c\library\core\src\panic\unwind_safe.rs:272
  13: std::panicking::try::do_call
             at /rustc/b5b13568fb5da4ac988bde370008d6134d3dfe6c\library\std\src\panicking.rs:559
  14: std::panicking::try
             at /rustc/b5b13568fb5da4ac988bde370008d6134d3dfe6c\library\std\src\panicking.rs:523
  15: std::panic::catch_unwind
             at /rustc/b5b13568fb5da4ac988bde370008d6134d3dfe6c\library\std\src\panic.rs:149
  16: futures_lite::future::impl$9::poll
             at C:\Users\kade\.cargo\registry\src\index.crates.io-6f17d22bba15001f\futures-lite-2.3.0\src\future.rs:588
  17: async_executor::impl$5::spawn_inner::async_block$0<enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<
             at C:\Users\kade\.cargo\registry\src\index.crates.io-6f17d22bba15001f\async-executor-1.12.0\src\lib.rs:249
  18: async_task::raw::impl$3::run::closure$1
             at C:\Users\kade\.cargo\registry\src\index.crates.io-6f17d22bba15001f\async-task-4.7.1\src\raw.rs:550
  19: core::ops::function::FnOnce::call_once
             at /rustc/b5b13568fb5da4ac988bde370008d6134d3dfe6c\library\core\src\ops\function.rs:250
  20: core::panic::unwind_safe::impl$25::call_once
             at /rustc/b5b13568fb5da4ac988bde370008d6134d3dfe6c\library\core\src\panic\unwind_safe.rs:272
  21: std::panicking::try::do_call
             at /rustc/b5b13568fb5da4ac988bde370008d6134d3dfe6c\library\std\src\panicking.rs:559
  22: std::panicking::try
             at /rustc/b5b13568fb5da4ac988bde370008d6134d3dfe6c\library\std\src\panicking.rs:523
  23: std::panic::catch_unwind
             at /rustc/b5b13568fb5da4ac988bde370008d6134d3dfe6c\library\std\src\panic.rs:149
  24: async_task::raw::RawTask<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<tuple$<>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::u
             at C:\Users\kade\.cargo\registry\src\index.crates.io-6f17d22bba15001f\async-task-4.7.1\src\raw.rs:549
  25: bevy_tasks::task_pool::impl$2::new_internal::closure$0::closure$0::closure$0::closure$0
             at C:\Users\kade\.cargo\registry\src\index.crates.io-6f17d22bba15001f\bevy_tasks-0.13.2\src\task_pool.rs:180
  26: std::panicking::try::do_call
             at /rustc/b5b13568fb5da4ac988bde370008d6134d3dfe6c\library\std\src\panicking.rs:559
  27: std::panicking::try
             at /rustc/b5b13568fb5da4ac988bde370008d6134d3dfe6c\library\std\src\panicking.rs:523
  28: std::panic::catch_unwind
             at /rustc/b5b13568fb5da4ac988bde370008d6134d3dfe6c\library\std\src\panic.rs:149
  29: bevy_tasks::task_pool::impl$2::new_internal::closure$0::closure$0::closure$0
             at C:\Users\kade\.cargo\registry\src\index.crates.io-6f17d22bba15001f\bevy_tasks-0.13.2\src\task_pool.rs:174
  30: std::thread::local::LocalKey<async_executor::LocalExecutor>::try_with
             at /rustc/b5b13568fb5da4ac988bde370008d6134d3dfe6c\library\std\src\thread\local.rs:283
  31: std::thread::local::LocalKey<async_executor::LocalExecutor>::with
             at /rustc/b5b13568fb5da4ac988bde370008d6134d3dfe6c\library\std\src\thread\local.rs:260
  32: bevy_tasks::task_pool::impl$2::new_internal::closure$0::closure$0
             at C:\Users\kade\.cargo\registry\src\index.crates.io-6f17d22bba15001f\bevy_tasks-0.13.2\src\task_pool.rs:167
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
Encountered a panic in system `separated::components::interactions::update_interactions`!
Encountered a panic in system `bevy_app::main_schedule::Main::run_main`!
```

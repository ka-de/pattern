# Compilation Options

---

## Render Engine Graphs

---

You can render graphs of the engine if you set the `RENDER_GRAPHS` environment variable `true`. By default it will output `.dot`, `.svg` and `.png` formats for each possible graphs in the `./docs/graph` folder, but it takes a long time so only do this if you like looking at game engines naked!

Bash:

```bash
RENDER_GRAPHS=true cargo run
```

PowerShell:

```pwsh
$env:RENDER_GRAPHS = "true"
cargo run
```

## Tracing

---

Bash:

```bash
$env:RUST_LOG="trace" && cargo run --release --features "bevy/trace_tracy"
```

PowerShell:

```pwsh
$env:RUST_LOG="trace"
cargo run --release --features bevy/trace_tracy
```

## Filter out DX12 spam with PowerShell

---

PowerShell:

```pwsh
cargo run 2>&1 | Out-String -Stream | Where-Object { $_ -notmatch "ID3D12Device" -and $_ -notmatch "Live Object at" }
```

With tracing:

```pwsh
 cargo run --features bevy/trace_tracy 2>&1 | Out-String -Stream | Where-Object { $_ -notmatch "ID3D12Device" -and $_ -notmatch "Live Object at" }
 ```

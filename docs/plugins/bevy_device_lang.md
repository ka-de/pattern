# `bevy_device_lang`

---

Provides access device language cross-platform: iOS, Android, Web (Wasm), Windows & Linux. Useful to support app localization in the right language.

Example usages:

```rust
fn bevy_system() {
    let lang : Option<String> = bevy_device_lang::get_lang();
    // ..
}

fn get_device_language() {
    info!("Device language is {:?}", bevy_device_lang::get_lang());
}

    add_systems(Startup, get_device_language)
```

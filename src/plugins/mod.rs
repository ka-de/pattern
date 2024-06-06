pub(crate) mod splashscreen;
pub(crate) mod dialogueview;

#[cfg(debug_assertions)]
pub(crate) mod debug;

/// this code is compiled only if debug assertions are disabled (release mode)
#[cfg(not(debug_assertions))]
pub(crate) mod debug {
    pub fn plugin(_app: &mut bevy::app::App) {}

    pub(crate) fn make_log_plugin() -> impl bevy::app::Plugin {
        bevy::log::LogPlugin {
            level: bevy::log::Level::INFO,
            filter: "warning,pattern=info".into(),
            update_subscriber: None,
        }
    }
}

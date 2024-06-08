use bevy::window::WindowLevel;

// GameWindowLevel
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameWindowLevel {
    Normal,
    AlwaysOnTop,
}

impl From<GameWindowLevel> for WindowLevel {
    fn from(level: GameWindowLevel) -> Self {
        match level {
            GameWindowLevel::Normal => WindowLevel::Normal,
            GameWindowLevel::AlwaysOnTop => WindowLevel::AlwaysOnTop,
        }
    }
}

// GameSettings
#[derive(Resource)]
pub struct GameSettings {
    pub window_level: GameWindowLevel,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            window_level: GameWindowLevel::Normal,
        }
    }
}

pub fn update_window_level(
    settings: Res<GameSettings>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>
) {
    if settings.is_changed() {
        if let Ok(mut window) = primary_window.get_single_mut() {
            window.window_level = settings.window_level.into();
        }
    }
}

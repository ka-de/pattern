use bevy::prelude::*;
use bevy::window::{ WindowLevel, PresentMode };

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

// GameVsyncMode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameVsyncMode {
    AutoNoVsync,
    AutoVsync,
}

impl From<GameVsyncMode> for PresentMode {
    fn from(mode: GameVsyncMode) -> Self {
        match mode {
            GameVsyncMode::AutoNoVsync => PresentMode::AutoNoVsync,
            GameVsyncMode::AutoVsync => PresentMode::AutoVsync,
        }
    }
}

// GameSettings
#[derive(Resource)]
pub struct GameSettings {
    pub window_level: GameWindowLevel,
    pub vsync_mode: GameVsyncMode,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            window_level: GameWindowLevel::Normal,
            vsync_mode: GameVsyncMode::AutoVsync,
        }
    }
}

pub fn update_window_settings(
    settings: Res<GameSettings>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>
) {
    if settings.is_changed() {
        if let Ok(mut window) = primary_window.get_single_mut() {
            window.window_level = settings.window_level.into();
            window.present_mode = settings.vsync_mode.into();
        }
    }
}

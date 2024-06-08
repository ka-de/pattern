use bevy::ecs::change_detection::DetectChanges;
use bevy::window::{ WindowLevel, PresentMode, PrimaryWindow, WindowMode };
use bevy::prelude::{ Resource, Res, Query, With, Window };

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
    Immediate,
}

impl From<GameVsyncMode> for PresentMode {
    fn from(mode: GameVsyncMode) -> Self {
        match mode {
            GameVsyncMode::AutoNoVsync => PresentMode::AutoNoVsync,
            GameVsyncMode::AutoVsync => PresentMode::AutoVsync,
            GameVsyncMode::Immediate => PresentMode::Immediate,
        }
    }
}

// GameWindowMode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameWindowMode {
    Windowed,
    Fullscreen,
}

impl From<GameWindowMode> for WindowMode {
    fn from(mode: GameWindowMode) -> Self {
        match mode {
            GameWindowMode::Windowed => WindowMode::Windowed,
            GameWindowMode::Fullscreen => WindowMode::Fullscreen,
        }
    }
}

// GameSettings
#[derive(Resource)]
pub struct GameSettings {
    pub window_level: GameWindowLevel,
    pub vsync_mode: GameVsyncMode,
    pub window_mode: GameWindowMode,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            window_level: GameWindowLevel::Normal,
            vsync_mode: GameVsyncMode::AutoVsync,
            window_mode: GameWindowMode::Fullscreen,
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
            window.mode = settings.window_mode.into();
        }
    }
}

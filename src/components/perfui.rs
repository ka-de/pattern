use crate::components::ui::{
    CursorWorldCoordinates,
    SpaceKeyPressCount,
    TimeSinceLastClick,
    TimeSinceLastKeypress,
};

use bevy::ecs::system::lifetimeless::SRes;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use perf_ui::prelude::*;

#[derive(Component)]
struct PerfUiCursorWorldCoordinates {
    label: String,
    color_gradient: ColorGradient,
    digits: u8,
    sort_key: i32,
}

impl Default for PerfUiCursorWorldCoordinates {
    fn default() -> Self {
        PerfUiCursorWorldCoordinates {
            label: String::new(),
            color_gradient: ColorGradient::new_preset_gyr(1.0, 4.0, 8.0).unwrap(),
            digits: 4,
            sort_key: perf_ui::utils::next_sort_key(),
        }
    }
}

impl PerfUiEntry for PerfUiCursorWorldCoordinates {
    type Value = Vec2;
    type SystemParam = SRes<CursorWorldCoordinates>;

    fn label(&self) -> &str {
        if self.label.is_empty() { "Cursor World Coords" } else { &self.label }
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(
        &self,
        coords: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>
    ) -> Option<Self::Value> {
        Some(coords.0)
    }

    fn format_value(&self, value: &Self::Value) -> String {
        let s = format!("{},{}", value.x as i32, value.y as i32);
        s
    }

    fn width_hint(&self) -> usize {
        let w = 9;
        w
    }

    fn value_color(&self, _value: &Self::Value) -> Option<Color> {
        None
    }

    fn value_highlight(&self, _value: &Self::Value) -> bool {
        false
    }
}

#[derive(Component)]
struct PerfUiTimeSinceLastClick {
    label: String,
    display_units: bool,
    threshold_highlight: Option<f32>,
    color_gradient: ColorGradient,
    digits: u8,
    precision: u8,
    sort_key: i32,
}

impl Default for PerfUiTimeSinceLastClick {
    fn default() -> Self {
        PerfUiTimeSinceLastClick {
            label: String::new(),
            display_units: true,
            threshold_highlight: Some(10.0),
            color_gradient: ColorGradient::new_preset_gyr(1.0, 4.0, 8.0).unwrap(),
            digits: 2,
            precision: 3,
            sort_key: perf_ui::utils::next_sort_key(),
        }
    }
}

impl PerfUiEntry for PerfUiTimeSinceLastClick {
    type Value = f64;
    type SystemParam = (SRes<Time>, SRes<TimeSinceLastClick>);

    fn label(&self) -> &str {
        if self.label.is_empty() { "Time since last click" } else { &self.label }
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(
        &self,
        (time, lastclick): &mut <Self::SystemParam as SystemParam>::Item<'_, '_>
    ) -> Option<Self::Value> {
        let d = time.elapsed() - lastclick.last_click;
        Some(d.as_secs_f64())
    }

    fn format_value(&self, value: &Self::Value) -> String {
        let mut s = perf_ui::utils::format_pretty_float(self.digits, self.precision, *value);
        if self.display_units {
            s.push_str(" s");
        }
        s
    }

    fn width_hint(&self) -> usize {
        let w = perf_ui::utils::width_hint_pretty_float(self.digits, self.precision);
        if self.display_units {
            w + 2
        } else {
            w
        }
    }

    fn value_color(&self, value: &Self::Value) -> Option<Color> {
        self.color_gradient.get_color_for_value(*value as f32)
    }

    fn value_highlight(&self, value: &Self::Value) -> bool {
        self.threshold_highlight.map(|t| (*value as f32) > t).unwrap_or(false)
    }
}

#[derive(Component)]
struct PerfUiTimeSinceLastKeypress {
    label: String,
    display_units: bool,
    threshold_highlight: Option<f32>,
    color_gradient: ColorGradient,
    digits: u8,
    precision: u8,
    sort_key: i32,
}

impl Default for PerfUiTimeSinceLastKeypress {
    fn default() -> Self {
        PerfUiTimeSinceLastKeypress {
            label: String::new(),
            display_units: true,
            threshold_highlight: Some(10.0),
            color_gradient: ColorGradient::new_preset_gyr(1.0, 4.0, 8.0).unwrap(),
            digits: 2,
            precision: 3,
            sort_key: perf_ui::utils::next_sort_key(),
        }
    }
}

impl PerfUiEntry for PerfUiTimeSinceLastKeypress {
    type Value = f64;
    type SystemParam = (SRes<Time>, SRes<TimeSinceLastKeypress>);

    fn label(&self) -> &str {
        if self.label.is_empty() { "Time since last key press" } else { &self.label }
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(
        &self,
        (time, lastkeypress): &mut <Self::SystemParam as SystemParam>::Item<'_, '_>
    ) -> Option<Self::Value> {
        let d = time.elapsed() - lastkeypress.last_keypress;
        Some(d.as_secs_f64())
    }

    fn format_value(&self, value: &Self::Value) -> String {
        let mut s = perf_ui::utils::format_pretty_float(self.digits, self.precision, *value);
        if self.display_units {
            s.push_str(" s");
        }
        s
    }

    fn width_hint(&self) -> usize {
        let w = perf_ui::utils::width_hint_pretty_float(self.digits, self.precision);
        if self.display_units {
            w + 2
        } else {
            w
        }
    }

    fn value_color(&self, value: &Self::Value) -> Option<Color> {
        self.color_gradient.get_color_for_value(*value as f32)
    }

    fn value_highlight(&self, value: &Self::Value) -> bool {
        self.threshold_highlight.map(|t| (*value as f32) > t).unwrap_or(false)
    }
}

#[derive(Component)]
struct PerfUiSpaceKeyPressCount {
    label: String,
    sort_key: i32,
}

impl Default for PerfUiSpaceKeyPressCount {
    fn default() -> Self {
        PerfUiSpaceKeyPressCount {
            label: String::new(),
            sort_key: perf_ui::utils::next_sort_key(),
        }
    }
}

impl PerfUiEntry for PerfUiSpaceKeyPressCount {
    type Value = u32;
    type SystemParam = SRes<SpaceKeyPressCount>;

    fn label(&self) -> &str {
        if self.label.is_empty() { "Space key press count" } else { &self.label }
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(
        &self,
        space_key_press_count: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>
    ) -> Option<Self::Value> {
        Some(space_key_press_count.count)
    }

    fn format_value(&self, value: &Self::Value) -> String {
        format!("{}", value)
    }

    fn width_hint(&self) -> usize {
        10
    }
}

fn setup_perfui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        PerfUiRoot {
            font_label: asset_server.load("fonts/bahnschrift.ttf"),
            font_value: asset_server.load("fonts/bahnschrift.ttf"),
            font_highlight: asset_server.load("fonts/bahnschrift.ttf"),
            values_col_width: Some(80.0),
            ..default()
        },
        PerfUiEntryFPS::default(),
        PerfUiCursorWorldCoordinates::default(),
        PerfUiTimeSinceLastClick::default(),
        PerfUiTimeSinceLastKeypress::default(),
        PerfUiSpaceKeyPressCount::default(),
    ));
}

pub trait CustomPerfUiAppExt {
    fn add_custom_perf_ui(&mut self) -> &mut Self;
}

impl CustomPerfUiAppExt for App {
    fn add_custom_perf_ui(&mut self) -> &mut Self {
        self.add_plugins(PerfUiPlugin)
            .add_perf_ui_entry_type::<PerfUiCursorWorldCoordinates>()
            .add_perf_ui_entry_type::<PerfUiTimeSinceLastClick>()
            .add_perf_ui_entry_type::<PerfUiTimeSinceLastKeypress>()
            .add_perf_ui_entry_type::<PerfUiSpaceKeyPressCount>()
            .add_systems(PreStartup, setup_perfui)
    }
}

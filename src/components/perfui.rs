use crate::plugins::input::{ KeyPressState, KeyPressTimers };
use bevy::ecs::system::lifetimeless::SRes;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use perf_ui::prelude::*;
use std::collections::HashMap;

#[derive(Component)]
struct PerfUiKeyPressTimers {
    label: String,
    display_units: bool,
    threshold_highlight: Option<f32>,
    color_gradient: ColorGradient,
    digits: u8,
    precision: u8,
    sort_key: i32,
}

impl Default for PerfUiKeyPressTimers {
    fn default() -> Self {
        PerfUiKeyPressTimers {
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

impl PerfUiEntry for PerfUiKeyPressTimers {
    type Value = HashMap<KeyCode, f64>;
    type SystemParam = SRes<KeyPressTimers>;

    fn label(&self) -> &str {
        if self.label.is_empty() { "Time since last key press" } else { &self.label }
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(
        &self,
        key_press_timers: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>
    ) -> Option<Self::Value> {
        let mut times = HashMap::new();
        for (key_code, timer) in &key_press_timers.timers {
            times.insert(*key_code, timer.elapsed_secs().into());
        }
        Some(times)
    }

    fn format_value(&self, value: &Self::Value) -> String {
        let mut output = String::new();
        for (key_code, time) in value {
            let mut time_str = perf_ui::utils::format_pretty_float(
                self.digits,
                self.precision,
                *time
            );
            if self.display_units {
                time_str.push_str(" s");
            }
            output.push_str(&format!("{:?}: {time_str}\n", key_code));
        }
        output
    }

    fn width_hint(&self) -> usize {
        let w = perf_ui::utils::width_hint_pretty_float(self.digits, self.precision);
        if self.display_units {
            w + 2
        } else {
            w
        }
    }

    fn value_color(&self, _value: &Self::Value) -> Option<Color> {
        None
    }

    fn value_highlight(&self, _value: &Self::Value) -> bool {
        false
    }
}

#[derive(Component)]
struct PerfUiKeyPressCounts {
    label: String,
    sort_key: i32,
}

impl Default for PerfUiKeyPressCounts {
    fn default() -> Self {
        PerfUiKeyPressCounts {
            label: String::new(),
            sort_key: perf_ui::utils::next_sort_key(),
        }
    }
}

impl PerfUiEntry for PerfUiKeyPressCounts {
    type Value = HashMap<KeyCode, u32>;
    type SystemParam = SRes<KeyPressState>;

    fn label(&self) -> &str {
        if self.label.is_empty() { "Key press counts" } else { &self.label }
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(
        &self,
        key_press_state: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>
    ) -> Option<Self::Value> {
        Some(key_press_state.counts.clone())
    }

    fn format_value(&self, value: &Self::Value) -> String {
        let mut output = String::new();
        for (key_code, count) in value {
            output.push_str(&format!("{:?}: {count}\n", key_code));
        }
        output
    }

    fn width_hint(&self) -> usize {
        10
    }

    fn value_color(&self, _value: &Self::Value) -> Option<Color> {
        None
    }

    fn value_highlight(&self, _value: &Self::Value) -> bool {
        false
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
        PerfUiKeyPressTimers::default(),
        PerfUiKeyPressCounts::default(),
    ));
}

pub fn setup_perf_ui(app: &mut App) {
    app.add_plugins(PerfUiPlugin)
        .add_perf_ui_entry_type::<PerfUiKeyPressTimers>()
        .add_perf_ui_entry_type::<PerfUiKeyPressCounts>()
        .add_systems(PreStartup, setup_perfui);
}

use bevy::prelude::*;
use bevy::utils::Duration;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::keyboard::KeyCode;
use bevy::input::ButtonState;
use bevy::ecs::system::lifetimeless::SRes;
use bevy::ecs::system::SystemParam;
use iyes_perf_ui::prelude::*;

#[derive(Resource, Default)]
pub struct SpaceKeyPressCount {
    count: u32,
}

#[derive(Resource, Default)]
pub struct TimeSinceLastClick {
    last_click: Duration,
}

#[derive(Resource, Default)]
pub struct TimeSinceLastKeypress {
    last_keypress: Duration,
}

#[derive(Component)]
pub struct PerfUiTimeSinceLastClick {
    pub label: String,
    pub display_units: bool,
    pub threshold_highlight: Option<f32>,
    pub color_gradient: ColorGradient,
    pub digits: u8,
    pub precision: u8,
    pub sort_key: i32,
}

#[derive(Component)]
pub struct PerfUiTimeSinceLastKeypress {
    pub label: String,
    pub display_units: bool,
    pub threshold_highlight: Option<f32>,
    pub color_gradient: ColorGradient,
    pub digits: u8,
    pub precision: u8,
    pub sort_key: i32,
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
            sort_key: iyes_perf_ui::utils::next_sort_key(),
        }
    }
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
            sort_key: iyes_perf_ui::utils::next_sort_key(),
        }
    }
}

#[derive(Component)]
pub struct PerfUiSpaceKeyPressCount {
    pub label: String,
    pub sort_key: i32,
}

impl Default for PerfUiSpaceKeyPressCount {
    fn default() -> Self {
        PerfUiSpaceKeyPressCount {
            label: String::new(),
            sort_key: iyes_perf_ui::utils::next_sort_key(),
        }
    }
}

impl PerfUiEntry for PerfUiSpaceKeyPressCount {
    type Value = u32;
    type SystemParam = SRes<SpaceKeyPressCount>;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Space key press count"
        } else {
            &self.label
        }
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(
        &self,
        space_key_press_count: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
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

impl PerfUiEntry for PerfUiTimeSinceLastClick {
    type Value = f64;
    type SystemParam = (SRes<Time>, SRes<TimeSinceLastClick>);

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Time since last click"
        } else {
            &self.label
        }
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(
        &self,
        (time, lastclick): &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        let d = time.elapsed() - lastclick.last_click;
        Some(d.as_secs_f64())
    }

    fn format_value(&self, value: &Self::Value) -> String {
        let mut s = iyes_perf_ui::utils::format_pretty_float(self.digits, self.precision, *value);
        if self.display_units {
            s.push_str(" s");
        }
        s
    }

    fn width_hint(&self) -> usize {
        let w = iyes_perf_ui::utils::width_hint_pretty_float(self.digits, self.precision);
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
        self.threshold_highlight
            .map(|t| (*value as f32) > t)
            .unwrap_or(false)
    }
}

impl PerfUiEntry for PerfUiTimeSinceLastKeypress {
    type Value = f64;
    type SystemParam = (SRes<Time>, SRes<TimeSinceLastKeypress>);

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Time since last key press"
        } else {
            &self.label
        }
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(
        &self,
        (time, lastkeypress): &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        let d = time.elapsed() - lastkeypress.last_keypress;
        Some(d.as_secs_f64())
    }

    fn format_value(&self, value: &Self::Value) -> String {
        let mut s = iyes_perf_ui::utils::format_pretty_float(self.digits, self.precision, *value);
        if self.display_units {
            s.push_str(" s");
        }
        s
    }

    fn width_hint(&self) -> usize {
        let w = iyes_perf_ui::utils::width_hint_pretty_float(self.digits, self.precision);
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
        self.threshold_highlight
            .map(|t| (*value as f32) > t)
            .unwrap_or(false)
    }
}

fn handle_click(
    time: Res<Time>,
    mut lastclick: ResMut<TimeSinceLastClick>,
    mut evr_mouse: EventReader<MouseButtonInput>,
) {
    for ev in evr_mouse.read() {
        if ev.state == ButtonState::Pressed {
            lastclick.last_click = time.elapsed();
        }
    }
}

fn handle_space_keypress(
    mut evr_keyboard: EventReader<KeyboardInput>,
    mut space_key_press_count: ResMut<SpaceKeyPressCount>,
) {
    for ev in evr_keyboard.read() {
        if ev.state == ButtonState::Pressed && ev.key_code == KeyCode::Space {
            space_key_press_count.count += 1;
        }
    }
}

fn handle_keypress(
    time: Res<Time>,
    mut lastkeypress: ResMut<TimeSinceLastKeypress>,
    mut evr_keyboard: EventReader<KeyboardInput>,
) {
    for ev in evr_keyboard.read() {
        if ev.state == ButtonState::Pressed {
            lastkeypress.last_keypress = time.elapsed();
        }
    }
}

fn setup(mut commands: Commands, ass: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        PerfUiRoot {
            font_label: ass.load("bahnschrift.ttf"),
            font_value: ass.load("bahnschrift.ttf"),
            font_highlight: ass.load("bahnschrift.ttf"),
            values_col_width: Some(80.0),
            ..default()
        },
        PerfUiEntryFPS::default(),
        PerfUiTimeSinceLastClick::default(),
        PerfUiTimeSinceLastKeypress::default(),
        PerfUiSpaceKeyPressCount::default(),
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
        .add_perf_ui_entry_type::<PerfUiTimeSinceLastClick>()
        .add_perf_ui_entry_type::<PerfUiTimeSinceLastKeypress>()
        .add_perf_ui_entry_type::<PerfUiSpaceKeyPressCount>()
        .init_resource::<TimeSinceLastClick>()
        .init_resource::<TimeSinceLastKeypress>()
        .init_resource::<SpaceKeyPressCount>()
        .add_systems(Startup, setup)
        .add_systems(Update, handle_click)
        .add_systems(Update, handle_keypress)
        .add_systems(Update, handle_space_keypress)
        .run();
}

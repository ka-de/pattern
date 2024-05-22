use std::any::TypeId;
use std::marker::PhantomData;

use crate::components::Health;

use crate::components::animals::{Animal, Cat, Dog};
use crate::components::{
    CursorWorldCoordinates, SpaceKeyPressCount, TimeSinceLastClick, TimeSinceLastKeypress,
};

use bevy::ecs::system::lifetimeless::SRes;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use perf_ui::prelude::*;

#[derive(Component)]
#[allow(dead_code)]
pub struct PerfUiCursorWorldCoordinates {
    pub label: String,
    pub color_gradient: ColorGradient,
    pub digits: u8,
    pub sort_key: i32,
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
        if self.label.is_empty() {
            "Cursor World Coords"
        } else {
            &self.label
        }
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(
        &self,
        coords: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
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

/**
 * PerfUI: Struct for tracking the time elapsed since the last click.
 */
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

/**
 * PerfUI: Implementation for tracking the time elapsed since the last click.
 */
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

/**
 * PerfUI: PerfUiEntry implementation for tracking the time elapsed since the last click.
 */
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
        self.threshold_highlight
            .map(|t| (*value as f32) > t)
            .unwrap_or(false)
    }
}

/**
 *  PerfUI: Struct for tracking the time elapsed since the last key pressed.
 */
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

/**
 * PerfUI: Default implementation for tracking the time elapsed since the last key pressed.
 */
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

/**
 * PerfUI: PerfUIEntry implementation for tracking the time elapsed since the last key pressed.
 */
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
        self.threshold_highlight
            .map(|t| (*value as f32) > t)
            .unwrap_or(false)
    }
}

/**
 * PerfUI: Struct for tracking how many times the Space key has been pressed.
 */
#[derive(Component)]
pub struct PerfUiSpaceKeyPressCount {
    pub label: String,
    pub sort_key: i32,
}

/**
 * PerfUI: Default implementation for tracking how many times the Space key has been pressed.
 */
impl Default for PerfUiSpaceKeyPressCount {
    fn default() -> Self {
        PerfUiSpaceKeyPressCount {
            label: String::new(),
            sort_key: perf_ui::utils::next_sort_key(),
        }
    }
}

/**
 * PerfUI: PerfUiEntry implementation for tracking how many times the Space key has been pressed.
 */
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

#[derive(Component)]
pub struct PerfUiAnimalName<T: Component> {
    pub label: String,
    pub sort_key: i32,
    _marker: PhantomData<T>,
}

impl<T: Component + Animal> Default for PerfUiAnimalName<T> {
    fn default() -> Self {
        PerfUiAnimalName {
            label: String::new(),
            sort_key: perf_ui::utils::next_sort_key(),
            _marker: PhantomData,
        }
    }
}

impl<T: Component + Animal> PerfUiEntry for PerfUiAnimalName<T> {
    type Value = String;
    type SystemParam = Query<'static, 'static, &'static T>;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            if TypeId::of::<T>() == TypeId::of::<Cat>() {
                "Cat Name"
            } else if TypeId::of::<T>() == TypeId::of::<Dog>() {
                "Dog Name"
            } else {
                "Animal Name"
            }
        } else {
            &self.label
        }
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(
        &self,
        animal_query: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        let animal = animal_query.single();
        Some(animal.name().clone())
    }

    fn format_value(&self, value: &Self::Value) -> String {
        value.clone()
    }

    fn width_hint(&self) -> usize {
        20
    }
}

#[derive(Component)]
pub struct PerfUiAnimalHunger<T: Component> {
    pub label: String,
    pub sort_key: i32,
    _marker: PhantomData<T>,
}

impl<T: Component> Default for PerfUiAnimalHunger<T> {
    fn default() -> Self {
        PerfUiAnimalHunger {
            label: String::new(),
            sort_key: perf_ui::utils::next_sort_key(),
            _marker: PhantomData,
        }
    }
}

impl<T: Component> PerfUiEntry for PerfUiAnimalHunger<T> {
    type Value = u32;
    type SystemParam = Query<'static, 'static, &'static Health, With<T>>;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            if TypeId::of::<T>() == TypeId::of::<Cat>() {
                "Cat Hunger"
            } else if TypeId::of::<T>() == TypeId::of::<Dog>() {
                "Dog Hunger"
            } else {
                "Animal Hunger"
            }
        } else {
            &self.label
        }
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(
        &self,
        health_query: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        let health = health_query.single();
        Some(health.hunger)
    }

    fn format_value(&self, value: &Self::Value) -> String {
        format!("{}", value)
    }

    fn width_hint(&self) -> usize {
        3
    }
}

#[derive(Component)]
pub struct PerfUiAnimalHealth<T: Component> {
    pub label: String,
    pub sort_key: i32,
    _marker: PhantomData<T>,
}

impl<T: Component> Default for PerfUiAnimalHealth<T> {
    fn default() -> Self {
        PerfUiAnimalHealth {
            label: String::new(),
            sort_key: perf_ui::utils::next_sort_key(),
            _marker: PhantomData,
        }
    }
}

impl<T: Component> PerfUiEntry for PerfUiAnimalHealth<T> {
    type Value = String;
    type SystemParam = Query<'static, 'static, &'static Health, With<T>>;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            if TypeId::of::<T>() == TypeId::of::<Cat>() {
                "Cat Health"
            } else if TypeId::of::<T>() == TypeId::of::<Dog>() {
                "Dog Health"
            } else {
                "Animal Health"
            }
        } else {
            &self.label
        }
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(
        &self,
        health_query: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        let health = health_query.single();
        Some(format!("{}/{}", health.current, health.max))
    }

    fn format_value(&self, value: &Self::Value) -> String {
        value.clone()
    }

    fn width_hint(&self) -> usize {
        10
    }
}

#[derive(Component)]
pub struct PerfUiAnimalGender<T: Component> {
    pub label: String,
    pub sort_key: i32,
    _marker: PhantomData<T>,
}

impl<T: Component + Animal> Default for PerfUiAnimalGender<T> {
    fn default() -> Self {
        PerfUiAnimalGender {
            label: T::species().to_owned() + " Gender",
            sort_key: perf_ui::utils::next_sort_key(),
            _marker: PhantomData,
        }
    }
}

impl<T: Component + Animal> PerfUiEntry for PerfUiAnimalGender<T> {
    type Value = String;
    type SystemParam = Query<'static, 'static, &'static T>;

    fn label(&self) -> &str {
        &self.label
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(
        &self,
        animal_query: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        let animal = animal_query.single();
        let gender = animal.gender();
        Some(gender.unwrap_or("Unknown").to_string())
    }

    fn format_value(&self, value: &Self::Value) -> String {
        value.clone()
    }

    fn width_hint(&self) -> usize {
        10
    }
}

fn setup_perfui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Performance UI
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

fn setup_animal_perfui<T: Animal + Component>(
    mut commands: Commands,
    mut query: Query<Entity, With<PerfUiRoot>>,
) {
    let perfui = query.single_mut();
    commands.entity(perfui).insert((
        PerfUiAnimalName::<T>::default(),
        PerfUiAnimalGender::<T>::default(),
        PerfUiAnimalHealth::<T>::default(),
        PerfUiAnimalHunger::<T>::default(),
    ));
}

pub(crate) trait CustomPerfUiAppExt {
    /// Add support for a custom perf UI entry type (component).
    fn add_custom_perf_ui(&mut self) -> &mut Self;

    fn add_animal_perf_ui<T: Component + Animal>(&mut self) -> &mut Self;
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

    fn add_animal_perf_ui<T: Component + Animal>(&mut self) -> &mut Self {
        self.add_perf_ui_entry_type::<PerfUiAnimalName<T>>()
            .add_perf_ui_entry_type::<PerfUiAnimalGender<T>>()
            .add_perf_ui_entry_type::<PerfUiAnimalHealth<T>>()
            .add_perf_ui_entry_type::<PerfUiAnimalHunger<T>>()
            .add_systems(Startup, setup_animal_perfui::<T>)
    }
}

use rand::thread_rng;
use rand::seq::SliceRandom;

use bevy::prelude::*;
use bevy::utils::Duration;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::keyboard::KeyCode;
use bevy::input::ButtonState;
use bevy::ecs::system::lifetimeless::SRes;
use bevy::ecs::system::SystemParam;
use bevy::window::PrimaryWindow;
use perf_ui::prelude::*;

fn setup(
    mut commands: Commands, asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

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
        PerfUiCatName::default(), // 🐈‍⬛
        PerfUiCatGender::default(),
        PerfUiCatHealth::default(),
        PerfUiCatHunger::default(),
        PerfUiDogName::default(), // 🐕
        PerfUiDogGender::default(),
        PerfUiDogHealth::default(),
        PerfUiDogHunger::default(),
    ));

    // 🐈‍⬛
    let cat_texture = asset_server.load("textures/cat-texture.png");
    let cat_layout = TextureAtlasLayout::from_grid(Vec2::new(26.0, 26.0), 4, 4, None, None);
    let cat_texture_atlas_layout = texture_atlas_layouts.add(cat_layout);
    let cat_animation_indices = AnimationIndices { first: 0, last: 3, current_index: 0 }; // idle animation
    let _cat_entity = commands.spawn((
        Cat {
            name: generate_animal_name(AnimalType::Cat),
        },
        Health {
            current: 100,
            max: 100,
            hunger: 100,
        },
        SpriteSheetBundle {
            texture: cat_texture.clone(),
            atlas: TextureAtlas {
                layout: cat_texture_atlas_layout,
                index: cat_animation_indices.first,
            },
            transform: Transform::from_xyz(25.0, 50.0, 0.0),
          ..Default::default()
        },
        AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
        cat_animation_indices.clone(),
        Velocity { x: 5.0, y: 0.0 },
        DeathAnimationPlayed(false)
    ));

    // 🐕
    let dog_texture = asset_server.load("textures/dog-texture.png");
    let dog_layout = TextureAtlasLayout::from_grid(Vec2::new(26.0, 26.0), 4, 4, None, None);
    let dog_texture_atlas_layout = texture_atlas_layouts.add(dog_layout);
    let dog_animation_indices = AnimationIndices { first: 0, last: 3, current_index: 0 }; // idle animation
    let _dog_entity = commands.spawn((
        Dog {
            name: generate_animal_name(AnimalType::Dog),
        },
        Health {
            current: 100,
            max: 100,
            hunger: 100,
        },
        SpriteSheetBundle {
            texture: dog_texture.clone(),
            atlas: TextureAtlas {
                layout: dog_texture_atlas_layout,
                index: dog_animation_indices.first,
            },
            transform: Transform::from_xyz(-25.0, 50.0, 0.0),
          ..Default::default()
        },
        AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
        dog_animation_indices.clone(),
        Velocity { x: -2.0, y: 0.0 },
        DeathAnimationPlayed(false)
    ));
}

fn main() {
    App::new()
        // The ImagePlugin::default_nearest() prevents blurry sprites
       .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
       .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
       .add_plugins(PerfUiPlugin)
       .add_perf_ui_entry_type::<PerfUiCursorWorldCoordinates>()
       .add_perf_ui_entry_type::<PerfUiTimeSinceLastClick>()
       .add_perf_ui_entry_type::<PerfUiTimeSinceLastKeypress>()
       .add_perf_ui_entry_type::<PerfUiSpaceKeyPressCount>()
       .add_perf_ui_entry_type::<PerfUiCatName>() // I hate this 🐈‍⬛ already omg
       .add_perf_ui_entry_type::<PerfUiCatGender>()
       .add_perf_ui_entry_type::<PerfUiCatHealth>()
       .add_perf_ui_entry_type::<PerfUiCatHunger>()
       .add_perf_ui_entry_type::<PerfUiDogName>() // Finally the 🐈‍⬛ stuff is over!
       .add_perf_ui_entry_type::<PerfUiDogGender>()
       .add_perf_ui_entry_type::<PerfUiDogHealth>()
       .add_perf_ui_entry_type::<PerfUiDogHunger>()
       .init_resource::<CursorWorldCoordinates>() // End of 🐕
       .init_resource::<TimeSinceLastClick>()
       .init_resource::<TimeSinceLastKeypress>()
       .init_resource::<SpaceKeyPressCount>()
       .init_resource::<SpaceKeyPressState>()
       .init_resource::<HungerTimer>()
       .add_systems(Startup, setup)
       .add_systems(Update, decrease_hunger) // Nyeheh
       .add_systems(Update, cursor_system)
       .add_systems(Update, handle_click)
       .add_systems(Update, handle_keypress)
       .add_systems(Update, handle_space_keypress)
       .add_systems(Update, move_entities)
       .add_systems(Update, update_facing_direction)
       .add_systems(Update, animate_cat_sprite)
       .add_systems(Update, animate_dog_sprite)
       .add_systems(Update, update_animation)
       .add_systems(Update, play_death_animation)
       .run();
}

#[derive(Component, Clone)]
struct AnimationIndices {
    first: usize,
    last: usize,
    current_index: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn update_animation(
    mut query: Query<(&mut AnimationIndices, &Velocity, &Health)>,
) {
    for (mut animation_indices, velocity, health) in query.iter_mut() {
        if health.current > 0 {
            let abs_velocity = velocity.x.abs();
            if abs_velocity < 0.01 {
                // idle animation
                if animation_indices.first != 0 {
                    animation_indices.first = 0;
                    animation_indices.last = 3;
                    animation_indices.current_index = 0;
                }
            } else if abs_velocity < 2.1 {
                // walking animation
                if animation_indices.first != 8 {
                    animation_indices.first = 8;
                    animation_indices.last = 11;
                    animation_indices.current_index = 8;
                }
            } else {
                // running animation
                if animation_indices.first != 12 {
                    animation_indices.first = 12;
                    animation_indices.last = 15;
                    animation_indices.current_index = 12;
                }
            }
        } else {
            // Death animation
            if animation_indices.first != 4 {
                animation_indices.first = 4;
                animation_indices.last = 4;
                animation_indices.current_index = 4;
            }
        }
    }
}

fn update_facing_direction(
    mut query: Query<(&mut Transform, &Velocity)>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        // Flip the sprite based on the direction of movement
        if velocity.x < 0.0 {
            transform.scale.x = transform.scale.x.abs() * -1.0;
        } else {
            transform.scale.x = transform.scale.x.abs();
        }
    }
}

/**
 * ↗️
 */
#[derive(Component, Debug)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct DeathAnimationPlayed(bool);

fn play_death_animation(
    mut query: Query<(
        &mut AnimationIndices,
        &Health,
        &mut DeathAnimationPlayed,
        &mut TextureAtlas,
    )>,
) {
    for (mut animation_indices, health, mut death_animation_played, mut atlas) in query.iter_mut() {
        if health.current == 0 &&!death_animation_played.0 {
            animation_indices.first = 4;
            animation_indices.last = 4;
            animation_indices.current_index = 4;
            atlas.index = animation_indices.current_index; // Update the TextureAtlas index
            death_animation_played.0 = true;
        }
    }
}

fn move_entities(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity, &Health)>,
) {
    for (mut transform, mut velocity, health) in query.iter_mut() {
        if health.current > 0 {
            let delta_seconds = time.delta_seconds();
            transform.translation.x += velocity.x * delta_seconds;
            transform.translation.y += velocity.y * delta_seconds;
        } else {
            velocity.x = 0.0;
            velocity.y = 0.0;
        }
    }
}

/**
 * Hunger 🍗
 */
#[derive(Resource, Default)]
struct HungerTimer(Timer);

fn decrease_hunger(
    time: Res<Time>,
    mut hunger_timer: ResMut<HungerTimer>,
    mut health_query: Query<&mut Health>,
) {
    hunger_timer.0.tick(time.delta());
    if hunger_timer.0.just_finished() {
        for mut health in health_query.iter_mut() {
            // Decrease hunger by 20 every second.
            health.hunger = health.hunger.saturating_sub(20);

            // If hunger reaches 0, decrease health by 20 every second.
            if health.hunger == 0 {
                health.current = health.current.saturating_sub(20);
            }
        }
        // Set the timer's duration to 60 seconds for periodic decrease
        hunger_timer.0.set_duration(Duration::from_secs(1));
        // Reset the timer to count down again.
        hunger_timer.0.reset();
    }
}

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

/**
 * The Health Component 🩸 
 */
#[derive(Component)]
pub struct Health {
    current: u32,
    max: u32,
    hunger: u32,
}

/**
 * Identifies the main camera. 🎥
 */
#[derive(Component)]
struct MainCamera;

/**
 * Stores the world position of the mouse cursor.
 */
#[derive(Resource, Default)]
pub struct CursorWorldCoordinates(Vec2);

/**
 * Function to handle the mouse cursor with world coordinates.
 */
fn cursor_system(
    mut coords: ResMut<CursorWorldCoordinates>,
    // Get the window.
    window_query: Query<&Window, With<PrimaryWindow>>,
    // Get the camera transform.
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // Get the camera info and transform.
    let (camera, camera_transform) = camera_query.single();

    // There is only one primary window, so we can get it from the query.
    let window = window_query.single();

    // Check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z.
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        coords.0 = world_position;
    }
}

#[derive(Component)]
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
 * Function to handle mouse clicks.
 */
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
 * Function to handle key presses.
 */
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
 * Struct for tracking if the Space key is being held.
 */
#[derive(Resource, Default)]
struct SpaceKeyPressState {
    last_pressed: bool,
}

/**
 * Function to handle when the Space key is being pressed.
 */
fn handle_space_keypress(
    mut evr_keyboard: EventReader<KeyboardInput>,
    mut space_key_press_count: ResMut<SpaceKeyPressCount>,
    mut space_key_press_state: ResMut<SpaceKeyPressState>,
) {
    for ev in evr_keyboard.read() {
        if ev.key_code == KeyCode::Space {
            if ev.state == ButtonState::Pressed && !space_key_press_state.last_pressed {
                space_key_press_count.count += 1;
                *space_key_press_state = SpaceKeyPressState { last_pressed: true };
            } else if ev.state == ButtonState::Released {
                *space_key_press_state = SpaceKeyPressState { last_pressed: false };
            }
        }
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

/**
 * A lot of names for the 🐕 and 🐈‍⬛.
 */
const ANIMAL_NAMES: &[(&str, &str, AnimalType)] = &[
    ("Malcolm", "male", AnimalType::Dog), ("Zoe", "female", AnimalType::Dog), ("Wash", "male", AnimalType::Dog),
    ("Inara", "female", AnimalType::Dog), ("Jayne", "male", AnimalType::Dog), ("Kaylee", "female", AnimalType::Dog),
    ("Simon", "male", AnimalType::Dog), ("River", "female", AnimalType::Dog), ("Book", "male", AnimalType::Dog),
    ("Saffron", "female", AnimalType::Dog), ("Badger", "male", AnimalType::Dog), ("Nandi", "female", AnimalType::Dog),
    ("Bester", "male", AnimalType::Dog), ("Dobson", "male", AnimalType::Dog), ("Atherton", "male", AnimalType::Dog),
    ("Gabriel", "male", AnimalType::Dog), ("Regan", "female", AnimalType::Dog), ("Tracey", "male", AnimalType::Dog),
    ("Amnon", "male", AnimalType::Dog), ("Fess", "male", AnimalType::Dog), ("Rance", "male", AnimalType::Dog),
    ("Magistrate", "male", AnimalType::Dog), ("Lucy", "female", AnimalType::Dog), ("Ruth", "female", AnimalType::Dog),
    ("Bree", "female", AnimalType::Dog), // End of 🐕
    ("Picard", "male", AnimalType::Cat), ("Beverly", "female", AnimalType::Cat), ("Data", "male", AnimalType::Cat),
    ("Troi", "female", AnimalType::Cat), ("Laforge", "male", AnimalType::Cat), ("Crusher", "male", AnimalType::Cat),
    ("Yar", "female", AnimalType::Cat), ("Kirk", "male", AnimalType::Cat), ("Spock", "male", AnimalType::Cat),
    ("Mccoy", "male", AnimalType::Cat), ("Scotty", "male", AnimalType::Cat), ("Uhura", "female", AnimalType::Cat),
    ("Sulu", "male", AnimalType::Cat), ("Chekov", "male", AnimalType::Cat), ("Chakotay", "male", AnimalType::Cat),
    ("Tuvok", "male", AnimalType::Cat), ("Sisko", "male", AnimalType::Cat), ("Kira", "female", AnimalType::Cat),
    ("Dax", "female", AnimalType::Cat), ("Bashir", "male", AnimalType::Cat), ("Odo", "male", AnimalType::Cat),
    ("Quark", "male", AnimalType::Cat), ("Archer", "male", AnimalType::Cat), ("Tucker", "male", AnimalType::Cat),
    ("Tpol", "female", AnimalType::Cat), ("Reed", "male", AnimalType::Cat), ("Mayweather", "male", AnimalType::Cat),
    ("Phlox", "male", AnimalType::Cat), ("Sato", "female", AnimalType::Cat), ("Sevenofnine", "female", AnimalType::Cat),
    ("Thedoctor", "male", AnimalType::Cat), ("Tomparis", "male", AnimalType::Cat), ("Harrykim", "male", AnimalType::Cat),
    ("Belanna", "female", AnimalType::Cat), ("Torres", "female", AnimalType::Cat), ("Jeanluc", "male", AnimalType::Cat),
    ("Lorca", "male", AnimalType::Cat), ("Burnham", "female", AnimalType::Cat), ("Saru", "male", AnimalType::Cat),
    ("Stamets", "male", AnimalType::Cat), ("Tilly", "female", AnimalType::Cat), ("Georgiou", "female", AnimalType::Cat), 
    ("Culber", "male", AnimalType::Cat), ("Cornwell", "female", AnimalType::Cat), ("Leland", "male", AnimalType::Cat),
    ("Vance", "male", AnimalType::Cat), ("Reno", "female", AnimalType::Cat), ("Booker", "male", AnimalType::Cat),
    ("Grudge", "female", AnimalType::Cat), ("Shaxs", "male", AnimalType::Cat), ("Detmer", "female", AnimalType::Cat),
    ("Owosekun", "female", AnimalType::Cat), ("Rhys", "male", AnimalType::Cat), ("Pike", "male", AnimalType::Cat),
    ("Number One", "male", AnimalType::Cat), ("Laan", "male", AnimalType::Cat), ("Chapel", "female", AnimalType::Cat), 
    ("Kyle", "male", AnimalType::Cat), ("Vina", "female", AnimalType::Cat), ("Mudd", "male", AnimalType::Cat),
    ("Garak", "male", AnimalType::Cat), ("Leyton", "male", AnimalType::Cat), ("Ross", "male", AnimalType::Cat),
    ("Nog", "male", AnimalType::Cat), ("Jake", "male", AnimalType::Cat), ("Seven", "female", AnimalType::Cat),
    ("Janeway", "female", AnimalType::Cat), ("Tuvix", "male", AnimalType::Cat), ("Neelix", "male", AnimalType::Cat),
    ("Kes", "female", AnimalType::Cat), ("Carey", "male", AnimalType::Cat), ("Vorik", "male", AnimalType::Cat),
    ("Wildman", "female", AnimalType::Cat), ("Zahir", "male", AnimalType::Cat), ("Seska", "female", AnimalType::Cat),
    ("Jonas", "male", AnimalType::Cat), ("Rio", "male", AnimalType::Cat), ("Maxwell", "male", AnimalType::Cat),
    ("Tryla", "female", AnimalType::Cat), ("Lorian", "male", AnimalType::Cat), ("Icheb", "male", AnimalType::Cat), 
    ("Q", "male", AnimalType::Cat), ("Guinan", "female", AnimalType::Cat), ("Pulaski", "female", AnimalType::Cat),
    ("Ro", "female", AnimalType::Cat), ("Hwomyn", "female", AnimalType::Cat), ("Riker", "male", AnimalType::Cat),
    ("Shelby", "female", AnimalType::Cat), ("Obrien", "male", AnimalType::Cat), ("Keiko", "female", AnimalType::Cat),
    ("Molly", "female", AnimalType::Cat), ("Kirayoshi", "male", AnimalType::Cat), ("Naomi", "female", AnimalType::Cat),
    ("Ezri", "female", AnimalType::Cat), ("Kassidy", "female", AnimalType::Cat), ("Leeta", "female", AnimalType::Cat), 
    ("Nog", "male", AnimalType::Cat), ("Rom", "male", AnimalType::Cat), ("Brunt", "male", AnimalType::Cat),
    ("Ishka", "female", AnimalType::Cat), ("Worf", "male", AnimalType::Cat), ("Martok", "male", AnimalType::Cat),
    ("Grilka", "female", AnimalType::Cat), ("Sharan", "male", AnimalType::Cat), ("Alexander", "male", AnimalType::Cat), 
    ("Kehleyr", "female", AnimalType::Cat), ("Lwaxana", "female", AnimalType::Cat), ("Kamala", "female", AnimalType::Cat),
    ("Vash", "female", AnimalType::Cat), ("Tasha", "female", AnimalType::Cat), ("Ogawa", "female", AnimalType::Cat),
    ("Barclay", "male", AnimalType::Cat), ("Maddox", "male", AnimalType::Cat), ("Soong", "male", AnimalType::Cat),
    ("Juliana", "female", AnimalType::Cat), ("Sela", "female", AnimalType::Cat), ("Toral", "male", AnimalType::Cat),
    ("Ziyal", "female", AnimalType::Cat), ("Dukat", "male", AnimalType::Cat), ("Damar", "male", AnimalType::Cat), 
    ("Weyoun", "male", AnimalType::Cat), ("Eddington", "male", AnimalType::Cat), ("Michael", "male", AnimalType::Cat),
    ("Sarina", "female", AnimalType::Cat), ("Hugh", "male", AnimalType::Cat), ("Lore", "male", AnimalType::Cat),
    ("Elaurian", "male", AnimalType::Cat) // End of 🐈‍⬛
];

#[derive(Component, PartialEq, Eq)]
enum AnimalType {
    Dog,
    Cat,
}

fn generate_animal_name(animal_type: AnimalType) -> String {
    let mut rng = thread_rng();
    let (name, _gender, name_type) = ANIMAL_NAMES.choose(&mut rng).unwrap();
    if *name_type == animal_type {
        name.to_string()
    } else {
        generate_animal_name(animal_type)
    }
}

fn get_animal_gender(name: &str) -> Option<&'static str> {
    for &(animal_name, gender, _) in ANIMAL_NAMES {
        if animal_name == name {
            return Some(gender);
        }
    }
    None
}

/**
 * The 🐕 Component
 */
#[derive(Component)]
pub struct Dog {
    name: String,
}

fn animate_dog_sprite(
    time: Res<Time>,
    mut query: Query<(&mut AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (mut indices, mut timer, mut atlas) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            indices.current_index = if indices.current_index == indices.last {
                if indices.first == 4 { // Death animation
                    4 // Loop back to the first frame of the death animation
                } else {
                    indices.first
                }
            } else {
                indices.current_index + 1
            };
            atlas.index = indices.current_index;
        }
    }
}

/**
 * More fucking 🐕 stuff.
 */
 #[derive(Component)]
pub struct PerfUiDogHunger {
    pub label: String,
    pub sort_key: i32,
}

impl Default for PerfUiDogHunger {
    fn default() -> Self {
        PerfUiDogHunger {
            label: String::new(),
            sort_key: perf_ui::utils::next_sort_key(),
        }
    }
}

impl PerfUiEntry for PerfUiDogHunger {
    type Value = u32;
    type SystemParam = Query<'static, 'static, &'static Health, With<Dog>>;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Dog Hunger"
        } else {
            &self.label
        }
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(&self, health_query: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>) -> Option<Self::Value> {
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
pub struct PerfUiDogHealth {
    pub label: String,
    pub sort_key: i32,
}

impl Default for PerfUiDogHealth {
    fn default() -> Self {
        PerfUiDogHealth {
            label: String::new(),
            sort_key: perf_ui::utils::next_sort_key(),
        }
    }
}

impl PerfUiEntry for PerfUiDogHealth {
    type Value = String;
    type SystemParam = Query<'static, 'static, &'static Health, With<Dog>>;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Dog Health"
        } else {
            &self.label
        }
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(&self, health_query: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>) -> Option<Self::Value> {
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
pub struct PerfUiDogName {
    pub label: String,
    pub sort_key: i32,
}

impl Default for PerfUiDogName {
    fn default() -> Self {
        PerfUiDogName {
            label: String::new(),
            sort_key: perf_ui::utils::next_sort_key(),
        }
    }
}

impl PerfUiEntry for PerfUiDogName {
    type Value = String;
    type SystemParam = Query<'static, 'static, &'static Dog>;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Dog Name"
        } else {
            &self.label
        }
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(&self, dog_query: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>) -> Option<Self::Value> {
        let dog = dog_query.single();
        Some(dog.name.clone())
    }

    fn format_value(&self, value: &Self::Value) -> String {
        value.clone()
    }

    fn width_hint(&self) -> usize {
        20
    }
}

/**
 * This is just for the fucking gender of the 🐕 in the PerfUI lmao
 */
#[derive(Component)]
pub struct PerfUiDogGender {
    pub label: String,
    pub sort_key: i32,
}

impl Default for PerfUiDogGender {
    fn default() -> Self {
        PerfUiDogGender {
            label: String::new(),
            sort_key: perf_ui::utils::next_sort_key(),
        }
    }
}

impl PerfUiEntry for PerfUiDogGender {
    type Value = String;
    type SystemParam = Query<'static, 'static, &'static Dog>;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Dog Gender"
        } else {
            &self.label
        }
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(&self, dog_query: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>) -> Option<Self::Value> {
        let dog = dog_query.single();
        let gender = get_animal_gender(&dog.name);
        Some(gender.unwrap_or("Unknown").to_string())
    }

    fn format_value(&self, value: &Self::Value) -> String {
        value.clone()
    }

    fn width_hint(&self) -> usize {
        10
    }
}

/**
 * The 🐈‍⬛ Component
 */
#[derive(Component)]
pub struct Cat {
    name: String,
}

fn animate_cat_sprite(
    time: Res<Time>,
    mut query: Query<(&mut AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (mut indices, mut timer, mut atlas) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            indices.current_index = if indices.current_index == indices.last {
                if indices.first == 4 { // Death animation
                    4 // Loop back to the first frame of the death animation
                } else {
                    indices.first
                }
            } else {
                indices.current_index + 1
            };
            atlas.index = indices.current_index;
        }
    }
}

/**
 * More fucking 🐈‍⬛ stuff.
 */
 #[derive(Component)]
pub struct PerfUiCatHunger {
    pub label: String,
    pub sort_key: i32,
}

impl Default for PerfUiCatHunger {
    fn default() -> Self {
        PerfUiCatHunger {
            label: String::new(),
            sort_key: perf_ui::utils::next_sort_key(),
        }
    }
}

impl PerfUiEntry for PerfUiCatHunger {
    type Value = u32;
    type SystemParam = Query<'static, 'static, &'static Health, With<Cat>>;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Cat Hunger"
        } else {
            &self.label
        }
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(&self, health_query: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>) -> Option<Self::Value> {
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
pub struct PerfUiCatHealth {
    pub label: String,
    pub sort_key: i32,
}

impl Default for PerfUiCatHealth {
    fn default() -> Self {
        PerfUiCatHealth {
            label: String::new(),
            sort_key: perf_ui::utils::next_sort_key(),
        }
    }
}

impl PerfUiEntry for PerfUiCatHealth {
    type Value = String;
    type SystemParam = Query<'static, 'static, &'static Health, With<Cat>>;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Cat Health"
        } else {
            &self.label
        }
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(&self, health_query: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>) -> Option<Self::Value> {
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
pub struct PerfUiCatName {
    pub label: String,
    pub sort_key: i32,
}

impl Default for PerfUiCatName {
    fn default() -> Self {
        PerfUiCatName {
            label: String::new(),
            sort_key: perf_ui::utils::next_sort_key(),
        }
    }
}

impl PerfUiEntry for PerfUiCatName {
    type Value = String;
    type SystemParam = Query<'static, 'static, &'static Cat>;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Cat Name"
        } else {
            &self.label
        }
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(&self, cat_query: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>) -> Option<Self::Value> {
        let cat = cat_query.single();
        Some(cat.name.clone())
    }

    fn format_value(&self, value: &Self::Value) -> String {
        value.clone()
    }

    fn width_hint(&self) -> usize {
        20
    }
}

/**
 * This is just for the fucking gender of the 🐈‍⬛ in the PerfUI lmao
 */
#[derive(Component)]
pub struct PerfUiCatGender {
    pub label: String,
    pub sort_key: i32,
}

impl Default for PerfUiCatGender {
    fn default() -> Self {
        PerfUiCatGender {
            label: String::new(),
            sort_key: perf_ui::utils::next_sort_key(),
        }
    }
}

impl PerfUiEntry for PerfUiCatGender {
    type Value = String;
    type SystemParam = Query<'static, 'static, &'static Cat>;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Cat Gender"
        } else {
            &self.label
        }
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(&self, cat_query: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>) -> Option<Self::Value> {
        let cat = cat_query.single();
        let gender = get_animal_gender(&cat.name);
        Some(gender.unwrap_or("Unknown").to_string())
    }

    fn format_value(&self, value: &Self::Value) -> String {
        value.clone()
    }

    fn width_hint(&self) -> usize {
        10
    }
}

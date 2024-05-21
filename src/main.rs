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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    // Performance UI
    commands.spawn((
        PerfUiRoot {
            font_label: asset_server.load("bahnschrift.ttf"),
            font_value: asset_server.load("bahnschrift.ttf"),
            font_highlight: asset_server.load("bahnschrift.ttf"),
            values_col_width: Some(80.0),
            ..default()
        },
        PerfUiEntryFPS::default(),
        PerfUiTimeSinceLastClick::default(),
        PerfUiTimeSinceLastKeypress::default(),
        PerfUiSpaceKeyPressCount::default(),
        PerfUiCatName::default(),
        PerfUiCatGender::default(),
        PerfUiCatHealth::default(),
        PerfUiCatHunger::default(),
    ));

    // 🐈‍⬛
    let cat_name = generate_cat_name();
    let _cat_entity = commands.spawn((
        Cat {
            name: cat_name.clone(),
        },
        Health {
            current: 100,
            max: 100,
            hunger: 100,
        },
        SpriteBundle {
            texture: asset_server.load("cat-idle-1.png"),
            transform: Transform::from_xyz(25.0, 50.0, 0.0),
            ..Default::default()
        }
    ));

    // Print the cat's name
    //println!("The cat's name is: {}", cat_name);

    //match get_cat_gender(&cat_name) {
    //    Some(gender) => println!("The gender of {} is {}.", cat_name, gender),
    //    None => println!("Could not find a cat named {}.", cat_name),
    //}
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
        .add_perf_ui_entry_type::<PerfUiTimeSinceLastClick>()
        .add_perf_ui_entry_type::<PerfUiTimeSinceLastKeypress>()
        .add_perf_ui_entry_type::<PerfUiSpaceKeyPressCount>()
        .add_perf_ui_entry_type::<PerfUiCatName>() // I hate this 🐈‍⬛ already omg
        .add_perf_ui_entry_type::<PerfUiCatGender>()
        .add_perf_ui_entry_type::<PerfUiCatHealth>()
        .add_perf_ui_entry_type::<PerfUiCatHunger>()
        .init_resource::<CursorWorldCoordinates>() // Finally the 🐈‍⬛ stuff is over!
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
        .run();
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
    type SystemParam = Query<'static, 'static, &'static Health>;

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
    type SystemParam = Query<'static, 'static, &'static Health>;

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
        let gender = get_cat_gender(&cat.name);
        Some(gender.unwrap_or("Unknown").to_string())
    }

    fn format_value(&self, value: &Self::Value) -> String {
        value.clone()
    }

    fn width_hint(&self) -> usize {
        10
    }
}

// Let's be evil to the 🐈‍⬛!
#[derive(Resource, Default)]
struct HungerTimer(Timer);

fn decrease_hunger(
    time: Res<Time>,
    mut hunger_timer: ResMut<HungerTimer>,
    mut cat_query: Query<&mut Health>,
) {
    hunger_timer.0.tick(time.delta());
    if hunger_timer.0.just_finished() {
        if let Ok(mut health) = cat_query.get_single_mut() {
            health.hunger = health.hunger.saturating_sub(5);

            // If hunger reaches 0, decrease health by 5 every second
            if health.hunger == 0 {
                health.current = health.current.saturating_sub(5);
            }
        }
        // Set the timer's duration to 60 seconds for periodic decrease
        hunger_timer.0.set_duration(Duration::from_secs(1));
        // Reset the timer to count down again.
        hunger_timer.0.reset();
    }
}

/**
 * Stores the world position of the mouse cursor.
 */
#[derive(Resource, Default)]
struct CursorWorldCoordinates(Vec2);


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

pub const CAT_NAMES: &[(&str, &str)] = &[
    ("Picard", "male"), ("Beverly", "female"), ("Data", "male"), ("Troi", "female"), 
    ("Laforge", "male"), ("Crusher", "male"), ("Yar", "female"), ("Kirk", "male"),
    ("Spock", "male"), ("Mccoy", "male"), ("Scotty", "male"), ("Uhura", "female"), 
    ("Sulu", "male"), ("Chekov", "male"), ("Chakotay", "male"), ("Tuvok", "male"),
    ("Sisko", "male"), ("Kira", "female"), ("Dax", "female"), ("Bashir", "male"), 
    ("Odo", "male"), ("Quark", "male"), ("Archer", "male"), ("Tucker", "male"),
    ("Tpol", "female"), ("Reed", "male"), ("Mayweather", "male"), ("Phlox", "male"), 
    ("Sato", "female"), ("Sevenofnine", "female"), ("Thedoctor", "male"),
    ("Tomparis", "male"), ("Harrykim", "male"), ("Belanna", "female"), 
    ("Torres", "female"), ("Jeanluc", "male"), ("Lorca", "male"), ("Burnham", "female"),
    ("Saru", "male"), ("Stamets", "male"), ("Tilly", "female"), ("Georgiou", "female"), 
    ("Culber", "male"), ("Cornwell", "female"), ("Leland", "male"),
    ("Vance", "male"), ("Reno", "female"), ("Booker", "male"), ("Grudge", "female"), 
    ("Shaxs", "male"), ("Detmer", "female"), ("Owosekun", "female"), ("Rhys", "male"),
    ("Pike", "male"), ("Number One", "male"), ("Laan", "male"), ("Chapel", "female"), 
    ("Kyle", "male"), ("Vina", "female"), ("Mudd", "male"), ("Garak", "male"),
    ("Leyton", "male"), ("Ross", "male"), ("Nog", "male"), ("Jake", "male"), 
    ("Seven", "female"), ("Janeway", "female"), ("Tuvix", "male"), ("Neelix", "male"),
    ("Kes", "female"), ("Carey", "male"), ("Vorik", "male"), ("Wildman", "female"), 
    ("Zahir", "male"), ("Seska", "female"), ("Jonas", "male"), ("Rio", "male"),
    ("Maxwell", "male"), ("Tryla", "female"), ("Lorian", "male"), ("Icheb", "male"), 
    ("Q", "male"), ("Guinan", "female"), ("Pulaski", "female"), ("Ro", "female"),
    ("Hwomyn", "female"), ("Riker", "male"), ("Shelby", "female"), ("Obrien", "male"), 
    ("Keiko", "female"), ("Molly", "female"), ("Kirayoshi", "male"),
    ("Naomi", "female"), ("Ezri", "female"), ("Kassidy", "female"), ("Leeta", "female"), 
    ("Nog", "male"), ("Rom", "male"), ("Brunt", "male"), ("Ishka", "female"), ("Worf", "male"),
    ("Martok", "male"), ("Grilka", "female"), ("Sharan", "male"), ("Alexander", "male"), 
    ("Kehleyr", "female"), ("Lwaxana", "female"), ("Kamala", "female"),
    ("Vash", "female"), ("Tasha", "female"), ("Ogawa", "female"), ("Barclay", "male"), 
    ("Maddox", "male"), ("Soong", "male"), ("Juliana", "female"), ("Sela", "female"),
    ("Toral", "male"), ("Ziyal", "female"), ("Dukat", "male"), ("Damar", "male"), 
    ("Weyoun", "male"), ("Eddington", "male"), ("Michael", "male"),
    ("Sarina", "female"), ("Hugh", "male"), ("Lore", "male"), ("Elaurian", "male")
];

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
 * The Cat Component 🐈‍⬛
 */
#[derive(Component)]
pub struct Cat {
    name: String,
}

fn generate_cat_name() -> String {
    let mut rng = thread_rng();
    let (name, _gender) = CAT_NAMES.choose(&mut rng).unwrap();
    name.to_string()
}

fn get_cat_gender(name: &str) -> Option<&'static str> {
    for &(cat_name, gender) in CAT_NAMES {
        if cat_name == name {
            return Some(gender);
        }
    }
    None
}

/**
 * Identifies the main camera.
 */
#[derive(Component)]
struct MainCamera;


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

#[derive(Component)]
pub struct PerfUiSpaceKeyPressCount {
    pub label: String,
    pub sort_key: i32,
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

#[derive(Resource, Default)]
struct SpaceKeyPressState {
    last_pressed: bool,
}

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

fn cursor_system(
    mut coords: ResMut<CursorWorldCoordinates>,
    // Get the window
    window_query: Query<&Window, With<PrimaryWindow>>,
    // Get the camera transform
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // Get the camera info and transform
    let (camera, camera_transform) = camera_query.single();

    // There is only one primary window, so we can get it from the query:
    let window = window_query.single();

    // Check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z.
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        coords.0 = world_position;
        //eprintln!("CursorWorldCoordinate: {}/{}", world_position.x, world_position.y);
    }
}

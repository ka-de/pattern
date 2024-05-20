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
    ));

    // 🐈‍⬛
    let cat_name = generate_cat_name();
    let _cat_entity = commands.spawn((
        // Marker
        Cat {
            name: cat_name.clone(),
        },
        // A 2D sprite
        SpriteBundle {
            texture: asset_server.load("cat.png"),
            transform: Transform::from_xyz(25.0, 50.0, 0.0),
            // use the default values for all other components in the bundle
            ..Default::default()
        }
    ));

    // Print the cat's name
    println!("The cat's name is: {}", cat_name);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
        .add_perf_ui_entry_type::<PerfUiTimeSinceLastClick>()
        .add_perf_ui_entry_type::<PerfUiTimeSinceLastKeypress>()
        .add_perf_ui_entry_type::<PerfUiSpaceKeyPressCount>()
        .init_resource::<CursorWorldCoordinates>()
        .init_resource::<TimeSinceLastClick>()
        .init_resource::<TimeSinceLastKeypress>()
        .init_resource::<SpaceKeyPressCount>()
        .add_systems(Startup, setup)
        .add_systems(Update, cursor_system)
        .add_systems(Update, handle_click)
        .add_systems(Update, handle_keypress)
        .add_systems(Update, handle_space_keypress)
        .run();
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


/**
 * The Health Component 🩸 
 */
#[derive(Component)]
struct Health {
    current: u32,
    max: u32,
}

/**
 * The Cat Component 🐈‍⬛
 */
#[derive(Component)]
struct Cat {
    name: String,
}

fn generate_cat_name() -> String {
    let mut rng = thread_rng();
    let cat_names = vec![
        "picard", "beverly", "data", "troi", "laforge", "crusher", "yar", "kirk",
        "spock", "mccoy", "scotty", "uhura", "sulu", "chekov", "chakotay", "tuvok",
        "sisko", "kira", "dax", "bashir", "odo", "quark", "archer", "tucker",
        "tpol", "reed", "mayweather", "phlox", "sato", "sevenofnine", "thedoctor",
        "tomparis", "harrykim", "belanna", "torres", "jeanluc", "lorca", "burnham",
        "saru", "stamets", "tilly", "georgiou", "culber", "cornwell", "leland",
        "vance", "reno", "booker", "grudge", "shaxs", "detmer", "owosekun", "rhys",
        "pike", "number-one", "laan", "chapel", "kyle", "vina", "mudd", "garak",
        "leyton", "ross", "nog", "jake", "seven", "janeway", "tuvix", "neelix",
        "kes", "carey", "vorik", "wildman", "zahir", "seska", "jonas", "rio",
        "maxwell", "tryla", "lorian", "icheb", "q", "guinan", "pulaski", "ro",
        "hwomyn", "riker", "shelby", "obrien", "keiko", "molly", "kirayoshi",
        "naomi", "ezri", "kassidy", "leeta", "nog", "rom", "brunt", "ishka", "worf",
        "martok", "grilka", "sharan", "alexander", "kehleyr", "lwaxana", "kamala",
        "vash", "tasha", "ogawa", "barclay", "maddox", "soong", "juliana", "sela",
        "toral", "ziyal", "dukat", "damar", "weyoun", "eddington", "michael",
        "sarina", "hugh", "lore", "el-aurian"
    ];


    let name = cat_names.choose(&mut rng).unwrap().to_string();
    return name;
}

/**
 * Identifies the main camera.
 */
#[derive(Component)]
struct MainCamera;

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

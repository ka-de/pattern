use rand::prelude::*;
use bevy::prelude::*;
use bevy::utils::Duration;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::keyboard::KeyCode;
use bevy::input::ButtonState;
use bevy::ecs::system::lifetimeless::SRes;
use bevy::ecs::system::SystemParam;
use bevy::window::PrimaryWindow;
use iyes_perf_ui::prelude::*;
use iyes_cli::prelude::*;

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
 * The Cat Component 🐈‍⬛
 */
#[derive(Component)]
struct Cat {
    name: String,
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

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        coords.0 = world_position;
        eprintln!("CursorWorldCoordinate: {}/{}", world_position.x, world_position.y);
    }
}

#[derive(Component)]
struct DespawnTimeout(Timer);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, world: &mut World) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
    let camera = MainCamera;
    world.spawn(camera);

    let cat_names = vec![
        "picard", "beverly", "data", "troi", "laforge", "crusher", "yar", "kirk",
    ];

    let mut rng = thread_rng();

    let name = cat_names.choose(&mut rng).unwrap().to_string();
    commands.spawn((Cat { name: name.clone() },));
    println!("Cat name: {}", name);

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
        .register_clicommand_noargs("version", show_version)
        .register_clicommand_noargs("help", show_help)
        .register_clicommand_noargs("spawn", spawn_sprite_random)
        .register_clicommand_args("spawn", spawn_sprite_at)
        .register_clicommand_noargs("despawn", despawn_sprites)
        .add_systems(Startup, (setup, setup_console))
        .add_systems(Update, (mouseclicks, console_text_input, despawn_timeout))
        .add_systems(Update, cursor_system)
        .add_systems(Update, handle_click)
        .add_systems(Update, handle_keypress)
        .add_systems(Update, handle_space_keypress)
        .run();
}

fn setup_console(world: &mut World) {
    let font = world.resource::<AssetServer>().load("Ubuntu-R.ttf");
    let console = world
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Percent(5.0),
                left: Val::Percent(5.0),
                top: Val::Auto,
                right: Val::Auto,
                padding: UiRect::all(Val::Px(8.0)),
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::BEIGE),
            ..Default::default()
        })
        .id();
    let prompt_style = TextStyle {
        font: font.clone(),
        font_size: 24.0,
        color: Color::RED,
    };
    let input_style = TextStyle {
        font: font.clone(),
        font_size: 16.0,
        color: Color::BLACK,
    };
    let prompt = world
        .spawn((
            CliPrompt,
            TextBundle {
                text: Text::from_sections([
                    TextSection::new("~ ", prompt_style),
                    TextSection::new("", input_style),
                ]),
                ..Default::default()
            },
        ))
        .id();
    world.entity_mut(console).push_children(&[prompt]);
}

#[derive(Component)]
struct CliPrompt;

fn show_version() {
    println!("TODO");
}

fn spawn_sprite_random(q_window: Query<&Window, With<PrimaryWindow>>, mut commands: Commands) {
    let window = q_window.single();
    let mut rng = thread_rng();
    commands.spawn((
        DespawnTimeout(Timer::new(Duration::from_secs(5), TimerMode::Once)),
        SpriteBundle {
            sprite: Sprite {
                color: Color::PINK,
                custom_size: Some(Vec2::splat(64.0)),
                ..default()
            },
            transform: Transform::from_xyz(
                rng.gen_range(0.0..window.width()),
                rng.gen_range(0.0..window.height()),
                1.0,
            ),
            ..default()
        },
    ));
}

fn spawn_sprite_at(In(args): In<Vec<String>>, mut commands: Commands) {
    if args.len() != 2 {
        error!("spawn command must take exactly 2 args!");
        return;
    }
    let Ok(x) = args[0].parse::<f32>() else {
        error!("spawn command args must be numbers!");
        return;
    };
    let Ok(y) = args[1].parse::<f32>() else {
        error!("spawn command args must be numbers!");
        return;
    };

    commands.spawn((
        DespawnTimeout(Timer::new(Duration::from_secs(5), TimerMode::Once)),
        SpriteBundle {
            sprite: Sprite {
                color: Color::PINK,
                custom_size: Some(Vec2::splat(64.0)),
                ..default()
            },
            transform: Transform::from_xyz(x, y, 1.0),
            ..default()
        },
    ));
}

fn despawn_sprites(mut commands: Commands, q: Query<Entity, With<Sprite>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}

fn show_help(world: &mut World) {
    let font = world.resource::<AssetServer>().load("bahnschrift.ttf");
    let help_box = world
        .spawn((
            DespawnTimeout(Timer::new(Duration::from_secs(5), TimerMode::Once)),
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(5.0),
                    left: Val::Percent(5.0),
                    bottom: Val::Auto,
                    right: Val::Auto,
                    padding: UiRect::all(Val::Px(8.0)),
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::BEIGE),
                ..Default::default()
            },
        ))
        .id();
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 12.0,
        color: Color::BLACK,
    };
    let prompt = world
        .spawn((TextBundle {
            text: Text::from_section(
                "Available console commands: \"help\", \"hello\", \"spawn\", \"spawn <x> <y>\", \"despawn\".\n
                Left/Right mouse click will run \"spawn\"/\"despawn\".",
                text_style,
            ),
            ..Default::default()
        },))
        .id();
    world.entity_mut(help_box).push_children(&[prompt]);
}

fn despawn_timeout(
    mut commands: Commands,
    t: Res<Time>,
    mut q: Query<(Entity, &mut DespawnTimeout)>,
) {
    for (e, mut timeout) in &mut q {
        timeout.0.tick(t.delta());
        if timeout.0.finished() {
            commands.entity(e).despawn_recursive();
        }
    }
}
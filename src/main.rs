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
            name: generate_cat_name(),
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
            name: generate_dog_name(),
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

fn update_facing_direction(
    mut query: Query<(&mut Transform, &Velocity), Or<(With<Dog>, With<Cat>)>>,
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
    type SystemParam = Query<'static, 'static, &'static Health, With<Cat>>;

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
    type SystemParam = Query<'static, 'static, &'static Health, With<Cat>>;

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

    fn update_value(&self, cat_query: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>) -> Option<Self::Value> {
        let cat = cat_query.single();
        let gender = get_dog_gender(&cat.name);
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

pub const DOG_NAMES: &[(&str, &str)] = &[
    ("Malcolm", "male"), ("Zoe", "female"), ("Wash", "male"), ("Inara", "female"),
    ("Jayne", "male"), ("Kaylee", "female"), ("Simon", "male"), ("River", "female"),
    ("Book", "male"), ("Saffron", "female"), ("Badger", "male"), ("Nandi", "female"),
    ("Bester", "male"), ("Dobson", "male"), ("Atherton", "male"), ("Gabriel", "male"),
    ("Regan", "female"), ("Tracey", "male"), ("Amnon", "male"), ("Fess", "male"),
    ("Rance", "male"), ("Magistrate", "male"), ("Lucy", "female"), ("Ruth", "female"),
    ("Bree", "female")
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
 * The 🐕 Component
 */
#[derive(Component)]
pub struct Dog {
    name: String,
}

// Function to generate dog names
fn generate_dog_name() -> String {
    let mut rng = thread_rng();
    let (name, _gender) = DOG_NAMES.choose(&mut rng).unwrap();
    name.to_string()
}

fn get_dog_gender(name: &str) -> Option<&'static str> {
    for &(cat_name, gender) in DOG_NAMES {
        if cat_name == name {
            return Some(gender);
        }
    }
    None
}

/**
 * The 🐈‍⬛ Component
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
        //eprintln!("CursorWorldCoordinate: {}/{}", world_position.x, world_position.y);
    }
}

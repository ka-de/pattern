use bevy::{
    asset::Handle,
    math::Vec2,
    prelude::{ Resource, States },
    render::{ camera::ScalingMode, color::Color, texture::Image },
};
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    standard_dynamic_asset::StandardDynamicAsset,
};

/// Configuration for the SplashScreenPlugin
#[derive(Resource, Debug, Clone)]
pub struct SplashScreenConfiguration<T: States + Clone> {
    /// This is the state which the splash screens should be displayed
    pub run_state: T,
    /// This is the state after all the splash screens have been displayed
    pub next_state: T,
    /// This is a path relative to the `assets` folder which contains the splash screens to be displayed
    pub images: StandardDynamicAsset,
    /// This is a way to override the size of each image.
    pub custom_size: Option<Vec2>,
    /// This is how long in seconds each splash screen should be displayed
    pub splash_timer: f32,
    /// This is the clear color for the splash camera
    pub clear_color: Color,
    /// This is the scaling mode to be set for the splash camera
    pub camera_scaling_mode: ScalingMode,
}

/// Asset collection holding all of the splash screen handles
#[derive(AssetCollection, Resource)]
pub struct SplashScreenImages {
    #[asset(key = "splash_screen_path", collection(typed))]
    pub images: Vec<Handle<Image>>,
}

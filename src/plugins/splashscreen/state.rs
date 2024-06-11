use bevy::ecs::schedule::States;

/// Controls the flow of our plugin
#[derive(States, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SplashScreenState {
    /// Nothing going on - setup initialization
    ///
    /// This keeps [`ProgressPlugin`] and [`LoadingState`] from running when not in `T::run_state`
    #[default]
    Idle,
    /// Load splash screen images
    Initialize,
    /// Display splash screen images
    Update,
}

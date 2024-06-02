use std::time::Duration;

use bevy::{ ecs::system::{ Query, Res, ResMut, Resource }, time::{ Time, Timer } };

use super::health::Health;

// 🍗
#[derive(Resource, Default)]
pub struct HungerTimer(Timer); // A timer to track the hunger of entities

// Function to decrease the hunger of entities over time
pub fn decrease_hunger(
    time: Res<Time>, // The current time
    mut hunger_timer: ResMut<HungerTimer>, // The hunger timer
    mut health_query: Query<&mut Health> // The health of the entities
) {
    // Update the hunger timer
    hunger_timer.0.tick(time.delta());
    // If the hunger timer has just finished
    if hunger_timer.0.just_finished() {
        // Iterate over the entities
        for mut health in &mut health_query {
            // Decrease the hunger of the entity by 1
            health.hunger = health.hunger.saturating_sub(1);
            // If the hunger of the entity reaches 0, decrease its health by 1
            if health.hunger == 0 {
                health.current = health.current.saturating_sub(1);
            }
        }
        // Set the duration of the hunger timer to 20 seconds
        hunger_timer.0.set_duration(Duration::from_secs(20));
        // Reset the hunger timer to count down again
        hunger_timer.0.reset();
    }
}

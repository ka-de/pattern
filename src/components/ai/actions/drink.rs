use bevy::{
    reflect::Reflect,
    time::Time,
    log::{ trace, debug },
    ecs::{ system::{ Query, Res }, component::Component },
};
use big_brain::prelude::{ Actor, ActionBuilder, ActionState, ActionSpan };

use crate::components::ai::thirst::Thirst;

#[derive(Reflect, Clone, Component, Debug, ActionBuilder)]
pub(crate) struct Drink {
    pub until: f32,
    pub per_second: f32,
}

// Action systems execute according to a state machine, where the states are
// labeled by ActionState.
pub(crate) fn drink_action_system(
    time: Res<Time>,
    mut thirsts: Query<&mut Thirst>,
    // We execute actions by querying for their associated Action Component
    // (Drink in this case). You'll always need both Actor and ActionState.
    mut query: Query<(&Actor, &mut ActionState, &Drink, &ActionSpan)>
) {
    for (Actor(actor), mut state, drink, span) in &mut query {
        // This sets up the tracing scope. Any `debug` calls here will be
        // spanned together in the output.
        let _guard = span.span().enter();

        // Use the drink_action's actor to look up the corresponding Thirst Component.
        if let Ok(mut thirst) = thirsts.get_mut(*actor) {
            match *state {
                ActionState::Requested => {
                    debug!("Time to drink some water!");
                    *state = ActionState::Executing;
                }
                ActionState::Executing => {
                    trace!("Drinking...");
                    thirst.thirst -=
                        drink.per_second * ((time.delta().as_micros() as f32) / 1_000_000.0);
                    if thirst.thirst <= drink.until {
                        // To "finish" an action, we set its state to Success or
                        // Failure.
                        debug!("Done drinking water");
                        *state = ActionState::Success;
                    }
                }
                // All Actions should make sure to handle cancellations!
                ActionState::Cancelled => {
                    debug!("Action was cancelled. Considering this a failure.");
                    *state = ActionState::Failure;
                }
                _ => {}
            }
        }
    }
}

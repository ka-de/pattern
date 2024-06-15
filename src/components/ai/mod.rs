// 🧠 - AI

// Not to be confused by an actual LLM!

pub(crate) mod actions;
pub(crate) mod scorers;

// Components
pub(crate) mod thirst;

use bevy::ecs::system::Commands;

use big_brain::prelude::{ FirstToScore, Thinker };

use crate::components::ai::{ actions::drink::Drink, scorers::thirsty::Thirsty, thirst::Thirst };

// Now that we have all that defined, it's time to add a Thinker to an entity!
// The Thinker is the actual "brain" behind all the AI. Every entity you want
// to have AI behavior should have one *or more* Thinkers attached to it.
pub(crate) fn setup(mut cmd: Commands) {
    // Create the entity and throw the Thirst component in there. Nothing special here.
    // Neutral AI Brain
    cmd.spawn((
        Thirst::new(75.0, 1.6),
        Thinker::build()
            .label("NeutralAIBrain")
            .picker(FirstToScore { threshold: 0.8 })
            // Technically these are supposed to be ActionBuilders and
            // ScorerBuilders, but our Clone impls simplify our code here.
            .when(Thirsty, Drink {
                until: 1.0,
                per_second: 5.0,
            }),
        // ⚠️ TODO:
        // When damaged by the player, become hostile.
    ));

    // AggressiveAIBrain
    cmd.spawn((
        Thirst::new(75.0, 1.6),
        Thinker::build()
            .label("AggressiveAIBrain")
            .picker(FirstToScore { threshold: 0.8 })
            .when(Thirsty, Drink {
                until: 1.0,
                per_second: 5.0,
            }),
        // ⚠️ TODO:
        // When player is in line of sight and at the proper
        // aggro radius, engage in combat.
    ));
}

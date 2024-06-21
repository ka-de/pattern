use bevy::{
    reflect::Reflect,
    time::Time,
    log::{ trace, debug },
    ecs::{ system::{ Commands, Query, Res }, component::Component },
};

// First, we define a "Thirst" component and associated system. This is NOT
// THE AI. It's a plain old system that just makes an entity "thirstier" over
// time. This is what the AI will later interact with.
//
// There's nothing special here. It's a plain old Bevy component.
#[derive(Component, Debug, Reflect)]
pub(crate) struct Thirst {
    pub per_second: f32,
    pub thirst: f32,
}

impl Thirst {
    pub fn new(thirst: f32, per_second: f32) -> Self {
        Self { thirst, per_second }
    }
}

pub(crate) fn thirst_system(time: Res<Time>, mut thirsts: Query<&mut Thirst>) {
    for mut thirst in &mut thirsts {
        thirst.thirst += thirst.per_second * ((time.delta().as_micros() as f32) / 1_000_000.0);
        if thirst.thirst >= 100.0 {
            debug!("Thirst >= {}", thirst.thirst);
            thirst.thirst = 100.0;
        }

        trace!("Thirst: {}", thirst.thirst);
    }
}

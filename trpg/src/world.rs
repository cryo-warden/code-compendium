use crate::resources::Time;
use crate::systems::{
    movement::{movement_logging_system, movement_system},
    time::time_system,
};
use bevy_ecs::bundle::Bundle;
use bevy_ecs::prelude::{Schedule, World};

pub struct TRPGWorld {
    world: World,
    schedule: Schedule,
}

impl TRPGWorld {
    pub fn new() -> Self {
        let mut schedule = Schedule::default();
        schedule.add_systems((time_system, movement_system, movement_logging_system));
        let mut world = World::default();
        world.insert_resource(Time::new());
        TRPGWorld { world, schedule }
    }

    pub fn spawn(&mut self, components: impl Bundle) {
        self.world.spawn(components);
    }

    pub fn update(&mut self) {
        self.schedule.run(&mut self.world);
    }
}

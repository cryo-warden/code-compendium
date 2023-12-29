use crate::components::{Logger, Position, Velocity};
use crate::resources::Time;
use bevy_ecs::prelude::*;

pub fn movement_system(mut query: Query<(&mut Position, &Velocity)>, time: Res<Time>) {
    for (mut position, velocity) in query.iter_mut() {
        position.position += velocity.velocity * (time.seconds_delta as f32);
    }
}

pub fn movement_logging_system(mut query: Query<(Entity, &Position, &Logger)>, time: Res<Time>) {
    println!(
        "Time {:?}, Delta Time {:?}",
        time.seconds, time.seconds_delta
    );
    for (entity, position, _) in query.iter_mut() {
        println!("Entity {:?} moved to {:?}", entity, position.position);
    }
}

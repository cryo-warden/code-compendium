use crate::resources::Time;
use bevy_ecs::prelude::*;

pub fn time_system(mut resource: ResMut<Time>) {
    // It is possible for the system time to go backward, but we want to absorb that silently.
    let new_seconds = Time::unix_now().max(resource.seconds);

    // Don't let seconds_delta be zero or negative, in case of potential division by zero or reversed physics.
    resource.seconds_delta = (new_seconds - resource.seconds).max(1e-9);

    resource.seconds = new_seconds;
    println!(
        "UPDATE: Time {:?}, Delta Time {:?}",
        resource.seconds, resource.seconds_delta
    );
}

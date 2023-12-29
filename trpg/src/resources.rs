use bevy_ecs::prelude::Resource;
use std::time::SystemTime;

#[derive(Resource)]
pub struct Time {
    /// Seconds since the last frame, useful for physics.
    pub seconds_delta: f64,
    /// Seconds since the UNIX epoch, useful for awaiting specific timestamps.
    pub seconds: f64,
}

impl Time {
    pub fn new() -> Self {
        Time {
            seconds_delta: 1e-9,
            seconds: Time::unix_now(),
        }
    }

    pub fn unix_now() -> f64 {
        match SystemTime::UNIX_EPOCH.elapsed() {
            Ok(duration) => duration.as_secs_f64(),
            _ => 0.0,
        }
    }
}

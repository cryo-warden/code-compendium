use bevy_ecs::prelude::*;
use geometry::prelude::*;

#[derive(Component)]
pub struct Position {
    pub position: Vector,
}

#[derive(Component)]
pub struct Velocity {
    pub velocity: Vector,
}

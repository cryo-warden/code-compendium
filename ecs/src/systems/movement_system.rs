use crate::components::{Position, Velocity};

pub struct MovementSystem {}

impl MovementSystem {
    pub fn update(&self, positions: &mut Vec<Position>, velocities: &mut Vec<Velocity>) {
        // TODO Need to implement matching for related components.
        dbg!(positions);
        dbg!(velocities);
    }
}

use crate::components::{Position, Velocity};

pub struct MovementSystem {}

impl MovementSystem {
    pub fn update(&self, components: &mut Vec<(&mut Position, &mut Velocity)>) {
        for (ref mut position, ref mut velocity) in components.iter_mut() {
            position.position += velocity.velocity;
        }
        dbg!(components);
    }
}

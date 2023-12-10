use ecs::components::{Position, Velocity};
use ecs::define_world;
use ecs::systems::GravitySystem;
use ecs::systems::MovementSystem;

define_world! {
  World {
    movement_system: MovementSystem {
      positions: Position,
      velocities: Velocity,
    }
    gravity_system: GravitySystem {
      positions: Position,
      velocities: Velocity,
    }
  }
}

fn main() {
    let mut world = World::new(MovementSystem {}, GravitySystem {});
    world.update();
}

use ecs::systems::MovementSystem;
use ecs::World;

fn main() {
    let mut world = World::new(MovementSystem {});
    world.update();
}

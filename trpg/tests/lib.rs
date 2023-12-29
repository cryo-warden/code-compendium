#[cfg(test)]
mod tests {
    use geometry::prelude::*;
    use rand::{rngs::ThreadRng, Rng};
    use trpg::components::{Logger, Position, Velocity};
    use trpg::World;

    fn rand_vector(rng: &mut ThreadRng) -> Vector {
        let rand_range = -1e6..1e6;
        Vector(
            rng.gen_range(rand_range.clone()),
            rng.gen_range(rand_range.clone()),
        )
    }

    #[test]
    fn creates_a_world() {
        println!("Creating a world");
        let mut world = World::new();

        world.spawn((
            Logger,
            Position {
                position: Vector(0.0, 0.0),
            },
            Velocity {
                velocity: Vector(1.0, 0.66),
            },
        ));
        world.spawn((
            Logger,
            Position {
                position: Vector(1000.0, 1000.0),
            },
            Velocity {
                velocity: Vector(-10.0, -6.6),
            },
        ));
        world.spawn((
            Logger,
            Position {
                position: Vector(0.0, 0.0),
            },
            Velocity {
                velocity: Vector(100.0, 0.0),
            },
        ));
        world.spawn((
            Logger,
            Position {
                position: Vector(0.0, 0.0),
            },
        ));

        let mut rng = rand::thread_rng();
        for _ in 0..10000 {
            world.spawn(Position {
                position: rand_vector(&mut rng),
            });
        }
        for _ in 0..10000 {
            world.spawn((
                Position {
                    position: rand_vector(&mut rng),
                },
                Velocity {
                    velocity: rand_vector(&mut rng),
                },
            ));
        }

        for _ in 0..10 {
            world.update();
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}

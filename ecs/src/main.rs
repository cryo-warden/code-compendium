use ecs::components::{Position, Velocity};
// use ecs::define_world;
use ecs::systems::GravitySystem;
use ecs::systems::MovementSystem;

// define_world! {
//   World {
//     movement_system: MovementSystem {
//       positions: Position,
//       velocities: Velocity,
//     }
//     gravity_system: GravitySystem {
//       positions: Position,
//       velocities: Velocity,
//     }
//   }
// }

// Explore possible designs by creating one invariant world definition outside of any specialized world macros.
// Problems to solve:
// Every component instance must be owned in a memory-local fashion, so that they can be iterated over in a cache-friendly way.
// Every system must receive a certain set of components when calling `update`, with each component behind a mutable reference, and only receive a component when certain other components are present on the same entity.
// A system might require multiple separate sets of same-entity components, to produce some interaction between these sets on update. One way to handle this is by adding an intermediate EntityQuery type.

// We might be able to define Query types in terms of an enum set of component types, and provide the results as a vec of ArchetypeBuilder references. Whenever a component is added or removed, update that entity's type set, then check the new type set against each query, adding and removing the entity from the query's results as necessary.

struct InvariantWorld {
    movement_system: MovementSystem,
    positions: Vec<Position>,
    velocities: Vec<Velocity>,
    gravity_system: GravitySystem,
    movement_system_query: MovementSystemQuery,
    gravity_system_query: GravitySystemQuery,
}

// Define a struct which can optionally contain each component that can possibly appear in the given world. Use that derived partial type to specify an individual entity.
struct WorldArchetype {
    position: Option<Position>,
    velocity: Option<Velocity>,
}

struct WorldMutRefArchetype {
    position: Option<&mut Position>,
    velocity: Option<&mut Velocity>,
}

pub struct MovementSystemQuery {
    pub results: Vec<(&mut Position, &mut Velocity)>,
}

impl MovementSystemQuery {
    pub fn new() -> Self {
        MovementSystemQuery { results: vec![] }
    }

    fn submit_archetype(&mut self, archetype: WorldMutRefArchetype) {
        if let (Some(position), Some(velocity)) = (archetype.position, archetype.velocity) {
            self.results.push((position, velocity));
        }
    }
}

// TODO Make separate Queries for gravity sources and gravity receivers. This can demonstrate multi-query systems.
struct GravitySystemQuery {
    results: Vec<(&mut Position, &mut Velocity)>,
}

impl GravitySystemQuery {
    pub fn new() -> Self {
        GravitySystemQuery { results: vec![] }
    }

    fn submit_archetype(&mut self, archetype: WorldMutRefArchetype) {
        if let (Some(position), Some(velocity)) = (archetype.position, archetype.velocity) {
            self.results.push((position, velocity));
        }
    }
}

impl InvariantWorld {
    pub fn new(movement_system: MovementSystem, gravity_system: GravitySystem) -> Self {
        InvariantWorld {
            movement_system,
            positions: vec![],
            velocities: vec![],
            gravity_system,
            movement_system_query: MovementSystemQuery::new(),
            gravity_system_query: GravitySystemQuery::new(),
        }
    }

    pub fn add_entity(&mut self, archetype: WorldArchetype) {
        let mut refMutArchetype = WorldMutRefArchetype {
            position: None,
            velocity: None,
        };

        if let Some(position) = archetype.position {
            self.positions.push(position);
            refMutArchetype.position = Some(&mut self.positions[self.positions.len() - 1]);
        }

        self.movement_system_query.submit_archetype(refMutArchetype);
        // self.gravity_system_query.submit_archetype(refMutArchetype);
    }

    pub fn update(&mut self) {
        // Explicitly invoke update method for each known system. No loop required, since the macro version will know all systems at compile time.
        self.movement_system
            .update(&mut self.movement_system_query.results);
        // self.gravity_system.update(&mut self.gravity_system_query.results);
    }
}

fn main() {
    let mut world = InvariantWorld::new(MovementSystem {}, GravitySystem {});
    world.update();
}

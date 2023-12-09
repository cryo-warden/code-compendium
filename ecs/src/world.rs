use crate::components::{Position, Velocity};
use crate::systems::MovementSystem;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Entity(u32);

// TODO Fix the world macro so that multiple systems can have the same components without creating duplicate field names.
// TODO Add the ability to actually add entities and components to a world.

#[macro_export]
macro_rules! define_world {
  ($world:ident, $($system_id:ident: $system_type:ty => [$($component_id:ident: $component_type:ty,)*]),*) => {
    pub struct $world {
      $(
        $system_id: $system_type,
        $($component_id: Vec<$component_type>,)*
      )*
    }

    impl $world {
      pub fn new($($system_id: $system_type, )*) -> Self {
        $world {
          $(
            $system_id,
            $($component_id: vec![],)*
          )*
        }
      }

      pub fn update(&mut self) {
        $(self.$system_id.update(
          $(&mut self.$component_id,)*
        ))*;
      }
    }
  };
}

define_world! {
  World,
  movement_system: MovementSystem => [
    positions: Position,
    velocities: Velocity,
  ]
}

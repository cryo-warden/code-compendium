#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Entity(u32);

// TODO Add the ability to actually add entities and components to a world.
// TODO Maybe convert this to a proc macro, because there's more that needs to be added, such as an enum over all possible component types and/or a function that can add a specific type of component to an entity, and we're already fighting the limitations of macro_rules to accomplish much less.

#[macro_export]
macro_rules! define_world {
  ($world:ident {
    $($system_id:ident: $system_type:ty {
      $($component_id:ident: $component_type:ty,)*
    })*
  }) => {
    ecs_macros::remove_duplicate_struct_fields! {
      pub struct $world {
        $(
          $system_id: $system_type,
          $($component_id: Vec<$component_type>,)*
        )*
      }
    }

    impl $world {
      pub fn new($($system_id: $system_type, )*) -> Self {
        ecs_macros::remove_duplicate_struct_initializer_fields! {
          $world {
            $(
              $system_id,
              $($component_id: vec![],)*
            )*
          }
        }
      }

      pub fn update(&mut self) {
        $(self.$system_id.update(
          $(&mut self.$component_id,)*
        );)*
      }
    }
  };
}

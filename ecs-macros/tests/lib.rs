#[macro_use]
extern crate ecs_macros;

#[cfg(test)]
mod tests {
    #[test]
    fn test_remove_duplicate_struct_fields() {
        remove_duplicate_struct_fields! {
          #[derive(Debug, Copy, Clone)]
          struct Deduplicated {
              field1: u32,
              field2: u32,
              field1: u32,
              field3: u32,
              field2: u32,
          }
        }

        let x = remove_duplicate_struct_initializer_fields! {
            Deduplicated {
              field1: 1,
              field2: 2,
              field2: 88,
              field2: 538,
              field3: 3,
              field3: 112,
              field3: 662,
              field1: 123,
              field2: 234,
              field3: 345,
          }
        };

        dbg!(&x);
        println!("{:?}, {:?}, {:?}", x.field1, x.field2, x.field3);
    }
}

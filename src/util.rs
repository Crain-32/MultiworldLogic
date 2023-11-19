#[cfg(test)]
pub mod test {
    pub mod world {
        use std::collections::BTreeMap;
        use bitvec::prelude::*;
        use test_context::TestContext;
        use crate::alias::{Item, Requirement, State};

        use crate::world::{Location, World};

        pub struct SingleWorldContext {
            pub world: World,
        }

        fn make_simple_world(id: usize) -> World {
            World{
                world_id: id,
                item_pool: vec![],
                available_locations: vec![],
                locked_locations: BTreeMap::from([
                    (Requirement { requirement: bitvec![0,0,0], needs_count: None }, vec![Location { id: 0, world_id: 0, current_item: Item::default() }, Location { id: 1, world_id: 0, current_item: Item::default() }]),
                    (Requirement { requirement: bitvec![0,0,1], needs_count: None }, vec![Location { id: 2, world_id: 0, current_item: Item::default() }, Location { id: 3, world_id: 0, current_item: Item::default() }])
                ]).to_owned(),
                player_state: State{state: bitvec![0,0,0], item_count: BTreeMap::default()} ,
            }
        }

        impl TestContext for SingleWorldContext {
            fn setup() -> Self {
                return SingleWorldContext { world: make_simple_world(0) };
            }

            fn teardown(self) {
                // Should be self-contained, so no teardown needed.
            }
        }
    }
}
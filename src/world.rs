pub mod world {
    use std::collections::HashSet;
    use std::iter::Map;
    use derive_more::Display;

    use crate::alias::alias::{Requirement, Item, State};


    #[derive(Debug, Display)]
    pub struct Area {
        name: str,
        locations: Vec<Location>,
        exits: Map<str, Exit>,
        is_accessible: bool,
        world_id: u8,
    }

    #[derive(Debug, Display)]
    pub struct Exit {
        name: str,
        to: Area,
        requirement: Requirement,
        world_id: u8,
    }

    #[derive(Debug, Display, Clone, Copy, Default)]
    pub struct Location {
        pub name: str,
        category_set: HashSet<str>, // Could probably just u32 this
        area_name: str,
        requirement: Requirement,
        pub current_item: Item,
        pub world_id: i16,
        logical: bool,
    }

    #[derive(Debug, Display)]
    pub struct World {
        world_id: u8,
        root: Area,
        item_pool: Vec<Item>,
        requirement_pool: Map<str, Requirement>,
        player_state: State
    }
}
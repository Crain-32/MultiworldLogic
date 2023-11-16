pub mod world {
    use std::collections::HashSet;
    use std::iter::Map;

    use crate::alias::alias::{Requirement, Item, State};


    pub struct Area {
        name: str,
        locations: Vec<Location>,
        exits: Map<str, Exit>,
        is_accessible: bool,
        world_id: u8,
    }

    pub struct Exit {
        name: str,
        to: Area,
        requirement: Requirement,
        world_id: u8,
    }

    pub struct Location {
        name: str,
        category_set: HashSet<str>, // Could probably just u32 this
        area_name: str,
        requirement: Requirement,
        current_item: Item,
        logical: bool,
    }

    pub struct World {
        world_id: u8,
        root: Area,
        requirement_pool: Map<str, Requirement>,
        player_state: State
    }
}
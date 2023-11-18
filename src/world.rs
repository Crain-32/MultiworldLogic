use std::iter::Map;

use bitvec::vec::BitVec;

use crate::alias::{Item, Requirement, State};

pub type LocationId = usize;

#[derive(Debug, Clone, Default)]
pub struct Location {
    pub id: LocationId,
    pub current_item: Item,
}

#[derive(Debug)]
pub struct World {
    pub world_id: usize,
    item_pool: Vec<Item>,
    pub locations: Map<Requirement, Vec<Location>>,
    pub player_state: State,
    pub count_state: Map<Item, usize>
}

// To Generate Playthrough crap.
#[derive(Debug)]
pub struct MetaData {
    world_id: usize,
    access_mapping: Vec<Vec<usize>>,
    // Indexed by Location Id
    region: Vec<usize>, // Region by Location Id
    items: Map<BitVec, BitVec>,
    location_name: Vec<String>,
    // Indexed by Location Id
    settings: Vec<String>,
}
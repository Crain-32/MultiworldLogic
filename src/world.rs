use std::iter::Map;

use bitvec::vec::BitVec;

use crate::alias::{Item, Requirement, State};

pub type LocationId = u64;

#[derive(Debug, Clone, Default)]
pub struct Location {
    pub id: LocationId,
    pub current_item: Item,
}

#[derive(Debug)]
pub struct World {
    world_id: usize,
    item_pool: Vec<Item>,
    locations: (Requirement, Vec<Location>),
    // Handle by Index
    player_state: State,
}

// To Generate Playthrough crap.
#[derive(Debug)]
pub struct MetaData {
    world_id: usize,
    access_mapping: Vec<Vec<u64>>,
    // Indexed by Location Id
    region: Vec<u64>, // Region by Location Id
    items: Map<BitVec, BitVec>,
    location_name: Vec<String>,
    // Indexed by Location Id
    settings: Vec<String>,
}
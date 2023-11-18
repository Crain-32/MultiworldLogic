use std::collections::BTreeMap;
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
    pub item_pool: Vec<Item>,
    pub available_locations: Vec<Location>,
    pub locked_locations: BTreeMap<Requirement, Vec<Location>>,
    pub player_state: State
}

// To Generate Playthrough crap.
#[derive(Debug)]
pub struct MetaData {
    world_id: usize,
    access_mapping: Vec<Vec<usize>>,
    // Indexed by Location Id
    region: Vec<usize>, // Region by Location Id
    items: BTreeMap<BitVec, BitVec>,
    location_name: Vec<String>,
    // Indexed by Location Id
    settings: Vec<String>,
}
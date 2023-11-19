use std::collections::BTreeMap;

use bitvec::vec::BitVec;

use crate::alias::{Item, Requirement, State};

pub type LocationId = usize;
pub type WorldId = usize;

#[derive(Debug, Clone, Default)]
pub struct Location {
    pub id: LocationId,
    pub world_id: WorldId,
    pub current_item: Item,
}

#[derive(Debug, Clone, Default)]
pub struct SphereLocation {
    pub location: Location,
    pub sphere: usize,
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        return (self.id == other.id) & (self.world_id == other.world_id);
    }
}

#[derive(Debug, Clone)]
pub struct World {
    pub world_id: WorldId,
    pub item_pool: Vec<Item>,
    pub available_locations: Vec<Location>,
    pub locked_locations: BTreeMap<Requirement, Vec<Location>>,
    pub player_state: State,
}

// To Generate Playthrough crap.
#[derive(Debug)]
pub struct MetaData {
    world_id: WorldId,
    access_mapping: Vec<Vec<usize>>,
    // Indexed by Location Id
    region: Vec<usize>,
    // Region by Location Id
    items: BTreeMap<BitVec, BitVec>,
    location_name: Vec<String>,
    // Indexed by Location Id
    settings: Vec<String>,
}
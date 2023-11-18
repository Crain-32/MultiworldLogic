use crate::alias::Item;
use crate::world::{Location, World};

// evaluate_requirement moved to the requirement crate

// Basically this entire module gets nuked by the linear scan change.

pub fn explore(
    worlds: Vec<World>, items: Vec<Item>,
    area: Area, exits_to_try: Vec<Exit>,
    locations_to_try: Vec<Location>,
) -> Vec<Location> {
    // This also gets changed when we move to a linear scan
    todo!()
}

pub fn search(
    search_mode: String, worlds: Vec<World>,
    input_items: Vec<Item>, world_id: usize, global_search: bool
) -> Vec<Location> {
    // Another Linear scanning thing getting nuked.
    todo!()
}

pub fn get_accessible_location(
    worlds: &Vec<World>, assumed_items: &Vec<Item>,
    allowed_locations: &Vec<Location>, world_to_search: usize, global_search: bool
) -> Vec<Location> {
    todo!()
}

pub fn game_beatable(
    worlds: Vec<World>
) -> bool {
    todo!()
}

pub fn pare_down_playthrough(
    worlds: Vec<World>
) -> Vec<World> {
    todo!()
}

pub fn generate_playthrough(
    worlds: Vec<World>
) {
    todo!()
}

pub fn locations_reachable(
    worlds: Vec<World>, items: Vec<Item>,
    locations_to_check: Vec<Location>, world_to_search: usize, global_search: bool
) -> bool {
    todo!()
}

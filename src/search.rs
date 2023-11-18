use bitvec::vec::BitVec;

use crate::alias::Item;
use crate::world::{Location, World};

// evaluate_requirement moved to the requirement crate

// Basically this entire module gets nuked by the linear scan change.

pub fn get_accessible_location(
    worlds: &Vec<World>, world_to_search: usize, global_search: bool,
) -> Vec<Location> {
    if global_search {
        return check_multiworld(worlds);
    } else {
        return worlds.get(world_to_search).map(check_single_world).unwrap();
    }
}

fn check_single_world(world: World) -> Vec<Location> {
    let mut world_state = world.player_state.clone();
    let mut count_state = world.count_state.clone();

    return todo!();
}

fn check_multiworld(worlds: &Vec<World>) -> Vec<Location> {
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
    locations_to_check: Vec<Location>, world_to_search: usize, global_search: bool,
) -> bool {
    todo!()
}

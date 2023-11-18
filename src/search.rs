use bitvec::vec::BitVec;

use crate::alias::{Item, satisfies};
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

fn check_single_world(mut world: World) -> Vec<Location> {
    // Scan all available locations
    for requirement_to_check in world.locked_locations {
        if !satisfies(requirement_to_check.0, world.player_state) {
            continue;
        }
        match world.locked_locations.remove(&requirement_to_check.0) {
            None => { panic!("This should never be reached so this is bad")}
            Some(mut locations_to_add) => {world.available_locations.append(&mut locations_to_add)}
        }
    }

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

use log::{debug, trace};

use crate::alias::{Item, satisfies};
use crate::world::{Location, SphereLocation, World};

// evaluate_requirement moved to the requirement crate

// Basically this entire module gets nuked by the linear scan change.

pub enum SearchError {
    FailedToFindEndGoal(String)
}

pub fn get_accessible_location(
    worlds: &mut Vec<World>, world_to_search: usize, global_search: bool,
) -> Vec<Location> {
    return if global_search {
        check_multiworld(worlds)
    } else {
        match worlds.get_mut(world_to_search) {
            None => { Vec::new() }
            Some(world) => { check_single_world(world) }
        }
    };
}

fn check_single_world(mut world: &mut World) -> Vec<Location> {
    let mut newable_available_locations: Vec<Location> = Vec::new();
    for requirement_to_check in world.locked_locations.clone() {
        debug!("Executing a Location Scan");
        if !satisfies(&requirement_to_check.0, &world.player_state) {
            continue;
        }
        match world.locked_locations.remove(&requirement_to_check.0) {
            None => { trace!("Failed to find Locked Locations") }
            Some(mut locations_to_add) => {
                debug!("Found {:?} Locations for Requirement {:?}", locations_to_add.len(), requirement_to_check.0);
                newable_available_locations.append(&mut locations_to_add)
            }
        }
    }
    debug!("Locations Found {}", newable_available_locations.len());
    return newable_available_locations;
}

fn check_multiworld(worlds: &mut Vec<World>) -> Vec<Location> {
    let mut multiworld_locations: Vec<Location> = Vec::new();
    for world in worlds {
        multiworld_locations.append(&mut check_single_world(world));
    }
    return multiworld_locations;
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
    &mut worlds: &mut Vec<World>
) -> Result<Vec<SphereLocation>, SearchError>{
    todo!()
}

pub fn locations_reachable(
    worlds: Vec<World>, items: Vec<Item>,
    locations_to_check: Vec<Location>, world_to_search: usize, global_search: bool,
) -> bool {
    todo!()
}


#[cfg(test)]
mod tests {
    use test_context::test_context;

    use crate::util::test::world::SingleWorldContext;

    use super::*;

    #[test_context(SingleWorldContext)]
    #[test]
    fn single_world_test_no_counting(ctx: &mut SingleWorldContext) {
        let locations_found = check_single_world(&mut ctx.world);
        let expect_response = vec![Location { id: 0,  world_id: 0, current_item: Item::default() }, Location { id: 1,  world_id: 0, current_item: Item::default() }];
        expect_response.iter().for_each(|x| debug_assert!(locations_found.contains(x)));
    }

    #[test_context(SingleWorldContext)]
    #[test]
    fn single_world_test_no_counting_second_pass_returns_nothing(ctx: &mut SingleWorldContext) {
        check_single_world(&mut ctx.world);
        let second_pass = check_single_world(&mut ctx.world);
        debug_assert_eq!(0, second_pass.len());
    }
}
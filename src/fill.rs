use bitvec::vec::BitVec;

use crate::alias::{Item, State};
use crate::extra;
use crate::search::{game_beatable, get_accessible_location};
use crate::world::{Location, World};

pub fn fill(mut worlds: Vec<World>, random_state: Option<u8>) -> Vec<World> {
    worlds = place_hardcoded_items(worlds);

    worlds = determine_major_items(worlds, random_state);

    worlds = place_race_mode_items(worlds, random_state);

    let mut item_pool = get_item_pool(&worlds);

    let mut locations: Vec<Location> = vec![];
        // get_location_pool(&worlds)
        // .iter()
        // // Should only be logical locations
        // .filter(|location| todo!())
        // .collect();

    let mut major_items: Vec<Item> = item_pool
        .iter()
        // Filters to Major Items
        .filter(|item| todo!())
        // Removes Item from World Pool
        // Might be better to handle post collect in order to bulk the operation?
        .map(|item| todo!())
        .collect();

    if major_items.len() > locations.len() {
        println!("Major Items amount: {}", major_items.len());
        println!("Logical Locations: {}", locations.len());
        println!("Please Enable more Spots for Major Items!");

        if true {
            locations.iter().for_each(|loc| println!("{:?}", loc));
            major_items.iter().for_each(|item| println!("{:?}", item));
            panic!("Invalid amount of Items")
        }
    }

    // Borrow checker doesn't like this, I'm not touching it right now.
    // worlds = assumed_fill(&worlds.clone(), &major_items, &locations, random_state, 0, true);
    //
    // if !game_beatable(worlds) {
    //     worlds.iter().for_each(extra::dump_world);
    //     panic!("The game is not beatable!");
    // } else {
    //     println!("Dumping World Graph");
    //     worlds.iter().for_each(extra::dump_world);
    // }

    // Should be worlds, but "Values shouldn't be moved"
    todo!();
}

pub fn place_hardcoded_items(mut worlds: Vec<World>) -> Vec<World> {
    // Likely can be removed since the final sink should be in the World Graph provided
    // We no longer set it
    todo!()
}

pub fn determine_major_items(worlds: Vec<World>, random_state: Option<u8>) -> Vec<World> {
    // This should be based on Category Settings, not an actual check.
    // AKA precompute this
    todo!()
}

pub fn place_race_mode_items(worlds: Vec<World>, random_state: Option<u8>) -> Vec<World> {
    // This is also just incorrect. The world graph should be providing a way to find the
    // Subgraph of required placements and items for them that we can reference.
    //
    // ALT Idea - Provide a list of locations and list of items, randomize between them.
    // We can have the client provide more information than just the world graph and settings.
    todo!()
}

pub fn handle_region_items(worlds: Vec<World>, random_state: Option<u8>) -> Vec<World> {
    // Renamed from handle_dungeon_items to be more generic
    todo!()
}

pub fn assumed_fill(
    mut worlds: &Vec<World>, mut logical_items: &Vec<Item>,
    mut logical_locations: &Vec<Location>, random_state: Option<u8>,
    world_id: usize, global_search: bool
) -> Vec<World> {
    if logical_items.len() > logical_locations.len() {
        panic!(
            "Tried to place {} Items for {} Locations",
            logical_items.len(),
            logical_locations.len()
        );
    }

    if world_id >= worlds.len() {
        panic!(
            "Tried to select world id {} with only {} worlds!",
            world_id,
            worlds.len()
        )
    }

    let mut retries: i32 = 5;
    let mut unsuccessful_placement = true;

    while unsuccessful_placement {
        if retries == 0 {
            println!("Ran out of Retries, attempting to forward fill {} locations", logical_locations.len());
            // worlds = &forward_fill_until_more_free_space(
            //     &worlds,
            //     logical_items,
            //     logical_locations,
            //     random_state,
            //     3,
            // );
            retries = 5;
            continue;
        }
        retries = -1;
        unsuccessful_placement = false;
        // Shuffle Logical Items
        let mut rollback: Vec<Location> = Vec::new();
        // while logical_items.len() > 0 {
        //     let mut item_pool = get_item_pool(worlds);
        //     let mut next_item = logical_items.pop().unwrap();
        //     let mut items_not_placed: Vec<Item> = logical_items.iter().cloned().collect();
        //
        //     item_pool.iter().clone().for_each(move |x| {
        //         items_not_placed.push(x.clone())
        //     });
        //
        //     let mut accessible_locations = get_accessible_location(
        //         worlds,
        //         &items_not_placed,
        //         logical_locations,
        //         world_id,
        //         true
        //     );
        //
        //     if accessible_locations.len() == 0 {
        //         println!("No Accessible Locations to place {}. Remaining Attempts this cycle: {}", next_item.world_id, retries);
        //         for mut location in logical_locations {
        //             // Remove location from the world
        //             if location.current_item.value.any() {
        //                 logical_items.push(location.current_item.to_owned());
        //             }
        //             location.current_item = Item { value: BitVec::new(), world_id }
        //             // replace location we removed
        //         }
        //         logical_items.push(next_item);
        //         rollback.clear();
        //         unsuccessful_placement = true;
        //         break;
        //     }
        //
        //     // Some sort of predicate for "Don't place items into dumb chains"
        //     // python ver
        //     // if next_item.chart_for_sunken_treasure {
        //     //  pass
        //     // }
        //
        //     // shuffle locations
        //     let mut location = accessible_locations.pop().unwrap();
        //     _ = worlds.get(location.world_id); // Remove location;
        //     _ = worlds.get(location.world_id); // Remove location from the area chain
        //
        //     location.current_item = next_item;
        //     rollback.push(location.clone());
        //     _ = worlds.get(location.world_id); // Add Location to world cache
        //     _ = worlds.get(location.world_id); // Add Location to area dict
        // }
    }
    todo!();
}

pub fn forward_fill_until_more_free_space(
    mut worlds: &Vec<World>, items_to_place: &Vec<Item>,
    mut input_locations: &Vec<Location>, random_state: Option<u8>,
    open_locations: usize) -> Vec<World> {
    let mut allowed_locations: Vec<Location> = vec![];

    if allowed_locations.len() < items_to_place.len() {
        panic!("Tried to place {} items for {} locations!", items_to_place.len(), allowed_locations.len());
    }

    let mut accessible_locations = get_accessible_location(worlds, &Vec::new(), &allowed_locations, 0, true);

    if accessible_locations.is_empty() {
        panic!("No accessible locations to place items!");
    }

    let mut forward_placed_items: Vec<Item> = Vec::new();

    while accessible_locations.len() < (open_locations * worlds.len()) {
        let mut placeable_locations: Vec<Location> = vec![];
        // shuffle items_to_place
        let original_size = forward_placed_items.len();
        for item in items_to_place {
            forward_placed_items.push(item.clone());
            let mut access_locs = get_accessible_location(worlds, &forward_placed_items, &placeable_locations, 0, true);
            if access_locs.is_empty() {
                let mut loc = Location::default(); // This is random location from placeable_locations
                // placeable_locations.remove(loc);
                println!("Item: {:?}, World {}, opened up more space", item, item.world_id);
                _ = worlds.get(loc.world_id); // remove location
                _ = worlds.get(loc.world_id); // remove  from area entries
                loc.current_item = item.clone();

                _ = worlds.get(loc.world_id); // push loc into world cache
                _ = worlds.get(loc.world_id); // push loc into area entries
            } else {
                forward_placed_items.remove(forward_placed_items.iter().position(|f_item| f_item.value.eq(&item.value)).unwrap());
            }
        }
        println!("Size of original {} vs current {}", original_size, forward_placed_items.len());

        if original_size == forward_placed_items.len() {
            worlds.iter().for_each(|world| extra::dump_world(world));
            panic!("No items opened up the forward fill")
        }
        accessible_locations = get_accessible_location(worlds, &forward_placed_items, &allowed_locations, 0, true);
    }

    // Remove all items in forward_placed_items from items_to_place

    todo!();
}

// See "place_race_mode_items"
pub fn fast_fill(
    mut locations: Vec<Location>, items: Vec<Item>,
    random_state: Option<u8>,
) -> (Vec<Location>, Vec<State>) {
    todo!()
}

pub fn fill_the_rest(
    mut locations: Vec<Location>, items: Vec<Item>,
    random_state: Option<u8>,
) -> Option<Item> {
    // See "fast_fill"
    todo!()
}

pub fn get_item_pool(
    worlds: &Vec<World>
) -> Vec<Item> {
    // Likely isn't needed? Unsure
    todo!()
}

pub fn get_location_pool(
    worlds: &Vec<World>
) -> Vec<Location> {
    // See "get_item_pool"
    todo!()
}



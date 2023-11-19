use log::{debug, error, info, trace};
use rand::Rng;

use crate::alias::Item;
use crate::fill::FillError::{FailedToRandomize, LackOfLocations};
use crate::search::{generate_playthrough, SearchError};
use crate::world::{Location, SphereLocation, World};

pub struct FilledWorld {
    filled_worlds: Vec<World>,
    spheres: Vec<SphereLocation>,
}

pub enum FillError {
    LackOfLocations(String),
    FailedToRandomize(String),
}

/// `adjusted_worlds` should be parsed to handle the randomized entrances BEFORE fill is called.
/// Likewise any location/region pruning should be handled as well.
/// And any "plando" locations.
pub fn fill_logical_locations(mut adjusted_worlds: Vec<World>, random_state: Box<dyn Rng>, with_playthrough: bool) -> Result<FilledWorld, FillError> {
    let mut item_pool = determine_required_items(&mut adjusted_worlds);

    let mut location_amount: usize = 0;
    adjusted_worlds.iter().for_each(|world|
        world.locked_locations.iter().for_each(|(key, val)| location_amount += val.len()));

    if item_pool.len() > location_amount {
        info!("Major Items amount: {}", item_pool.len());
        info!("Logical Location Amount: {}", location_amount);
        error!("Please Enable more Spots for Major Items!");
        item_pool.iter().for_each(|item| debug!("{:?}", item));
        adjusted_worlds.iter().for_each(|world|
            world.locked_locations.iter().for_each(
                |(key, val)| val.iter().for_each(
                    |loc| debug!("Location Id {:?} | World Id {:?}", loc.id, loc.world_id)
                )
            )
        );
        return Err(LackOfLocations("Invalid amount of Items".parse().unwrap()));
    }
    handle_region_items(&mut adjusted_worlds, &random_state);
    debug!("Forward Filling {:?} worlds, with {:?} Items", adjusted_worlds.len(), item_pool.len());
    forward_fill(&mut adjusted_worlds, &mut item_pool, &random_state);

    let play_through = match generate_playthrough(&mut adjusted_worlds) {
        Ok(spheres) if with_playthrough => { Ok(spheres) }
        Ok(_) => { Ok(Vec::new()) }
        Err(SearchError::FailedToFindEndGoal(reason)) => { Err(FailedToRandomize(reason)) }
    };

    return match play_through {
        Ok(spheres) => { Ok(FilledWorld { filled_worlds: adjusted_worlds, spheres }) }
        Err(err) => { Err(err) }
    };
}

fn determine_required_items(&mut adjusted_worlds: &mut Vec<World>) -> Vec<Item> {
    // We can't determine all the required items till after the entrances are randomized.
    todo!()
}

pub fn handle_region_items(&mut worlds: &mut Vec<World>, random_state: &Box<dyn Rng>) -> Vec<World> {
    // Renamed from handle_dungeon_items to be more generic
    todo!()
}

pub fn forward_fill(
    mut worlds: &mut Vec<World>, mut logical_items: &Vec<Item>, random_state: &Box<dyn Rng>,
) -> Vec<World> {
    while logical_items.len() > 0 {}
    todo!();
}

pub fn forward_fill_until_more_free_space(
    mut worlds: &Vec<World>, items_to_place: &Vec<Item>,
    mut input_locations: &Vec<Location>, &random_state: &Box<dyn Rng>,
    open_locations: usize) -> Vec<World> {
    let mut allowed_locations: Vec<Location> = vec![];

    if allowed_locations.len() < items_to_place.len() {
        panic!("Tried to place {} items for {} locations!", items_to_place.len(), allowed_locations.len());
    }

    // let mut accessible_locations = get_accessible_location(worlds, &Vec::new(), &allowed_locations, 0, true);

    // if accessible_locations.is_empty() {
    //     panic!("No accessible locations to place items!");
    // }
    //
    // let mut forward_placed_items: Vec<Item> = Vec::new();
    //
    // while accessible_locations.len() < (open_locations * worlds.len()) {
    //     let mut placeable_locations: Vec<Location> = vec![];
    //     // shuffle items_to_place
    //     let original_size = forward_placed_items.len();
    //     for item in items_to_place {
    //         forward_placed_items.push(item.clone());
    //         let mut access_locs = get_accessible_location(worlds, &forward_placed_items, &placeable_locations, 0, true);
    //         if access_locs.is_empty() {
    //             let mut loc = Location::default(); // This is random location from placeable_locations
    //             // placeable_locations.remove(loc);
    //             println!("Item: {:?}, World {}, opened up more space", item, item.world_id);
    //             _ = worlds.get(loc.world_id); // remove location
    //             _ = worlds.get(loc.world_id); // remove  from area entries
    //             loc.current_item = item.clone();
    //
    //             _ = worlds.get(loc.world_id); // push loc into world cache
    //             _ = worlds.get(loc.world_id); // push loc into area entries
    //         } else {
    //             forward_placed_items.remove(forward_placed_items.iter().position(|f_item| f_item.value.eq(&item.value)).unwrap());
    //         }
    //     }
    //     println!("Size of original {} vs current {}", original_size, forward_placed_items.len());
    //
    //     if original_size == forward_placed_items.len() {
    //         worlds.iter().for_each(|world| extra::dump_world(world));
    //         panic!("No items opened up the forward fill")
    //     }
    //     accessible_locations = get_accessible_location(worlds, &forward_placed_items, &allowed_locations, 0, true);
    // }

    // Remove all items in forward_placed_items from items_to_place

    todo!();
}

pub fn fill_the_rest(
    mut locations: Vec<Location>, items: Vec<Item>,
    random_state: &Box<dyn Rng>,
) -> Option<Item> {
    // See "fast_fill"
    todo!()
}
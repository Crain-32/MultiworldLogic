use crate::world::world::World;

pub mod fill {
    use std::collections::hash_map::RandomState;
    use crate::alias::alias::State;
    use crate::world::world::{Location, World};

    pub fn fill(worlds: Vec<World>, random_state: None) -> Some(String) {
        todo!()
    }

    pub fn place_hardcoded_items(worlds: Vec<World>) -> Vec<World> {
        todo!()
    }

    pub fn determine_major_items(worlds: Vec<World>, random_state: None) -> Vec<World> {
        todo!()
    }

    pub fn place_race_mode_items(worlds: Vec<World>, random_state: None) -> Vec<World> {
        todo!()
    }
    pub fn handle_dungeon_items(worlds: Vec<World>, random_state: None) -> Vec<World> {
        todo!()
    }

    pub fn assumed_fill(
        worlds: Vec<World>, logical_items: Vec<State>,
        logical_locations: Vec<Location>, random_state: None,
    world_id: u8) -> Vec<World> {
        todo!()
    }

    pub fn forward_fill_until_more_free_space(
    worlds: Vec<World>, items_to_place: Vec<State>,
        input_locations: Vec<Location>, random_state: None,
        open_locations: u8) -> Vec<World> {
        todo!()
    }

    pub fn fast_fill(
        locations: Vec<Location>, items: Vec<State>,
        random_state: None
    ) -> (Vec<Location>, Vec<State>) {
        todo!()
    }

    pub fn fill_the_rest(
        locations: Vec<Location>, items: Vec<State>,
        random_state: None
    ) -> None {
        todo!()
    }

    pub fn get_item_pool(
        worlds: Vec<World>
    ) -> Vec<State> {
        todo!()
    }

    pub fn get_location_pool(
        worlds: Vec<World>
    ) -> Vec<Location> {
        todo!()
    }
}


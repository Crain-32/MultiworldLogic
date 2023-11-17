pub mod search {
    use crate::alias::alias::{Requirement, State};
    use crate::world::world::{Area, Exit, Location, World};

    pub fn evaluate_requirement(
        world: World, requirement: Requirement,
    ) -> bool {
        todo!()
    }

    pub fn explore(
        worlds: Vec<World>, items: Vec<State>,
        area: Area, exits_to_try: Vec<Exit>,
        locations_to_try: Vec<Location>,
    ) -> Vec<Location> {
        todo!()
    }

    pub fn search(
        search_mode: String, worlds: Vec<World>,
        input_items: Vec<State>, world_id: u8,
    ) -> Vec<Location> {
        todo!()
    }

    pub fn get_accessible_location(
        worlds: Vec<World>, assumed_items: Vec<State>,
        allowed_locations: Vec<Location>, world_to_search: u8,
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
        worlds: Vec<World>, items: Vec<State>,
        locations_to_check: Vec<Location>, world_to_search: u8,
    ) -> bool {
        todo!()
    }
}
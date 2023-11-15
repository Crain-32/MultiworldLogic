pub mod requirements {
    use crate::alias::alias::{Requirement, Item, State};

    fn evaluate_requirement(requirement: Requirement, state: State) -> bool {
        return requirement.contains(&state);
    }

    fn needs_item(requirement: Requirement, item: Item) -> bool {
        return requirement.contains(&item);
    }
    // # https://docs.rs/jsonc-parser/0.23.0/jsonc_parser/
}
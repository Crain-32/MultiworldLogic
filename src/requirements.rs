pub mod requirements {

    pub enum ReqType {
        Or,
        And,
        Not,
        HasItem,
        Count, // Unsure how to handle this
        CanAccess,
        Setting, // Could be pre-evaluated before the first pass to prune things.
        Macro,
        None
    }

    use std::panic::panic_any;
    use crate::alias::alias::{Requirement, Item, State};

    fn evaluate_requirement(requirement: Requirement, state: State) -> bool {
        return requirement.contains(&state);
    }

    fn needs_item(requirement: Requirement, item: Item) -> bool {
        return requirement.contains(&item);
    }

    /// Applies `requirement_two` to `requirement_one` based on the `req_type`
    ///
    /// # Notes
    /// Currently the only value [`ReqType`] values are
    /// - [`ReqType::Or`] ORs the two values
    /// - [`ReqType::And`] ANDs the two values
    /// - [`ReqType::Not`] NOTs `requirement_two` then ANDs it to `requirement_one`
    /// - [`ReqType::None`] Returns `requirement_one`
    fn combine_requirements(requirement_one: Requirement, requirement_two: Requirement, req_type: ReqType) -> Requirement {
        return match req_type {
            ReqType::Or => requirement_one | requirement_two,
            ReqType::And => requirement_one & requirement_two,
            ReqType::Not => requirement_one & !requirement_two,
            ReqType::None => requirement_one,
            _ => {
                print!("Invalid Req Type Provided {}", stringify!(req_type));
                requirement_one
            }
        }
    }
    // # https://docs.rs/jsonc-parser/0.23.0/jsonc_parser/
}
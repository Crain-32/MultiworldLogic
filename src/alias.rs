use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::ops::{BitAnd, BitAndAssign};

use bitvec::prelude::BitVec;
use log::{debug, trace};

#[derive(Debug, Clone, Default)]
pub struct Item {
    pub value: BitVec,
    pub is_count: bool,
    pub world_id: usize,
}

// impl Display for Item {
//     // No idea what we do here
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }

// impl<T> Add<T> for Item where T: Deref<Target=BitVec> {
//     type Output = T;
//
//     fn add(self, rhs: T) -> T {
//         return rhs.bitor(self.value);
//     }
// }

#[derive(Debug, Clone, Default)]
pub struct State {
    pub state: BitVec,
    pub item_count: BTreeMap<BitVec, usize>,
}

#[derive(Debug, Clone, Default, Ord)]
pub struct Requirement {
    pub requirement: BitVec,
    pub needs_count: Option<(BitVec, usize)>,
}

impl Eq for Requirement {}

impl PartialEq<Self> for Requirement {
    fn eq(&self, other: &Self) -> bool {
        return self.requirement.eq(&other.requirement);
    }
}

impl PartialOrd<Self> for Requirement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return self.requirement.partial_cmp(&other.requirement);
    }
}

// impl Ord for Requirement {
//     fn cmp(&self, other: &Self) -> Ordering {
//         return self.requirement.cmp(&other.requirement);
//     }
//
//     fn max(self, other: Self) -> Self where Self: Sized {
//         return match self.requirement.max(other.requirement.clone()).eq(&self.requirement) {
//             true => {self}
//             false => {other}
//         }
//     }
//
//     fn min(self, other: Self) -> Self where Self: Sized {
//         todo!()
//     }
//
//     fn clamp(self, min: Self, max: Self) -> Self where Self: Sized, Self: PartialOrd {
//         todo!()
//     }
// }

pub fn satisfies(requirement: &Requirement, state: &State) -> bool {
    return match &requirement.needs_count {
        None => {
            trace!("None");
            bitvec_satisfies(&requirement.requirement, &state.state)
        }
        Some(count_req) => {
            trace!("Some, {:?} : {:?}", count_req.0, count_req.1);
            bitvec_satisfies(&requirement.requirement, &state.state) &
                (state.item_count.get(&count_req.0).unwrap() >= &count_req.1)
        }
    };
}

fn bitvec_satisfies(first: &BitVec, state: &BitVec) -> bool {
    let ahhhh = first.clone();
    return (ahhhh & state).eq(first);
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use bitvec::prelude::*;
    use log::info;

    use crate::alias::{Requirement, State};

    use super::*;

    #[test]
    fn satisfies_no_count() {
        let empty_req = bitvec![0,0,0];
        let other_req = bitvec![0,0,1];

        let req = Requirement { requirement: empty_req, needs_count: None };
        let state = State { state: other_req, item_count: BTreeMap::default() };
        debug_assert!(satisfies(&req, &state));
    }


    #[test]
    fn fails_satisfies_no_count() {
        let empty_req = bitvec![0,0,0];
        let other_req = bitvec![0,0,1];

        let req = Requirement { requirement: other_req, needs_count: None };
        let state = State { state: empty_req, item_count: BTreeMap::default() };
        let result = satisfies(&req, &state);
        info!("Result: {:?}", result);
        debug_assert!(!result);
    }

    #[test]
    fn satisfies_with_count() {
        let empty_req = bitvec![0,0,0];
        let state_vec = empty_req.clone();
    }
}
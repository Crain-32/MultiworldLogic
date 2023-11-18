use std::collections::BTreeMap;

use bitvec::prelude::BitVec;
use derive_more::Display;

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

#[derive(Debug, Display, Clone)]
pub struct State {
    state: BitVec,
    item_count: BTreeMap<BitVec, usize>,
}

#[derive(Debug, Clone, Default, Display)]
pub struct Requirement {
    requirement: BitVec,
    needs_count: Option<(BitVec, usize)>,
}

pub fn satisfies(requirement: Requirement, state: State) -> bool {
    return match requirement.needs_count {
        None => { bitvec_satisfies(requirement.requirement, state.state) }
        Some(count_req) => {
            bitvec_satisfies(requirement.requirement, state.state) &
                (state.item_count.get(&*count_req.0).unwrap() == count_req.1)
        }
    };
}

fn bitvec_satisfies(first: BitVec, second: BitVec) -> bool {
    let mut value = first.clone();
    return ((value ^ second) & first) == 0;
}


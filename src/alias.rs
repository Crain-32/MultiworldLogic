use bitvec::prelude::BitVec;
use derive_more::{Deref, DerefMut, Display};

#[derive(Debug, Clone, Default)]
pub struct Item {
    pub value: BitVec,
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

#[derive(Deref, DerefMut)]
#[derive(Debug, Display)]
#[deref(forward)]
pub struct State(BitVec);

#[derive(Deref, DerefMut)]
#[derive(Debug, Clone, Default, Display)]
#[deref(forward)]
pub struct Requirement(BitVec);




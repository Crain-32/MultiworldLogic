pub mod alias {
    use std::ops::{Add, BitOr, Deref};
    use bitvec::prelude::BitVec;
    use derive_more::{Deref, DerefMut};

    pub struct Item {
        value: BitVec,
        world_id: u8
    }

    impl <T> Add<T> for Item where T: Deref<Target = BitVec> {
        type Output = T;

        fn add(self, rhs: T) -> Self::Output {
            return rhs.bitor(self.value);
        }
    }

    #[derive(Deref, DerefMut)]
    #[deref(forward)]
    pub struct State(BitVec);
    #[derive(Deref, DerefMut)]
    #[deref(forward)]
    pub struct Requirement(BitVec);
}



#![forbid(unsafe_code)]
#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::todo,
    clippy::unimplemented
)]
// allowing use_self lints due to a bug where proc-macro's (such as serde::Serialize) can trigger it to hinted on type definitions
#![allow(clippy::use_self)]

use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;

mod alias;
mod extra;
mod file;
mod fill;
mod requirements;
mod search;
mod world;
mod util;


pub fn main() -> bool {
    let mut rand_state: dyn Rng = Pcg64Mcg::seed_from_u64(0);
}


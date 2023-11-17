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
mod alias;
mod extra;
mod file;
mod fill;
mod requirements;
mod search;
mod world;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

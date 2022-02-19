//! # My Crate
//! docs for crates.io for this lib.rs
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.


#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_macros,
    unused_assignments,
    unused_mut,
)]


/// Documentation for crates.io
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// `
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_one() {
        assert_eq!(3, add_one(2));
    }
}



pub fn add_two(x: i32) -> i32 {
    x + 2
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two() {
        assert_eq!(4, add_two(2));
    }
}

fn main() {
    unimplemented!();
}

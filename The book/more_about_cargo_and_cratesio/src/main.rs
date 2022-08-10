//! # My Crate
//!
//! `my_crate` is a collection of utilities...
//! This comment is to document the item that contains
//! this comment, in this case, it's main.rs
//!

use more_about_cargo_and_cratesio::test_doc;
use more_about_cargo_and_cratesio::test_module;

fn main() {
    println!("Hello, world!");
}

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
///
/// # Panics
///
/// # Errors
///
/// # Safety
///
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

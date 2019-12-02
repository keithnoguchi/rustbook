// SPDX-License-Identifier: GPL-2.0
//! # rustbox
//!
//! `rustbox` is a collection of examples provided in
//! [the Rust Programming Language] with [table driven] unit tests.
//!
//! [the rust programming language]: https://doc.rust-lang.org/stable/book/
//! [table driven]: https://dave.cheney.net/2019/05/07/prefer-table-driven-tests
//!
/// Adds two to the number given.
///
pub use test::add_two;

mod array;
mod borrow;
mod boxed;
mod first;
mod fs;
mod futures;
mod generic;
mod hashmap;
mod lifetime;
mod ptr;
mod second;
mod simple;
mod stream;
mod string;
mod test;
mod vector;

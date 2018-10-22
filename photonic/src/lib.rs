#![feature(self_struct_ctor)]
#![feature(cell_update)]

#[cfg(test)]
#[macro_use]
extern crate assert_approx_eq;
extern crate ezing;
#[macro_use]
extern crate failure;
extern crate scarlet;

pub mod core;
pub mod buffer;
pub mod color;
pub mod utils;
pub mod math;
pub mod attributes;
pub mod inspection;

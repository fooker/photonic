#![feature(self_struct_ctor)]
#![feature(cell_update)]
#![feature(duration_float)]

#[cfg(test)]
#[macro_use]
extern crate assert_approx_eq;
#[macro_use]
extern crate failure;
extern crate scarlet;

pub mod core;
pub mod buffer;
pub mod color;
pub mod math;
pub mod attributes;
pub mod inspection;

#![feature(self_struct_ctor)]
#![feature(cell_update)]
#![feature(duration_float)]
#![feature(fnbox)]

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
pub mod utils;
pub mod values;
pub mod animation;
pub mod trigger;
pub mod inspection;

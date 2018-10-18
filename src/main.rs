#![feature(self_struct_ctor)]

#[cfg(test)]
#[macro_use]
extern crate assert_approx_eq;
extern crate ezing;
#[macro_use]
extern crate failure;
extern crate num;
extern crate rand;
extern crate scarlet;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use core::*;
use std::thread;
use std::time::{Duration, Instant};

mod core;
mod buffer;
mod color;
mod utils;
mod math;
mod nodes;
mod outputs;
mod services;
mod attributes;
mod config;

fn main() {
    let config = config::load("config.yaml")
            .expect("Failed to load config");

    let mut root_node: Box<Node> = config.into();

    let mut output = outputs::console::ConsoleOutput::new();

    let mut last = Instant::now();
    loop {
        let curr = Instant::now();

        // Update all manager
        let duration = curr - last;
        root_node.update(duration);

        {
            let root_renderer = root_node.render();
            output.render(root_renderer.as_ref());
        }

        // Remember when frame started
        // TODO: Print render time and other FPS stats
        last = curr;

        // Sleep until it's time to render next frame
        let next = curr + Duration::from_secs(1) / FPS;
        let curr = Instant::now();
        if next > curr {
            thread::sleep(next - curr);
        }
    }
}

const FPS: u32 = 60;

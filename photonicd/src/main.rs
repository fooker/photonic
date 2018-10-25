#![feature(self_struct_ctor)]

#[macro_use]
extern crate failure;
extern crate num;
#[macro_use]
extern crate photonic_derive;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use photonic::attributes::*;
use photonic::core::*;
use std::rc::Rc;
use std::thread;
use std::time::{Duration, Instant};

mod nodes;
mod outputs;
mod config;
mod api;


fn main() {
    let config = config::load("config.yaml")
            .expect("Failed to load config");

    let mut root_node: Box<Node> = config.into();

    let mut output = outputs::console::ConsoleOutput::new();

    let remote = api::serve(api::Config {
        address: "localhost:1337",
    }, &*root_node);

    let mut last = Instant::now();
    loop {
        let curr = Instant::now();

        // Update all manager
        let duration = curr - last;
        root_node.update(&duration);

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

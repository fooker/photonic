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

use photonic::core::*;
use std::thread;
use std::time::{Duration, Instant};

mod nodes;
mod outputs;
mod config;


fn dump(ident: usize, name: &str, node: &Node) {
    println!("{}{}: {}", "  ".repeat(ident), node.class(), name);

    for attr in node.attrs().iter() {
        println!("{}# {} = {}", "  ".repeat(ident), attr.name, attr.attr.get());
    }

    for node in node.nodes().iter() {
        dump(ident + 1, node.name, node.node);
    }
}

fn main() {
    let config = config::load("config.yaml")
            .expect("Failed to load config");

    let mut root_node: Box<Node> = config.into();
//    dump(0, "root", root_node.as_ref());

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

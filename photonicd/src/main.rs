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
use photonic::attributes::*;
use std::thread;
use std::time::{Duration, Instant};
use std::rc::Rc;

mod nodes;
mod outputs;
mod config;


fn collect_dynamic_attributes(attributes: &mut Vec<Rc<Box<DynamicAttribute>>>, node: &Node) {
    for attr in node.attributes() {
        if let Attribute::Dynamic(dynamic) = attr.as_ref() {
            attributes.push(dynamic.clone());
        }
    }

    for node in node.children() {
        collect_dynamic_attributes(attributes, node.as_ref());
    }
}

fn main() {
    let config = config::load("config.yaml")
            .expect("Failed to load config");

    let mut root_node: Box<Node> = config.into();

    let mut attrs = Vec::new();
    collect_dynamic_attributes(&mut attrs, root_node.as_ref());

    for attr in attrs {
        println!("{} = {}", attr.name(), attr.value());
    }

    let mut output = outputs::console::ConsoleOutput::new();

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

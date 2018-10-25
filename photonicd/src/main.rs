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
extern crate clap;

use photonic::attributes::*;
use photonic::core::*;
use std::rc::Rc;
use std::thread;
use std::time::{Duration, Instant};
use clap::{App, Arg, ArgMatches, SubCommand};

mod nodes;
mod outputs;
mod config;
mod api;


fn main() {
    let matches = App::new("Photonic Daemon")
            .author("Dustin Frisch <fooker@lab.sh>")
            .about("Shine on you crazy diamond")
            .arg(Arg::with_name("listen")
                    .long("listen")
                    .value_name("HOST:PORT")
                    .takes_value(true)
                    .default_value("localhost:1337")
                    .help("Host and port to bind the remote control interface to"))
            .arg(Arg::with_name("config")
                    .long("config")
                    .value_name("FILE")
                    .takes_value(true)
                    .default_value("config.yaml")
                    .help("Path to the config file"))
            .arg(Arg::with_name("fps")
                    .long("fps")
                    .value_name("NUMBER")
                    .takes_value(true)
                    .default_value("60")
                    .help("Maximum FPS"))
            .get_matches();

    let config = config::load(matches.value_of("config").unwrap())
            .expect("Failed to load config");

    let mut root_node: Box<Node> = config.into();

    let mut output = outputs::console::ConsoleOutput::new();

    let remote = api::serve(api::Config {
        address: "localhost:1337",
    }, &*root_node);

    // Calculate maximum frame duration from FPS
    let frame_dur = Duration::from_secs(1) / matches.value_of("fps").unwrap().parse().unwrap();

    let mut frame_lst = Instant::now();
    loop {
        let frame_cur = Instant::now();

        // Update all manager
        let duration = frame_cur - frame_lst;
        root_node.update(&duration);

        {
            let root_renderer = root_node.render();
            output.render(root_renderer.as_ref());
        }

        // Remember when frame started
        // TODO: Print render time and other FPS stats
        frame_lst = frame_cur;

        // Sleep until it's time to render next frame
        let next = frame_cur + frame_dur;
        let curr = Instant::now();
        if next > curr {
            thread::sleep(next - curr);
        }
    }
}


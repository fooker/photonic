#![feature(self_struct_ctor)]
#![feature(duration_float)]

extern crate clap;
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
extern crate ezing;

use clap::{App, Arg, ArgMatches, SubCommand};
use photonic::values::*;
use photonic::core::*;
use self::utils::*;

mod nodes;
mod outputs;
mod config;
mod api;
mod utils;


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

    // FIXME: Make this configurable via CLI / depending on output
    let size: usize = 120;

    // Load the config and build a node tree from it
    let config = config::load(matches.value_of("config").unwrap())
            .expect("Failed to load config");

    let mut builder = config::Builder::with_size(size);
    let mut node = builder.build(&config)
            .expect("Failed to build node tree");

    // Build the output
    let mut output = outputs::console::ConsoleOutput::new(size, false);

    // Start the remote API
    let remote = api::serve(api::Config {
        address: matches.value_of("listen").unwrap().to_owned(),
    }, &*node);

    let mut stats = FrameStats::new();

    // Start main loop
    let fps: usize = matches.value_of("fps").unwrap().parse().unwrap();
    for duration in FrameTimer::new(fps) {
        // Update node tree
        node.update(&duration);

        // Render node tree to output
        output.render(node.render().as_ref());

        if let Some(stats) = stats.update(duration, fps) {
            eprintln!("Stats: min={:3.2}, max={:3.2}, avg={:3.2}", stats.min_fps(), stats.max_fps(), stats.avg_fps())
        }
    }
}


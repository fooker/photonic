#![feature(never_type)]

use std::fs::File;
use std::path::PathBuf;

use anyhow::{Error, format_err};
use clap::Clap;

use photonic_run::builder::Builder;
use photonic_run::config;
use std::net::SocketAddr;
use std::str::FromStr;

enum Interface {
    GRPC(SocketAddr),
    MQTT(String),
    Varlink(SocketAddr),
}

impl FromStr for Interface {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (k, v) = s.split_once(":")
            .ok_or(format_err!("Invalid interface format. TYPE:CONFIG expected."))?;

        return Ok(match k {
            "grpc" => Self::GRPC(v.parse()?),
            "mqtt" => Self::MQTT(v.parse()?),
            "varlink" => Self::Varlink(v.parse()?),
            _ => return Err(format_err!("Unknown interface type: {}", k))
        });
    }
}

#[derive(Clap)]
#[clap()]
struct CLI {
    #[clap(default_value = "scene.yaml")]
    scene: PathBuf,

    #[clap(short, long, default_value = "30")]
    fps: usize,

    #[clap(short, long)]
    interface: Vec<Interface>
}

#[tokio::main]
async fn main() -> Result<!, Error> {
    let cli = CLI::parse();

    let scene: config::Scene = serde_yaml::from_reader(File::open(&cli.scene)?)?;

    let (main, introspection) = Builder::build(scene)?;

    for interface in cli.interface {
        let interface = match interface {
            Interface::GRPC(addr) => photonic_grpc::GrpcInterface::bind(addr),
            Interface::MQTT(addr) => todo!(),
            Interface::Varlink(addr) => todo!(),
        };

        introspection.clone().serve(interface)?;
    }

    return main.run(cli.fps).await;
}

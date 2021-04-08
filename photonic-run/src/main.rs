#![allow(clippy::needless_return)]
#![feature(never_type)]

use std::fs::File;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::{format_err, Error, Result};
use clap::Clap;

use photonic_core::Introspection;
use photonic_run::builder::Builder;
use photonic_run::config;

enum Interface {
    Grpc(SocketAddr),
    Mqtt {
        host: String,
        port: u16,
        realm: String,
    },
    Varlink(SocketAddr),
}

// TODO: Let clap do the hard work
impl FromStr for Interface {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (k, v) = s
            .split_once(":")
            .ok_or_else(|| format_err!("Invalid interface format. TYPE:CONFIG expected."))?;

        return Ok(match k {
            "grpc" => Self::Grpc(v.parse()?),
            "mqtt" => {
                let (addr, realm) = v.split_once("@").ok_or_else(|| {
                    format_err!("Invalid MQTT interface format: HOST:PORT@REALM expected.")
                })?;

                let (host, port) = addr.split_once(":").ok_or_else(|| {
                    format_err!("Invalid MQTT interface format: HOST:PORT@REALM expected.")
                })?;

                Self::Mqtt {
                    host: host.to_owned(),
                    port: port.parse()?,
                    realm: realm.to_owned(),
                }
            }
            "varlink" => Self::Varlink(v.parse()?),
            _ => return Err(format_err!("Unknown interface type: {}", k)),
        });
    }
}

impl Interface {
    fn serve(self, introspection: Arc<Introspection>) -> Result<()> {
        return match self {
            Self::Grpc(addr) => introspection.serve(photonic_grpc::GrpcInterface::bind(addr)),
            Self::Mqtt { host, port, realm } => {
                introspection.serve(photonic_mqtt::MqttInterface::connect(host, port, realm)?)
            }
            Self::Varlink(_) => todo!(),
        };
    }
}

#[derive(Clap)]
#[clap()]
struct Args {
    #[clap(default_value = "scene.yaml")]
    scene: PathBuf,

    #[clap(short, long, default_value = "30")]
    fps: usize,

    #[clap(short, long)]
    interface: Vec<Interface>,
}

#[tokio::main]
async fn main() -> Result<!> {
    let args = Args::parse();

    let scene: config::Scene = serde_yaml::from_reader(File::open(&args.scene)?)?;

    let (main, introspection) = Builder::build(scene)?;

    for interface in args.interface {
        interface.serve(introspection.clone())?
    }

    return main.run(args.fps).await;
}

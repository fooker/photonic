#![feature(never_type)]

use std::fs::File;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{format_err, Error};
use clap::Clap;

use photonic_core::Introspection;
use photonic_run::builder::Builder;
use photonic_run::config;
use std::sync::Arc;

enum Interface {
    GRPC(SocketAddr),
    MQTT {
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
        let (k, v) = s.split_once(":").ok_or(format_err!(
            "Invalid interface format. TYPE:CONFIG expected."
        ))?;

        return Ok(match k {
            "grpc" => Self::GRPC(v.parse()?),
            "mqtt" => {
                let (addr, realm) = v.split_once("@").ok_or(format_err!(
                    "Invalid MQTT interface format: HOST:PORT@REALM expected."
                ))?;

                let (host, port) = addr.split_once(":").ok_or(format_err!(
                    "Invalid MQTT interface format: HOST:PORT@REALM expected."
                ))?;

                Self::MQTT {
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
    fn serve(self, introspection: Arc<Introspection>) -> Result<(), Error> {
        return match self {
            Self::GRPC(addr) => introspection.serve(photonic_grpc::GrpcInterface::bind(addr)),
            Self::MQTT { host, port, realm } => {
                introspection.serve(photonic_mqtt::MqttInterface::connect(host, port, realm)?)
            }
            Self::Varlink(addr) => todo!(),
        };
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
    interface: Vec<Interface>,
}

#[tokio::main]
async fn main() -> Result<!, Error> {
    let cli = CLI::parse();

    let scene: config::Scene = serde_yaml::from_reader(File::open(&cli.scene)?)?;

    let (main, introspection) = Builder::build(scene)?;

    for interface in cli.interface {
        interface.serve(introspection.clone())?
    }

    return main.run(cli.fps).await;
}

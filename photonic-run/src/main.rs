#![allow(clippy::needless_return)]

use std::ffi::OsStr;
use std::fs::File;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::{format_err, Error, Result};
use clap::Clap;

use photonic_core::Introspection;
use photonic_dyn::builder::{Builder, NodeBuilder};
use photonic_dyn::registry::CombinedOutputRegistry;
use photonic_dyn::{config, registry};

enum Interface {
    Grpc(SocketAddr),
    Mqtt { host: String, port: u16, realm: String },
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
            Self::Mqtt {
                host,
                port,
                realm,
            } => introspection.serve(photonic_mqtt::MqttInterface::connect(host, port, realm)?),
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

struct Registry;

impl registry::Registry for Registry {
    type Output = CombinedOutputRegistry<
        photonic_console::registry::Registry,
        photonic_ledstrip::registry::Registry,
    >;
    type Node = photonic_effects::registry::Registry;
    type BoundAttr = photonic_effects::registry::Registry;
    type UnboundAttr = photonic_effects::registry::Registry;
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let config: config::Scene = match args.scene.extension().and_then(OsStr::to_str) {
        Some("yaml") | Some("yml") => serde_yaml::from_reader(File::open(&args.scene)?)?,
        Some("json") => serde_json::from_reader(File::open(&args.scene)?)?,
        Some("dhall") => serde_dhall::from_file(&args.scene).parse()?,
        Some("toml") => toml::from_slice(&std::fs::read(&args.scene)?)?,
        Some("sexp") => serde_lexpr::from_reader(File::open(&args.scene)?)?,
        Some("ron") => ron::from_str(&std::fs::read_to_string(&args.scene)?)?,
        _ => anyhow::bail!("Unknown scene file extension"),
    };

    let mut builder = Builder::<Registry>::new(config.size);

    let root = builder.node("root", config.root)?;
    let output = builder.output(config.output)?;

    let (main, introspection) = builder.finish().run(root, output)?;

    for interface in args.interface {
        interface.serve(introspection.clone())?
    }

    return main.run(args.fps).await;
}

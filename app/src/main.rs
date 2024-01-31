use std::ffi::OsStr;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::{bail, Context, Result};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use structopt::StructOpt;

use photonic::attr::Bounded;
use photonic::AttrValue;
use photonic_effects::attrs::Button;
use photonic_effects::nodes::Alert;
use photonic_output_terminal::Terminal;

use crate::boxed::{BoxedBoundAttrDecl, BoxedFreeAttrDecl, BoxedNodeDecl, BoxedOutputDecl};
use crate::builder::{AttrBuilder, Builder, NodeBuilder, OutputBuilder};
use crate::config::Anything;
use crate::registry::{BoundAttrFactory, FreeAttrFactory, NodeFactory, OutputFactory, Registry};

mod config;
mod builder;

mod registry;

mod boxed;

#[derive(StructOpt)]
struct Opt {
    #[structopt(default_value = "scene.yaml")]
    scene: PathBuf,

    #[structopt(short, long, default_value = "30")]
    fps: usize,

    // #[structopt(short, long)]
    // interface: Vec<Interface>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    let scene = tokio::fs::read_to_string(&opt.scene).await
        .with_context(|| format!("Failed to read config file: '{}'", opt.scene.display()))?;

    let scene: config::Scene = match opt.scene.extension().and_then(OsStr::to_str) {
        Some("yaml") | Some("yml") => serde_yaml::from_str(&scene)?,
        Some("json") => serde_json::from_str(&scene)?,
        Some("dhall") => serde_dhall::from_str(&scene).parse()?,
        Some("toml") => toml::from_str(&scene)?,
        Some("ron") => ron::from_str(&scene)?,
        _ => bail!("Unknown scene file extension"),
    };

    let mut builder = Builder::<MyRegistry>::new(scene.size);

    let root = builder.node("root", scene.root)?;
    let output = builder.output(scene.output)?;

    let scene = builder.build();
    let scene = scene.run(root, output).await?;

    // let cli = photonic_interface_cli::stdio::CLI;
    // let mqtt = photonic_interface_mqtt::MQTT::new("mqtt://localhost:1884?client_id=photonic")?;

    tokio::select! {
        // Err(err) = scene.serve(cli) => bail!(err),
        // Err(err) = scene.serve(mqtt) => bail!(err),
        Err(err) = scene.run(opt.fps) => bail!(err),
        else => return Ok(())
    }
}

struct MyRegistry {}

impl Registry for MyRegistry {
    fn node<Builder>(kind: &str) -> Option<NodeFactory<Builder>>
        where Builder: NodeBuilder,
    {
        return Some(match kind {
            "alert" => Box::new(|config: Anything, builder: &mut Builder| -> Result<BoxedNodeDecl> {
                #[derive(Deserialize, Clone, Debug)]
                struct Config {
                    pub hue: config::Attr,
                    pub block: config::Attr,
                    pub speed: config::Attr,
                }

                let config: Config = Config::deserialize(config)?;

                return Ok(Box::new(Alert {
                    hue: builder.bound_attr("hue", config.hue)?,
                    block: builder.bound_attr("block", config.block)?,
                    speed: builder.free_attr("speed", config.speed)?,
                }));
            }),
            // "blackout" => Box::new(|config, builder| {
            //     return Ok(todo!());
            // }),
            // "noise" => Box::new(|config, builder| {
            //     return Ok(todo!());
            // }),
            // "raindrops" => Box::new(|config, builder| {
            //     return Ok(todo!());
            // }),
            _ => return None
        });
    }

    fn free_attr<V, Builder>(kind: &str) -> Option<FreeAttrFactory<V, Builder>>
        where Builder: AttrBuilder,
              V: AttrValue + DeserializeOwned,
    {
        return Some(match kind {
            "button" => Box::new(|config: Anything, builder: &mut Builder| -> Result<BoxedFreeAttrDecl<V>> {
                #[derive(Deserialize, Clone, Debug)]
                struct Config<V> {
                    pub value_pressed: V,
                    pub value_release: V,
                    pub hold_time: Duration,
                    pub trigger: config::Input,
                }

                let config: Config<V> = Config::deserialize(config)?;

                return Ok(Box::new(Button {
                    value_release: config.value_release,
                    value_pressed: config.value_pressed,
                    hold_time: config.hold_time,
                    trigger: builder.input(config.trigger)?,
                }));
            }),
            _ => return None
        });
    }

    fn bound_attr<V, Builder>(kind: &str) -> Option<BoundAttrFactory<V, Builder>>
        where Builder: AttrBuilder,
              V: AttrValue + DeserializeOwned + Bounded,
    {
        return Some(match kind {
            "button" => Box::new(|config: Anything, builder: &mut Builder| -> Result<BoxedBoundAttrDecl<V>> {
                #[derive(Deserialize, Clone, Debug)]
                struct Config<V> {
                    pub value_pressed: V,
                    pub value_release: V,
                    pub hold_time: Duration,
                    pub trigger: config::Input,
                }

                let config: Config<V> = Config::deserialize(config)?;

                return Ok(Box::new(Button {
                    value_release: config.value_release,
                    value_pressed: config.value_pressed,
                    hold_time: config.hold_time,
                    trigger: builder.input(config.trigger)?,
                }));
            }),
            _ => return None
        });
    }

    fn output<Builder>(kind: &str) -> Option<OutputFactory<Builder>>
        where Builder: OutputBuilder,
    {
        return Some(match kind {
            "terminal" => Box::new(|config: Anything, _builder: &mut Builder| -> Result<BoxedOutputDecl> {
                #[derive(Deserialize, Clone, Debug)]
                struct Config {
                    pub waterfall: bool,
                    pub path: Option<PathBuf>,
                }

                let config: Config = Config::deserialize(config)?;

                return Ok(Box::new(Terminal {
                    waterfall: config.waterfall,
                    path: config.path,
                }));
            }),
            _ => return None
        })
    }
}

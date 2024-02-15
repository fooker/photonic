use std::ffi::OsStr;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::{bail, Context, Result};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use structopt::StructOpt;

use photonic::attr::{Bounded, Range};
use photonic::{AttrValue, FreeAttrDecl};
use photonic_output_terminal::Terminal;

use photonic_dyn::boxed::{BoxedBoundAttrDecl, BoxedFreeAttrDecl, BoxedNodeDecl, BoxedOutputDecl};
use photonic_dyn::builder::{AttrBuilder, Builder, NodeBuilder, OutputBuilder};
use photonic_dyn::config;
use photonic_dyn::registry::{BoundAttrFactory, FreeAttrFactory, NodeFactory, OutputFactory, Registry};
use photonic_dyn::DynamicNode;


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
            "alert" => photonic_effects::nodes::Alert::<BoxedBoundAttrDecl<f32>, BoxedBoundAttrDecl<i64>, BoxedFreeAttrDecl<f32>>::factory(),
            "blackout" => photonic_effects::nodes::Blackout::<BoxedNodeDecl, BoxedFreeAttrDecl<bool>>::factory(),
            "brightness" => photonic_effects::nodes::Brightness::<BoxedNodeDecl, BoxedBoundAttrDecl<f32>>::factory(),
            "color_wheel" => photonic_effects::nodes::ColorWheel::<>::factory(),
            "noise" => photonic_effects::nodes::Noise::<BoxedFreeAttrDecl<f32>, BoxedFreeAttrDecl<f32>>::factory(),
            "overlay" => photonic_effects::nodes::Overlay::<BoxedNodeDecl, BoxedNodeDecl, BoxedBoundAttrDecl<f32>>::factory(),
            //"raindrops" => photonic_effects::nodes::Raindrops::<BoxedBoundAttrDecl<f32>, BoxedFreeAttrDecl<Range<photonic::color::palette::rgb::Rgb>>, BoxedBoundAttrDecl<Range<f32>>>::factory(),
            _ => return None
        });
    }

    fn free_attr<V, Builder>(kind: &str) -> Option<FreeAttrFactory<V, Builder>>
        where Builder: AttrBuilder,
              V: AttrValue + DeserializeOwned,
    {
        return Some(match kind {
            "button" => Box::new(|config: config::Anything, builder: &mut Builder| -> Result<BoxedFreeAttrDecl<V>> {
                #[derive(Deserialize, Clone, Debug)]
                struct Config<V> {
                    pub value_pressed: V,
                    pub value_release: V,
                    pub hold_time: Duration,
                    pub trigger: config::Input,
                }

                let config: Config<V> = Config::deserialize(config)?;

                return todo!();

                // return Ok(Box::new(Button {
                //     value_release: config.value_release,
                //     value_pressed: config.value_pressed,
                //     hold_time: config.hold_time,
                //     trigger: builder.input(config.trigger)?,
                // }));
            }),
            _ => return None
        });
    }

    fn bound_attr<V, Builder>(kind: &str) -> Option<BoundAttrFactory<V, Builder>>
        where Builder: AttrBuilder,
              V: AttrValue + DeserializeOwned + Bounded,
    {
        return Some(match kind {
            "button" => Box::new(|config: config::Anything, builder: &mut Builder| -> Result<BoxedBoundAttrDecl<V>> {
                #[derive(Deserialize, Clone, Debug)]
                struct Config<V> {
                    pub value_pressed: V,
                    pub value_release: V,
                    pub hold_time: Duration,
                    pub trigger: config::Input,
                }

                let config: Config<V> = Config::deserialize(config)?;

                return todo!();

                // return Ok(Box::new(Button {
                //     value_release: config.value_release,
                //     value_pressed: config.value_pressed,
                //     hold_time: config.hold_time,
                //     trigger: builder.input(config.trigger)?,
                // }));
            }),
            _ => return None
        });
    }

    fn output<Builder>(kind: &str) -> Option<OutputFactory<Builder>>
        where Builder: OutputBuilder,
    {
        return Some(match kind {
            "terminal" => Box::new(|config: config::Anything, _builder: &mut Builder| -> Result<BoxedOutputDecl> {
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

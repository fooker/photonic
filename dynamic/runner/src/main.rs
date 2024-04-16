use std::ffi::OsStr;
use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use clap::Parser;

use photonic_dynamic::builder::{Builder, Registries};
use photonic_dynamic::config;

#[derive(Parser)]
struct Opt {
    #[clap(default_value = "scene.yaml")]
    scene: PathBuf,

    #[structopt(short, long, default_value = "30")]
    fps: usize,
    // #[structopt(short, long)]
    // interface: Vec<Interface>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::parse();

    let scene = tokio::fs::read_to_string(&opt.scene)
        .await
        .with_context(|| format!("Failed to read config file: '{}'", opt.scene.display()))?;

    let scene: config::Scene = match opt.scene.extension().and_then(OsStr::to_str) {
        Some("yaml") | Some("yml") => serde_yaml::from_str(&scene)?,
        Some("json") => serde_json::from_str(&scene)?,
        Some("dhall") => serde_dhall::from_str(&scene).parse()?,
        Some("toml") => toml::from_str(&scene)?,
        Some("ron") => ron::from_str(&scene)?,
        _ => bail!("Unknown scene file extension"),
    };

    let mut builder = Builder::<RunnerRegistries>::new();

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

struct RunnerRegistries {}

impl Registries<Builder<Self>> for RunnerRegistries {
    type FreeAttr<V> = ();
    type BoundAttr<V> = ();
    type Node = photonic_effects::nodes::NodeRegistry;
    type Output = ();
}

use std::ffi::OsStr;
use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use clap::Parser;
use photonic::attr::Bounded;
use photonic::input::InputValue;
use photonic::AttrValue;
use serde::de::DeserializeOwned;

use photonic_dynamic::factory::{BoundAttrFactory, FreeAttrFactory, NodeFactory, OutputFactory};
use photonic_dynamic::registry::Registry;
use photonic_dynamic::{combine, config, Builder};

#[derive(Parser)]
struct Opt {
    #[arg(default_value = "scene.yaml")]
    scene: PathBuf,

    #[arg(short, long, default_value = "30")]
    fps: usize,
    // #[arg(short, long)]
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

impl Registry<Builder<Self>> for RunnerRegistries {
    fn node(kind: &str) -> Option<NodeFactory<Builder<Self>>> {
        return combine!(node, kind, (photonic_effects::dynamic::Registry));
    }

    fn free_attr<V>(kind: &str) -> Option<FreeAttrFactory<Builder<Self>, V>>
    where V: AttrValue + DeserializeOwned + InputValue {
        return combine!(free_attr, kind, (photonic_effects::dynamic::Registry));
    }

    fn bound_attr<V>(kind: &str) -> Option<BoundAttrFactory<Builder<Self>, V>>
    where V: AttrValue + DeserializeOwned + InputValue + Bounded {
        return combine!(bound_attr, kind, (photonic_effects::dynamic::Registry));
    }

    fn output(kind: &str) -> Option<OutputFactory<Builder<Self>>> {
        return combine!(
            output,
            kind,
            (photonic_effects::dynamic::Registry, photonic_output_terminal::dynamic::Registry)
        );
    }
}

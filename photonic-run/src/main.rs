#![feature(never_type)]

use std::fs::File;
use std::path::PathBuf;

use anyhow::Error;
use clap::Clap;

use photonic_run::builder::Builder;
use photonic_run::config;

#[derive(Clap)]
#[clap()]
struct CLI {
    #[clap(default_value = "scene.yaml")]
    scene: PathBuf,

    #[clap(short, long, default_value = "30")]
    fps: usize,
}

#[tokio::main]
async fn main() -> Result<!, Error> {
    let cli = CLI::parse();

    let scene: config::Scene = serde_yaml::from_reader(File::open(&cli.scene)?)?;

    let (main, _) = Builder::build(scene)?;

    return main.run(cli.fps).await;
}

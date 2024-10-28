#![allow(unused_variables)]

use anyhow::Result;
use palette::Hsl;

use photonic::attr::AsFixedAttr;
use photonic::Scene;
use photonic_effects::nodes::Raindrops;
use photonic_output_net::wled;

#[tokio::main]
async fn main() -> Result<()> {
    let mut scene = Scene::new();

    let raindrops = scene.node("raindrops", Raindrops {
        rate: 0.3.fixed(),
        decay: (0.96, 0.98).fixed(),
        color: (Hsl::new(27.5, 0.25, 0.5), Hsl::new(79.0, 0.5, 0.5)).fixed(),
    })?;

    let output = wled::WledSender {
        mode: Default::default(),
        size: 50,
        target: "192.168.0.29:21324".parse()?,
    };

    let scene = scene.run(raindrops, output).await?;

    return scene.run(60).await;
}

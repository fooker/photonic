use anyhow::Result;

use photonic::attr::{AsFixedAttr, FreeAttrDeclExt, Range};
use photonic::color::palette::Hsl;
use photonic::Scene;
use photonic_audio::attr::Power;
use photonic_effects::nodes::Raindrops;
use photonic_output_net::wled;
use photonic_output_terminal::Terminal;

#[tokio::main]
async fn main() -> Result<()> {
    let mut scene = Scene::new();

    let raindrops = scene.node("raindrops", Raindrops {
        rate: Power::new().with_low_pass_filter(200.0).scale(0.3),
        decay: (2.0, 3.0).fixed(),
        color: Range(Hsl::new(245.31, 0.5, 0.5), Hsl::new(333.47, 0.7, 0.5)).fixed(),
    })?;

    let _output = Terminal::new(80).with_waterfall(false);

    let output = wled::WledSender {
        mode: Default::default(),
        size: 50,
        target: "192.168.0.29:21324".parse()?,
    };

    let scene = scene.run(raindrops, output).await?;

    return scene.run(60).await;
}

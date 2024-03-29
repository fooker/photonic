use anyhow::{bail, Result};
use palette::{FromColor, Hsl, IntoColor};
use palette::rgb::Rgb;

use photonic::attr::{AsFixedAttr, Range};
use photonic::attr::FreeAttrDeclExt;
use photonic::Scene;
use photonic::scene::InputHandle;
use photonic_effects::nodes::{Brightness, Raindrops};
use photonic_output_terminal::Terminal;

#[tokio::main]
async fn main() -> Result<()> {
    let mut scene = Scene::new();

    let rate = scene.input::<f32>("rate")?;
    let color = scene.input::<Range<Rgb>>("color")?;

    let base = scene.node("raindrops", Raindrops {
        rate: rate.attr(0.3),
        decay: (0.96, 0.98).fixed(),
        color: color.attr(Range(
            Hsl::new(187.5, 0.25, 0.5).into_color(),
            Hsl::new(223.92, 0.5, 0.5).into_color(),
        )).map(|v| v.map(Hsl::from_color)),
    })?;

    let brightness = scene.node("brightness", Brightness {
        value: 1.0.fixed(),
        source: base,
        range: None,
    })?;

    let output = Terminal::new(80)
        .with_path("/tmp/photonic")
        .with_waterfall(true);

    let scene = scene.run(brightness, output).await?;

    let cli = photonic_interface_cli::stdio::CLI;

    tokio::select! {
        Err(err) = scene.serve(cli) => bail!(err),
        Err(err) = scene.run(60) => bail!(err),
        else => return Ok(())
    }
}

use anyhow::{bail, Result};
use palette::rgb::Rgb;
use palette::{Hsl, IntoColor};

use photonic::attr::{AsFixedAttr, Range};
use photonic::scene::InputHandle;
use photonic::Scene;
use photonic_effects::nodes::{Brightness, Raindrops};
use photonic_output_terminal::Terminal;

#[tokio::main]
async fn main() -> Result<()> {
    let mut scene = Scene::new(100);

    let rate = scene.input("rate")?;
    let color: InputHandle<Range<Rgb>> = scene.input("color")?;

    let base = scene.node("raindrops", Raindrops {
        //rate: 0.3.fixed(),
        rate: rate.attr(0.3),
        decay: (0.96, 0.98).fixed(),
        //color: (Hsl::new(187.5, 0.25, 0.5), Hsl::new(223.92, 0.5, 0.5)).fixed(),
        color: color.attr(Range(Hsl::new(187.5, 0.25, 0.5).into_color(), Hsl::new(223.92, 0.5, 0.5).into_color())),
    })?;

    let brightness = scene.node("brightness", Brightness {
        value: 1.0,
        source: base,
    })?;

    let output = Terminal::with_path("/tmp/photonic").with_waterfall(true);

    let scene = scene.run(brightness, output).await?;

    let cli = photonic_interface_cli::stdio::CLI;

    tokio::select! {
        Err(err) = scene.serve(cli) => bail!(err),
        Err(err) = scene.run(60) => bail!(err),
        else => return Ok(())
    }
}

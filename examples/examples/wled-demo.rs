use anyhow::Result;
use palette::{Hsl, Srgb};

use photonic::attr::AsFixedAttr;
use photonic::Scene;
use photonic_effects::nodes::{Brightness, ColorWheel, Raindrops};
use photonic_output_null::Null;

#[tokio::main]
async fn main() -> Result<()> {
    let mut scene = Scene::new();

    let base = scene.node("color_wheel", ColorWheel {
        scale: 1.0,
        speed: 0.3,
        offset: 0.0,
        saturation: 1.0,
        intensity: 1.0,
    })?;

    let base = scene.node("raindrops", Raindrops {
        rate: 0.3.fixed(),
        decay: (0.96, 0.98).fixed(),
        color: (Hsl::new(187.5, 0.25, 0.5), Hsl::new(223.92, 0.5, 0.5)).fixed(),
    })?;

    let brightness = scene.node("brightness", Brightness {
        value: 1.0.fixed(),
        source: base,
        range: None,
    })?;

    // let (scene, introspection) = scene.run(brightness, Terminal {
    //     waterfall: true,
    // })?;

    let scene = scene.run(brightness, Null::<Srgb>::default()).await?;

    return Ok(scene.run(60).await?);
}

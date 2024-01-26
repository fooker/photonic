use anyhow::Result;
use palette::LinSrgb;

use photonic::Scene;
use photonic_effects::nodes::{Brightness, ColorWheel};
use photonic_output_null::Null;

#[tokio::main]
async fn main() -> Result<()> {
    let mut scene = Scene::new(100);

    let base = scene.node("base", ColorWheel {
        scale: 1.0,
        speed: 0.3,
        offset: 0.0,
        saturation: 1.0,
        intensity: 1.0,
    })?;

    let brightness = scene.node("brightness", Brightness {
        value: 0.5,
        source: base,
    })?;

    let (scene, ) = scene.run(brightness, Null::<LinSrgb>::default())?;

    scene.run(60).await?;

    return Ok(());
}
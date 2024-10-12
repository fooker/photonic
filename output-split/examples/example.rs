use anyhow::Result;
use palette::rgb::Rgb;

use photonic::{Buffer, Scene};
use photonic_output_null::Null;
use photonic_output_split::Split;

#[tokio::main]
async fn main() -> Result<()> {
    let mut scene = Scene::new();

    let node = scene.node("solid", Buffer::<Rgb>::from_value(400, Rgb::new(0.0, 0.5, 1.0)))?;

    let out1 = Null::<Rgb>::with_size(100);
    let out2 = Null::<Rgb>::with_size(100);
    let out3 = Null::<Rgb>::with_size(100);
    let out4 = Null::<Rgb>::with_size(100);

    let recurse = Split::new(vec![Box::new(out1), Box::new(out2), Box::new(out3)]);

    let output = Split::new(vec![Box::new(recurse), Box::new(out4)]);

    let scene = scene.run(node, output).await?;

    return scene.run(60).await;
}

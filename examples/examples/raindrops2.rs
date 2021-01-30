#![feature(never_type)]

use std::time::Duration;

use failure::Error;

use photonic_console::ConsoleOutputDecl;
use photonic_core::animation;
use photonic_core::animation::Easing;
use photonic_core::color::HSLColor;
use photonic_core::core::Scene;
use photonic_core::timer::Ticker;
use photonic_core::attr::{AsFixedAttr, Range};
use photonic_effects::nodes::raindrops::RaindropsNodeDecl;
use photonic_effects::attrs::fader::FaderDecl;
use photonic_effects::attrs::sequence::SequenceDecl;
use photonic_grpc::GrpcInterface;

const SIZE: usize = 120;
const FPS: usize = 60;

#[tokio::main]
async fn main() -> Result<!, Error> {
    let mut scene = Scene::new(SIZE);

    let grpc = GrpcInterface::bind("localhost:5764".parse()?);

    let ticker = Ticker::new(Duration::from_secs(5));
    let raindrops_color = SequenceDecl {
        values: vec![
            Range(HSLColor::new(245.31, 0.5, 0.5),
                  HSLColor::new(333.47, 0.7, 0.5)),
            Range(HSLColor::new(0.0, 0.45, 0.5),
                  HSLColor::new(17.5, 0.55, 0.5)),
            Range(HSLColor::new(187.5, 0.25, 0.5),
                  HSLColor::new(223.92, 0.5, 0.5)),
        ],
        trigger: ticker.1,
    };
    let raindrops_color = FaderDecl {
        input: raindrops_color,
        easing: Easing::with(animation::linear, Duration::from_secs(4)),
    };

    let raindrops = scene.node("raindrops:node", RaindropsNodeDecl {
        rate: 0.3_f64.fixed(),
        color: raindrops_color,
        decay: (0.96, 0.98).fixed(),
    })?;

    let (main, registry) = scene.output(raindrops, ConsoleOutputDecl {
        whaterfall: true
    })?;

    registry.serve(grpc)?;

    main.run(FPS).await?;
}

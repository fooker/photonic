#![feature(never_type)]

use std::time::Duration;

use failure::Error;

use photonic_console::ConsoleOutputDecl;
use photonic_core::animation;
use photonic_core::animation::Easing;
use photonic_core::color::HSLColor;
use photonic_core::scene::Scene;
use photonic_core::timer::Ticker;
use photonic_core::attr::{AsFixedAttr, Range};
use photonic_effects::nodes::raindrops::RaindropsNodeDecl;
use photonic_effects::attrs::fader::FaderDecl;
use photonic_effects::attrs::sequence::SequenceDecl;
use photonic_grpc::GrpcInterface;
use photonic_effects::nodes::blackout::BlackoutNodeDecl;
use photonic_effects::attrs::manual::ManualDecl;

const SIZE: usize = 120;
const FPS: usize = 60;

#[tokio::main]
async fn main() -> Result<!, Error> {
    let mut scene = Scene::new(SIZE);

    let grpc = GrpcInterface::bind("127.0.0.1:5764".parse()?);

    let switcher = scene.input("ticker")?;

    // let ticker = Ticker::new(Duration::from_secs(5));

    let raindrops_color = SequenceDecl {
        values: vec![
            Range(HSLColor::new(245.31, 0.5, 0.5),
                  HSLColor::new(333.47, 0.7, 0.5)),
            Range(HSLColor::new(0.0, 0.45, 0.5),
                  HSLColor::new(17.5, 0.55, 0.5)),
            Range(HSLColor::new(187.5, 0.25, 0.5),
                  HSLColor::new(223.92, 0.5, 0.5)),
        ],
        trigger: switcher.0,
    };
    let raindrops_color = FaderDecl {
        input: raindrops_color,
        easing: Easing::with(animation::linear, Duration::from_secs(4)),
    };

    let raindrops = scene.node("raindrops", RaindropsNodeDecl {
        rate: 0.3_f64.fixed(),
        color: raindrops_color,
        decay: (0.96, 0.98).fixed(),
    })?;

    let brightness = scene.input("brightness")?;

    let blackout = scene.node("blackout", BlackoutNodeDecl {
        source: raindrops,
        value: ManualDecl { value: brightness.0 },
        range: None
    })?;

    let (main, registry) = scene.run(blackout, ConsoleOutputDecl {
        whaterfall: true
    })?;

    println!("{:#?}", registry.root);

    registry.serve(grpc)?;

    main.run(FPS).await?;
}

#![feature(never_type)]

use std::time::Duration;

use failure::Error;

use photonic_console::ConsoleOutputDecl;
use photonic_core::animation;
use photonic_core::animation::Easing;
use photonic_core::core::Scene;
use photonic_core::timer::Ticker;
use photonic_effects::nodes::raindrops::RaindropsNodeDecl;
use photonic_effects::nodes::switch::SwitchNodeDecl;
use photonic_effects::attrs::looper::LooperDecl;
use photonic_core::color::HSLColor;
use photonic_core::attr::AsFixedAttr;
use photonic_grpc::GrpcInterface;

const SIZE: usize = 120;
const FPS: usize = 60;

#[tokio::main]
async fn main() -> Result<!, Error> {
    let mut scene = Scene::new(SIZE);

    let grpc = GrpcInterface::bind("127.0.0.1:5764".parse()?);

    let raindrops_violet = scene.node("raindrops:violet", RaindropsNodeDecl {
        rate: 0.3_f64.fixed(),
        color: (HSLColor::new(245.31, 0.5, 0.5),
                HSLColor::new(333.47, 0.7, 0.5)).fixed(),
        decay: (0.96, 0.98).fixed(),
    })?;

    let raindrops_orange = scene.node("raindrops:orange", RaindropsNodeDecl {
        rate: 0.3_f64.fixed(),
        color: (HSLColor::new(0.0, 0.45, 0.5),
                HSLColor::new(17.5, 0.55, 0.5)).fixed(),
        decay: (0.96, 0.98).fixed(),
    })?;

    let raindrops_iceblue = scene.node("raindrops:iceblue", RaindropsNodeDecl {
        rate: 0.3_f64.fixed(),
        color: (HSLColor::new(187.5, 0.25, 0.5),
                HSLColor::new(223.92, 0.5, 0.5)).fixed(),
        decay: (0.96, 0.98).fixed(),
    })?;

    let ticker = Ticker::new(Duration::from_secs(5));
    let switch_raindrops_timer = LooperDecl {
        step: 1,
        trigger: ticker.1,
    };

    let switch_raindrops = scene.node("raindrops", SwitchNodeDecl {
        sources: vec![raindrops_violet, raindrops_orange, raindrops_iceblue],
        fade: switch_raindrops_timer,
        easing: Easing::some(animation::linear, Duration::from_secs(4)),
    })?;

    let (main, registry) = scene.run(switch_raindrops, ConsoleOutputDecl {
        whaterfall: true
    })?;

    println!("{:#?}", registry.root);

    registry.serve(grpc)?;

    main.run(FPS).await?;
}

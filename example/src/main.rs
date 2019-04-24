#![feature(never_type)]

use std::time::Duration;

use photonic::nodes::raindrops::{RaindropsNode, RaindropsNodeDecl};
use photonic::nodes::overlay::{OverlayNode, OverlayNodeDecl};
use photonic::nodes::larson::{LarsonNode, LarsonNodeDecl};
use photonic::nodes::switch::{SwitchNode, SwitchNodeDecl};
use photonic::outputs::console::{ConsoleOutput, ConsoleOutputDecl};
use photonic::core::Scene;
use photonic::core::Node;
use photonic::core::Loop;
use failure::Error;
use photonic::values::looper::{Looper, LooperDecl};
use photonic::values::Value;
use photonic::trigger::Timer;
use photonic::values::random::RandomDecl;

const SIZE: usize = 42;
const FPS: usize = 60;

fn main() -> Result<!, Error> {
    let mut scene = Scene::new(SIZE);

    let raindrops_violet = scene.node("raindrops:violet", RaindropsNodeDecl {
        rate: 0.3.into(),
        hue: (245.31.into(), 333.47.into()),
        lightness: (1.0.into(), 1.0.into()),
        saturation: (0.5.into(), 0.5.into()),
        decay: (0.96.into(), 0.98.into()),
    })?;

    let raindrops_orange = scene.node("raindrops:violet", RaindropsNodeDecl {
        rate: 0.3.into(),
        hue: (0.00.into(), 17.50.into()),
        lightness: (0.95.into(), 1.0.into()),
        saturation: (0.5.into(), 0.5.into()),
        decay: (0.96.into(), 0.98.into()),
    })?;

    let raindrops_iceblue = scene.node("raindrops:violet", RaindropsNodeDecl {
        rate: 0.3.into(),
        hue: (187.50.into(), 223.92.into()),
        lightness: (0.5.into(), 1.0.into()),
        saturation: (0.5.into(), 0.5.into()),
        decay: (0.96.into(), 0.98.into()),
    })?;


//    let switch_raindrops_timer = Value::Dynamic{
//        name: String::from("raindrops:color"),
//        value: Box::new(Looper::new(0, 3, 1, Some(Duration::from_secs(5)))),
//    };

    let switch_raindrops_timer = Box::new(LooperDecl {
        step: 1,
        auto_trigger: Timer::new(Some(Duration::from_secs(5))),
    });

    let switch_raindrops = scene.node("raindrops", SwitchNodeDecl {
        sources: vec![raindrops_violet, raindrops_orange, raindrops_iceblue],
        position: switch_raindrops_timer,
        easing: None,
    })?;

    let overlay_bell = scene.node("bell", LarsonNodeDecl {
        hue: 0.0.into(),
        speed: 50.0.into(),
        width: 25.0.into(),
    })?;
    let overlay_bell = scene.node("bell:overlay", OverlayNodeDecl {
        base: switch_raindrops,
        overlay: overlay_bell,
        blend: 0.0.into(),
        easing: None,
    })?;

    let output = scene.output(ConsoleOutputDecl {
        whaterfall: true,
    })?;

    Loop::new(overlay_bell, output).run(FPS)?;
}

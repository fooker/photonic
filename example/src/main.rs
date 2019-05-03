#![feature(never_type)]
#![feature(type_alias_enum_variants)]

use std::time::Duration;

use failure::Error;

use photonic_console::ConsoleOutputDecl;
use photonic_core::animation;
use photonic_core::animation::Easing;
use photonic_core::core::Scene;
use photonic_core::timer::Timer;
use photonic_effects::nodes::larson::LarsonNodeDecl;
use photonic_effects::nodes::overlay::OverlayNodeDecl;
use photonic_effects::nodes::raindrops::RaindropsNodeDecl;
use photonic_effects::nodes::switch::SwitchNodeDecl;
use photonic_effects::values::button::ButtonDecl;
use photonic_effects::values::fader::FaderDecl;
use photonic_effects::values::looper::LooperDecl;
use photonic_mqtt::MqttHandleBuilder;

const SIZE: usize = 42;
const FPS: usize = 60;

fn main() -> Result<!, Error> {
    let mut scene = Scene::new(SIZE);

    let mut timer = Timer::new();
    let mut mqtt = MqttHandleBuilder::new("photonic", "localhost", 1883)
        .with_realm("photonic");

    let raindrops_violet = scene.node("raindrops:violet", RaindropsNodeDecl {
        rate: 0.3.into(),
        hue: (245.31.into(), 333.47.into()),
        lightness: (0.5.into(), 0.7.into()),
        saturation: (0.5.into(), 0.5.into()),
        decay: (0.96.into(), 0.98.into()),
    })?;

    let raindrops_orange = scene.node("raindrops:violet", RaindropsNodeDecl {
        rate: 0.3.into(),
        hue: (0.00.into(), 17.50.into()),
        lightness: (0.45.into(), 0.55.into()),
        saturation: (0.5.into(), 0.5.into()),
        decay: (0.96.into(), 0.98.into()),
    })?;

    let raindrops_iceblue = scene.node("raindrops:violet", RaindropsNodeDecl {
        rate: 0.3.into(),
        hue: (187.50.into(), 223.92.into()),
        lightness: (0.25.into(), 0.5.into()),
        saturation: (0.5.into(), 0.5.into()),
        decay: (0.96.into(), 0.98.into()),
    })?;

    let switch_raindrops_timer = Box::new(LooperDecl {
        step: 1,
        trigger: timer.ticker(Duration::from_secs(5)),
    });

    let switch_raindrops = scene.node("raindrops", SwitchNodeDecl {
        sources: vec![raindrops_violet, raindrops_orange, raindrops_iceblue],
        position: switch_raindrops_timer,
        easing: Easing::some(animation::linear, Duration::from_secs(4)),
    })?;

    let overlay_bell = scene.node("bell", LarsonNodeDecl {
        hue: 0.0.into(),
        speed: 50.0.into(),
        width: 25.0.into(),
    })?;

    let overlay_bell_button = Box::new(ButtonDecl {
        value: (0.0, 1.0),
        hold_time: Duration::from_secs(4),
        trigger: mqtt.trigger("bell"),
    });
    let overlay_bell_button = Box::new(FaderDecl {
        input: overlay_bell_button,
        easing: Easing::with(animation::linear, Duration::from_secs(2)),
    });

    let overlay_bell = scene.node("bell:overlay", OverlayNodeDecl {
        base: switch_raindrops,
        overlay: overlay_bell,
        blend: overlay_bell_button,
    })?;

    let mut main = scene.output(overlay_bell, ConsoleOutputDecl {
        whaterfall: true
    })?;

    mqtt.start()?;

    main.register(move |d| timer.update(d));

    main.run(FPS)?;
}

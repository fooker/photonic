#![feature(never_type)]

use std::time::Duration;

use failure::Error;

use photonic_console::ConsoleOutputDecl;
use photonic_core::animation;
use photonic_core::animation::Easing;
use photonic_core::attr::{AsFixedAttr, Range};
use photonic_core::color::HSLColor;
use photonic_core::core::Scene;
use photonic_core::timer::Ticker;
use photonic_effects::attrs::button::ButtonDecl;
use photonic_effects::attrs::fader::FaderDecl;
use photonic_effects::attrs::sequence::SequenceDecl;
use photonic_effects::nodes::larson::LarsonNodeDecl;
use photonic_effects::nodes::overlay::OverlayNodeDecl;
use photonic_effects::nodes::raindrops::RaindropsNodeDecl;
use photonic_mqtt::MqttHandleBuilder;

const SIZE: usize = 120;
const FPS: usize = 60;

#[tokio::main]
async fn main() -> Result<!, Error> {
    let mut scene = Scene::new(SIZE);

    let mut mqtt = MqttHandleBuilder::new("photonic", "localhost", 1883)
        .with_realm("photonic");

    let ticker = Ticker::new(Duration::from_secs(5));
    let raindrops_color = SequenceDecl {
        values: vec![
            Range::new(HSLColor::new(245.31, 0.5, 0.5),
                       HSLColor::new(333.47, 0.7, 0.5)),
            Range::new(HSLColor::new(0.0, 0.45, 0.5),
                       HSLColor::new(17.5, 0.55, 0.5)),
            Range::new(HSLColor::new(187.5, 0.25, 0.5),
                       HSLColor::new(223.92, 0.5, 0.5)),
        ],
        trigger: ticker.1,
    };
    let raindrops_color = FaderDecl {
        input: raindrops_color,
        easing: Easing::with(animation::linear, Duration::from_secs(4)),
    };

    let raindrops = scene.node("raindrops:violet", RaindropsNodeDecl {
        rate: 0.3_f64.fixed(),
        color: raindrops_color,
        decay: (0.96, 0.98).fixed(),
    })?;

    let overlay_bell = scene.node("bell", LarsonNodeDecl {
        hue: 0.0.fixed(),
        speed: 50.0.fixed(),
        width: 25.0.fixed(),
    })?;

    let overlay_bell_button = ButtonDecl {
        value: (0.0, 1.0),
        hold_time: Duration::from_secs(4),
        trigger: mqtt.trigger("bell"),
    };
    let overlay_bell_button = FaderDecl {
        input: overlay_bell_button,
        easing: Easing::with(animation::linear, Duration::from_secs(2)),
    };

    let effect = scene.node("bell:overlay", OverlayNodeDecl {
        base: raindrops,
        overlay: overlay_bell,
        blend: overlay_bell_button,
    })?;

//    let effect = scene.node("effect", PlasmaNodeDecl {
//        h: ((0.0.into(), 360.0.into()), 50.0.into()),
//        s: ((0.8.into(), 1.0.into()), 20.0.into()),
//        v: ((1.0.into(), 1.0.into()), 1.0.into()),
//        speed: 25.0.into(),
//    })?;
//
//    let effect = scene.node("effect", AlertNodeDecl {
//        hue: 0.0.into(),
//        block_size: 8.into(),
//        speed: 1.0.into(),
//    })?;
//
//    let effect = scene.node("effect", UdpReciverNodeDecl::<_, RGBColor>::bind("127.0.0.1:7331"))?;

    let (main, _) = scene.run(effect, ConsoleOutputDecl {
        whaterfall: true
    })?;

    mqtt.start()?;

    main.run(FPS).await?;
}

use std::time::Duration;

use anyhow::{bail, Result};
use palette::{Hsl, IntoColor, Srgb};

use photonic::attr::{AsFixedAttr, Range};
use photonic::{Scene, WhiteMode};
use photonic_effects::attrs::{Button, Fader, Sequence};
use photonic_effects::easing::{EasingDirection, Easings};
use photonic_effects::nodes::{Alert, Blackout, Brightness, ColorWheel, Larson, Noise, Overlay, Raindrops, Splice, Switch};
use photonic_output_net::netdmx::{Channel, Fixture, NetDmxSender};
use photonic_output_terminal::Terminal;

#[tokio::main]
async fn main() -> Result<()> {
    let mut scene = Scene::new();

    let input_next = scene.input("next")?;
    let input_prev = scene.input("prev")?;

    let raindrops = scene.node("raindrops", Raindrops {
        rate: 0.3.fixed(),
        decay: (0.96, 0.98).fixed(),
        color: Fader {
            input: Sequence {
                next: Some(input_next),
                prev: Some(input_prev),
                values: vec![
                    Range(Hsl::new(245.31, 0.5, 0.5).into_color(), Hsl::new(333.47, 0.7, 0.5).into_color()),
                    Range(Hsl::new(0.0, 0.45, 0.5).into_color(), Hsl::new(17.5, 0.55, 0.5).into_color()),
                    Range(Hsl::new(187.5, 0.25, 0.5).into_color(), Hsl::new(223.92, 0.5, 0.5).into_color()),
                ],
            },
            easing: Easings::Quadratic(EasingDirection::InOut).with_speed(Duration::from_secs(2)),
        },
    })?;

    let noise = scene.node("noise", Noise {
        speed: 0.05.fixed(),
        stretch: 1.0.fixed(),
        noise: noise::OpenSimplex::default(),
    })?;

    let colors = scene.node("colors", ColorWheel {
        scale: 0.0,
        speed: 0.1,
        offset: 0.0,
        saturation: 1.0,
        intensity: 1.0,
    })?;

    // TODO: Add switcher for more animations
    let input_animation = scene.input::<i64>("animation")?;
    let animation = scene.node("animation", Switch {
        sources: vec![colors],
        value: input_animation.attr(0),
        easing: Easings::Quartic(EasingDirection::InOut).with_speed(Duration::from_secs(3)),
    })?;

    let input_brightness = scene.input::<f32>("brightness")?;
    let brightness = scene.node("brightness", Brightness {
        value: Fader {
            input: input_brightness.attr(0.0),
            easing: Easings::Cubic(EasingDirection::InOut).with_speed(Duration::from_secs(1)),
        },
        source: animation,
        range: None,
    })?;

    let alert = scene.node("alert", Alert {
        hue: 0.0.fixed(),
        block: 1.fixed(),
        speed: 1.0.fixed(),
    })?;

    let input_alert = scene.input("alert")?;
    let alert = scene.node("alert_overlay", Overlay {
        base: brightness,
        pave: alert,
        blend: Fader {
            input: Button {
                value_release: 0.0,
                value_pressed: 1.0,
                hold_time: Duration::from_secs(5),
                trigger: input_alert,
            },
            easing: Easings::Quartic(EasingDirection::InOut).with_speed(Duration::from_secs(1)),
        },
    })?;

    let input_kitchen = scene.input::<bool>("kitchen")?;
    let kitchen = scene.node("kitchen", Blackout {
        source: alert,
        active: input_kitchen.attr(false),
        value: Srgb::new(1.0, 1.0, 1.0).into_color(),
        range: Some(0..2),
    })?;

    let larson1 = scene.node("larson1", Larson {
        hue: 0.0.fixed(),
        width: 4.0.fixed(),
        speed: 1.0.fixed(),
    })?;

    let larson2 = scene.node("larson2", Larson {
        hue: 0.0.fixed(),
        width: 4.0.fixed(),
        speed: 1.0.fixed(),
    })?;

    let larson = scene.node("larson", Splice {
        n1: larson1,
        n2: larson2,
        split: 8,
    })?;

    let splice = scene.node("larson_splice", Splice {
       n1: kitchen,
       n2: larson,
       split: -16,
    })?;

    let output = NetDmxSender::with_address("127.0.0.1:34254".parse()?)
        .add_fixture(Fixture {
            dmx_address: 500,
            dmx_channels: vec![Channel::Red, Channel::Green, Channel::Blue, Channel::White],
            white_mode: WhiteMode::Accurate,
        })
        .add_fixture(Fixture {
            dmx_address: 508,
            dmx_channels: vec![Channel::Red, Channel::Green, Channel::Blue, Channel::White],
            white_mode: WhiteMode::Accurate,
        })
        .add_fixtures(20, |n| Fixture {
            dmx_address: 1 + n * 3,
            dmx_channels: vec![Channel::Red, Channel::Green, Channel::Blue],
            white_mode: WhiteMode::None,
        });

    let output = Terminal::new(100)
        .with_path("/tmp/photonic")
        .with_waterfall(true);

    let scene = scene.run(splice, output).await?;

    let cli = photonic_interface_cli::stdio::CLI;

    let mqtt = photonic_interface_mqtt::MQTT::new("mqtt://localhost:1884?client_id=photonic")?;

    tokio::select! {
        Err(err) = scene.serve(cli) => bail!(err),
        Err(err) = scene.serve(mqtt) => bail!(err),
        Err(err) = scene.run(20) => bail!(err),
        else => return Ok(())
    }
}

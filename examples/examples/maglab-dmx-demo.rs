#![allow(unused_variables)]

use std::time::Duration;

use anyhow::Result;
use palette::{FromColor, Hsl, Srgb};

use photonic::attr::{AsFixedAttr, Range};
use photonic::node::map::Map;
use photonic::{Rgbw, Scene, WithWhite};
use photonic_effects::attrs::{Button, Fader, Sequence, Switch};
use photonic_effects::easing::{EasingDirection, Easings};
use photonic_effects::nodes::{Alert, Blackout, Brightness, ColorWheel, Noise, Overlay, Raindrops, Select};
use photonic_output_net::netdmx::{Channel, Channels, Fixture, NetDmxSender};
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
                    Range(Hsl::new(245.31, 0.5, 0.5), Hsl::new(333.47, 0.7, 0.5)),
                    Range(Hsl::new(0.0, 0.45, 0.5), Hsl::new(17.5, 0.55, 0.5)),
                    Range(Hsl::new(187.5, 0.25, 0.5), Hsl::new(223.92, 0.5, 0.5)),
                ],
            },
            easing: Easings::Quadratic(EasingDirection::InOut).with_speed(Duration::from_secs(2)),
        },
    })?;

    let noise = scene.node("noise", Noise {
        speed: 0.05.fixed(),
        stretch: 0.01.fixed(),
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
    let animation = scene.node("animation", Select {
        sources: vec![noise],
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

    // let uv_larson = scene.node("uv_larson", Larson {
    //     hue: 0.0.fixed(),
    //     width: 4.0.fixed(),
    //     speed: 1.0.fixed(),
    // })?;
    //
    // let uv_larson = scene.node("uv_larson:splice", Splice {
    //    n1: alert,
    //    n2: uv_larson,
    //    split: -8,
    // })?;

    let output = alert;

    let output = scene.node("output:rgbw", Map {
        source: output,
        mapper: |e| Srgb::from_color(e).black(),
    })?;

    let input_kitchen = scene.input::<bool>("kitchen")?;
    let kitchen = scene.node("kitchen", Blackout {
        source: output,
        active: Fader {
            input: Switch {
                value_release: 0.0,
                value_pressed: 1.0,
                input: input_kitchen,
            },
            easing: Easings::Linear.with_speed(Duration::from_secs(1)),
        },
        value: Srgb::new(0.0, 0.0, 0.0).full(),
        range: Some(0..2),
    })?;

    let output = NetDmxSender::<Rgbw>::with_address("127.0.0.1:34254".parse()?)
        .add_fixture(
            Fixture::with_address(500)
                .with_channel(Channels::<Rgbw>::red())
                .with_channel(Channels::<Rgbw>::green())
                .with_channel(Channels::<Rgbw>::blue())
                .with_channel(Channels::<Rgbw>::white().calibrate(0.7)),
        )
        .add_fixture(
            Fixture::with_address(508)
                .with_channel(Channels::<Rgbw>::red())
                .with_channel(Channels::<Rgbw>::green())
                .with_channel(Channels::<Rgbw>::blue())
                .with_channel(Channels::<Rgbw>::white().calibrate(0.7)),
        )
        .add_fixtures(20, |n| {
            Fixture::with_address(1 + n * 3)
                .with_channel(Channels::<Rgbw>::red())
                .with_channel(Channels::<Rgbw>::green())
                .with_channel(Channels::<Rgbw>::blue())
        });

    let kitchen = scene.node("output:rgb", Map {
        source: kitchen,
        mapper: |e| e.color,
    })?;
    let output = Terminal::new(100).with_path("/tmp/photonic").with_waterfall(true);

    let mut scene = scene.run(kitchen, output).await?;

    let restore = photonic_interface_restore::Restore {
        path: "/tmp/photonic.example.restore".into(),
        write_threshold: 5,
        write_timeout: Duration::from_secs(5),
    };
    scene.serve("restore", restore);

    let cli = photonic_interface_cli::stdio::CLI;
    scene.serve("CLI", cli);

    let mqtt = photonic_interface_mqtt::MQTT::with_url("mqtt://localhost:1883?client_id=photonic")?;
    scene.serve("MQTT", mqtt);

    let grpc = photonic_interface_grpc::GRPC::new()?;
    scene.serve("GRPC", grpc);

    return scene.run(20).await;
}

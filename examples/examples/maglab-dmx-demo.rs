use std::time::Duration;

use anyhow::{bail, Result};
use palette::{Hsl, IntoColor};

use photonic::{Scene, WhiteMode};
use photonic::attr::{AsFixedAttr, Range};
use photonic_effects::attrs::{Fader, Sequence};
use photonic_effects::easing::{EasingDirection, Easings};
use photonic_effects::nodes::{Alert, Brightness, Raindrops};
use photonic_output_net::netdmx::{Channels, Fixture, NetDmxSender};
use photonic_output_terminal::Terminal;

#[tokio::main]
async fn main() -> Result<()> {
    let mut scene = Scene::new(100);

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
                    Range(Hsl::new(245.31, 0.5, 0.5).into_color(),
                          Hsl::new(333.47, 0.7, 0.5).into_color()),
                    Range(Hsl::new(0.0, 0.45, 0.5).into_color(),
                          Hsl::new(17.5, 0.55, 0.5).into_color()),
                    Range(Hsl::new(187.5, 0.25, 0.5).into_color(),
                          Hsl::new(223.92, 0.5, 0.5).into_color()),
                ],
            },
            easing: Easings::Quadratic(EasingDirection::InOut)
                .with_speed(Duration::from_secs(2)),
        },
    })?;

    let alert = scene.node("alert", Alert {
        hue: 0.0.fixed(),
        block: 1.fixed(),
        speed: 1.0.fixed(),
    })?;

    let input_brightness = scene.input("brightness")?.attr(1.0);
    let brightness = scene.node("brightness", Brightness {
        value: input_brightness,
        source: raindrops,
    })?;

    let output = NetDmxSender::with_address("127.0.0.1:34254".parse()?)
        .add_fixture(Fixture {
            pixel: 0,
            dmx_address: 500,
            dmx_channels: Channels::RGBW(WhiteMode::Accurate),
        })
        .add_fixture(Fixture {
            pixel: 1,
            dmx_address: 504,
            dmx_channels: Channels::RGBW(WhiteMode::Accurate),
        })
        .add_fixtures(20, |n| Fixture {
            pixel: n + 1,
            dmx_address: 1 + n * 3,
            dmx_channels: Channels::RGB,
        });

    // let output = Terminal::with_path("/tmp/photonic")
    //     .with_waterfall(true);

    let scene = scene.run(brightness, output).await?;

    let cli = photonic_interface_cli::stdio::CLI;
    let mqtt = photonic_interface_mqtt::MQTT::new("mqtt://localhost:1884?client_id=photonic")?;

    tokio::select! {
        Err(err) = scene.serve(cli) => bail!(err),
        Err(err) = scene.serve(mqtt) => bail!(err),
        Err(err) = scene.run(60) => bail!(err),
        else => return Ok(())
    }
}
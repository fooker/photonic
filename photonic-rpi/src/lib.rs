#![cfg(target = "arm-unknown-linux-gnueabihf")]
#![allow(clippy::needless_return)]

use anyhow::Error;
use palette::Component;

use photonic_core::color::RGBColor;
use photonic_core::output::{Output, OutputDecl};
use photonic_core::scene::Render;

pub type Kind = rs_ws281x::StripType;

pub struct StripDecl {
    pub pin: u8,
    pub kind: Kind,
    pub brightness: f64,
}

pub struct Strip {
    size: usize,
    controller: rs_ws281x::Controller,
}

impl OutputDecl for StripDecl {
    type Element = RGBColor;
    type Target = Strip;

    fn materialize(self, size: usize) -> Result<Self::Target, Error> {
        let controller = rs_ws281x::ControllerBuilder::new()
            .freq(800_000)
            .channel(
                0,
                rs_ws281x::ChannelBuilder::new()
                    .pin(self.pin as i32)
                    .count(size as i32)
                    .strip_type(self.kind)
                    .brightness((self.brightness * 255.0) as u8)
                    .build(),
            )
            .render_wait_time(0)
            .build()?;

        return Ok(Self::Target { size, controller });
    }
}

impl Output for Strip {
    type Element = RGBColor;

    fn render<E: Into<Self::Element>>(&mut self, render: &Render<Element = E>) {
        let leds = self.controller.leds_mut(0);

        for i in 0..self.size {
            let rgb: RGBColor = render.get(i).into();
            let (r, g, b) = rgb.into_components();
            leds[i] = [r.convert(), g.convert(), b.convert(), 0];
        }

        self.controller.render().expect("WS281x render error");
    }
}

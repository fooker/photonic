use anyhow::Error;

use photonic_core::color::RGBColor;
use photonic_core::node::Render;
use photonic_core::Output;

pub struct LedStripOutput {
    size: usize,
    controller: rs_ws281x::Controller,
}

impl Output for LedStripOutput {
    type Element = RGBColor;

    const KIND: &'static str = "Raspberry Pi LED Strip";

    fn render(&mut self, render: &dyn Render<Element = Self::Element>) {
        let leds = self.controller.leds_mut(0);

        for i in 0..self.size {
            let rgb: RGBColor = render.get(i).into();
            let (r, g, b) = rgb.into_components();
            leds[i] = [r.convert(), g.convert(), b.convert(), 0];
        }

        self.controller.render().expect("Strip render error");
    }
}

pub fn materialize(desc: super::LedStripOutputDecl, size: usize) -> Result<LedStripOutput, Error> {
    let controller = rs_ws281x::ControllerBuilder::new()
        .freq(800_000)
        .channel(
            0,
            rs_ws281x::ChannelBuilder::new()
                .pin(desc.pin as i32)
                .count(size as i32)
                .strip_type(desc.kind)
                .brightness((desc.brightness * 255.0) as u8)
                .build(),
        )
        .render_wait_time(0)
        .build()?;

    return Ok(Self::Target { size, controller });
}

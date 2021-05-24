use anyhow::Error;
use palette::Component;

use photonic_core::color::RGBColor;
use photonic_core::node::Render;
use photonic_core::Output;

use super::Chip;

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
                .strip_type(match desc.chip {
                    Chip::Sk6812Rgbw => rs_ws281x::StripType::Sk6812Rgbw,
                    Chip::Sk6812Rbgw => rs_ws281x::StripType::Sk6812Rbgw,
                    Chip::Sk6812Gbrw => rs_ws281x::StripType::Sk6812Gbrw,
                    Chip::Sk6812Grbw => rs_ws281x::StripType::Sk6812Grbw,
                    Chip::Sk6812Brgw => rs_ws281x::StripType::Sk6812Brgw,
                    Chip::Sk6812Bgrw => rs_ws281x::StripType::Sk6812Bgrw,
                    Chip::Ws2811Rgb  => rs_ws281x::StripType::Ws2811Rgb,
                    Chip::Ws2811Rbg  => rs_ws281x::StripType::Ws2811Rbg,
                    Chip::Ws2811Grb  => rs_ws281x::StripType::Ws2811Grb,
                    Chip::Ws2811Gbr  => rs_ws281x::StripType::Ws2811Gbr,
                    Chip::Ws2811Brg  => rs_ws281x::StripType::Ws2811Brg,
                    Chip::Ws2811Bgr  => rs_ws281x::StripType::Ws2811Bgr,
                    Chip::Ws2812     => rs_ws281x::StripType::Ws2812,
                    Chip::Sk6812     => rs_ws281x::StripType::Sk6812,
                    Chip::Sk6812W    => rs_ws281x::StripType::Sk6812W,
                })
                .brightness((desc.brightness * 255.0) as u8)
                .build(),
        )
        .render_wait_time(0)
        .build()?;

    return Ok(LedStripOutput { size, controller });
}

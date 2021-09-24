use std::thread;
use std::time::Duration;

use anyhow::Result;

use photonic_core::color::Black;
use photonic_core::color::RGBColor;
use photonic_ledstrip::Chip::{Sk6812Grbw};
use photonic_ledstrip::{Controller, RenderContext};
use photonic_ledstrip::controllers::spi;

const SIZE: usize = 100;

struct Render {
    state: usize,
}

impl Render {
    fn next(&mut self) {
        self.state += 1;
    }
}

impl Default for Render {
    fn default() -> Self {
        return Render { state: 0 };
    }
}

impl photonic_core::node::Render for Render {
    type Element = RGBColor;

    fn get(&self, index: usize) -> Self::Element {
        let colors: &[RGBColor] = &[
            RGBColor::new(1.0, 0.0, 0.0),
            RGBColor::new(1.0, 1.0, 0.0),
            RGBColor::new(0.0, 1.0, 0.0),
            RGBColor::new(0.0, 1.0, 1.0),
            RGBColor::new(0.0, 0.0, 1.0),
            RGBColor::new(1.0, 0.0, 1.0),
            RGBColor::new(1.0, 1.0, 1.0),
        ];

        return if index % 3 == self.state % 3 {
            colors[self.state % colors.len()]
        } else {
            RGBColor::black()
        };
    }
}

fn main() -> Result<()> {
    let config = spi::Config {
        dev: "/dev/spidev0.0".into(),
    };

    let mut controller = spi::SPI::new(Sk6812Grbw, SIZE, config)?;

    let mut render = Render::default();

    let ctx = RenderContext {
        brightness: 0.5,
        gamma_factor: Some(2.8),
    };

    loop {
        controller.update(&render, &ctx)?;
        controller.send()?;

        thread::sleep(Duration::from_secs(1));

        render.next()
    }
}
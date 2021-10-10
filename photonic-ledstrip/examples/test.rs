use std::thread;
use std::time::Duration;

use anyhow::Result;

use photonic_core::color::Black;
use photonic_core::color::RGBColor;
use photonic_ledstrip::controllers::Controller;
use photonic_ledstrip::controllers::spi;
use photonic_ledstrip::{chips, LedStripOutput};
use photonic_ledstrip::LedStripOutputDecl;
use photonic_core::{OutputDecl, Output, Node, NodeDecl};
use photonic_core::node::{Render, RenderType};
use photonic_core::scene::NodeBuilder;

const SIZE: usize = 100;

struct Example {
    state: usize,
}

impl <'a> RenderType<'a, Self> for Example { type Render = &'a Self; }

impl NodeDecl for Example {
    type Element = RGBColor;

    type Target = Example;

    fn materialize(self, size: usize, builder: &mut NodeBuilder) -> Result<Self::Target> where Self::Target: Sized {
        return Ok(self);
    }
}

impl Node for Example {
    const KIND: &'static str = "example";

    type Element = RGBColor;

    fn update(&mut self, duration: Duration) -> Result<()> {
        return Ok(());
    }

    fn render(&self) -> Result<<Self as RenderType<Self>>::Render> {
        return Ok(&self);
    }
}

impl Render for &Example {
    type Element = RGBColor;

    fn get(&self, index: usize) -> Result<Self::Element> {
        let x = self.state / 6000;

        let b = (self.state % 6000) as f64 / 600.0; // 0 .. 1
        let b = b * 2.0 - 1.0; // -1 .. 1
        let b = 1.0 - f64::abs(b); // 0 .. 1 .. 0

        let colors: &[RGBColor] = &[
            RGBColor::new(1.0, 0.0, 0.0),
            RGBColor::new(1.0, 1.0, 0.0),
            RGBColor::new(0.0, 1.0, 0.0),
            RGBColor::new(0.0, 1.0, 1.0),
            RGBColor::new(0.0, 0.0, 1.0),
            RGBColor::new(1.0, 0.0, 1.0),
            RGBColor::new(1.0, 1.0, 1.0),
        ];

        return Ok(if index % 3 == x % 3 {
            colors[x % colors.len()] * b
        } else {
            RGBColor::new(0.0, 0.0, 0.0)
        });
    }
}

impl Default for Example {
    fn default() -> Self {
        return Self { state: 0 };
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let output: LedStripOutputDecl::<spi::SPI, chips::Sk6812Grbw> = LedStripOutputDecl {
        config: spi::Config {
            dev: "/dev/spidev0.0".into(),
        },
        brightness: 1.0,
        gamma_factor: None,
        correction: None
    };

    let mut output = OutputDecl::<Example>::materialize(output, SIZE)?;

    let mut render = Example::default();

    for state in 0.. {
        render.state = state;
        Output::<Example>::render(&mut output, &render)?;
    }

    return Ok(());
}
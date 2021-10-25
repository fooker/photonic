use std::thread;
use std::time::Duration;

use anyhow::Result;

use photonic_core::color::{Black, RGBColor};
use photonic_core::node::{Render, RenderType};
use photonic_core::scene::NodeBuilder;
use photonic_core::{Node, NodeDecl, Output, OutputDecl};
use photonic_ledstrip::controllers::{spi, Controller};
use photonic_ledstrip::{chips, LedStripOutput, LedStripOutputDecl};

const SIZE: usize = 100;

struct Example {
    state: usize,
}

impl<'a> RenderType<'a, Self> for Example {
    type Render = &'a Self;
}

impl NodeDecl for Example {
    type Element = RGBColor;

    type Target = Example;

    fn materialize(self, size: usize, builder: &mut NodeBuilder) -> Result<Self::Target>
    where
        Self::Target: Sized,
    {
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
        const CYCLE: usize = 600;

        let x = self.state / CYCLE;

        let b = (self.state % CYCLE) as f64 / CYCLE as f64; // 0 .. 1
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
        return Self {
            state: 0,
        };
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let output: LedStripOutputDecl<spi::SPI, chips::Sk6812Grbw> = LedStripOutputDecl {
        config: spi::Config {
            dev: "/dev/spidev0.0".into(),
        },
        brightness: 1.0,
        gamma_factor: Some(2.2),
        correction: None,
    };

    let mut output = OutputDecl::<Example>::materialize(output, SIZE)?;

    let mut render = Example::default();

    for state in 0.. {
        render.state = state;
        Output::<Example>::render(&mut output, &render)?;

        tokio::time::sleep(Duration::from_millis(10)).await;
    }

    return Ok(());
}

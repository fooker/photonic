use std::time::Duration;

use failure::Error;

use photonic_core::color::*;
use photonic_core::core::*;
use photonic_core::math;
use photonic_core::value::*;

pub struct AlertRenderer {
    hue: f64,
    block_size: usize,
    value: f64,
}

impl Render for AlertRenderer {
    type Element = HSVColor;

    fn get(&self, index: usize) -> Self::Element {
        let x = (index / self.block_size) % 2 == 0;

        return HSVColor::new(
            self.hue,
            1.0,
            if x { self.value } else { 1.0 - self.value },
        );
    }
}

pub struct AlertNodeDecl<Hue, BlockSize, Speed> {
    pub hue: Hue,
    pub block_size: BlockSize,
    pub speed: Speed,
}

pub struct AlertNode<Hue, BlockSize, Speed> {
    hue: Hue,
    block_size: BlockSize,
    speed: Speed,

    time: f64,
}

impl<Hue, BlockSize, Speed> NodeDecl for AlertNodeDecl<Hue, BlockSize, Speed>
    where Hue: BoundValueDecl<f64>,
          BlockSize: BoundValueDecl<usize>,
          Speed: UnboundValueDecl<f64> {
    type Element = HSVColor;
    type Target = AlertNode<Hue::Value, BlockSize::Value, Speed::Value>;

    fn new(self, size: usize) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            hue: self.hue.new((0.0, 360.0).into())?,
            block_size: self.block_size.new((0, size).into())?,
            speed: self.speed.new()?,

            time: 0.0,
        });
    }
}

impl<Hue, BlockSize, Speed> Dynamic for AlertNode<Hue, BlockSize, Speed>
    where Hue: Value<f64>,
          BlockSize: Value<usize>,
          Speed: Value<f64> {
    fn update(&mut self, duration: &Duration) {
        self.block_size.update(duration);
        self.speed.update(duration);

        self.time += duration.as_secs_f64() * self.speed.get();
    }
}

impl<'a, Hue, BlockSize, Speed> RenderType<'a> for AlertNode<Hue, BlockSize, Speed> {
    type Element = HSVColor;
    type Render = AlertRenderer;
}

impl<Hue, BlockSize, Speed> Node for AlertNode<Hue, BlockSize, Speed>
    where Hue: Value<f64>,
          BlockSize: Value<usize>,
          Speed: Value<f64> {
    fn render<'a>(&'a self, _renderer: &'a Renderer) -> <Self as RenderType<'a>>::Render {
        return AlertRenderer {
            hue: self.hue.get(),
            block_size: self.block_size.get(),
            value: math::remap(math::clamp(f64::sin(self.time * std::f64::consts::PI), (-1.0, 1.0)),
                               (-1.0, 1.0), (0.0, 1.0)),
        };
    }
}

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

pub struct AlertNodeDecl {
    pub hue: Box<BoundValueDecl<f64>>,
    pub block_size: Box<BoundValueDecl<usize>>,
    pub speed: Box<UnboundValueDecl<f64>>,
}

pub struct AlertNode {
    hue: Box<Value<f64>>,
    block_size: Box<Value<usize>>,
    speed: Box<Value<f64>>,

    time: f64,
}

impl NodeDecl for AlertNodeDecl {
    type Element = HSVColor;
    type Target = AlertNode;

    fn new(self, size: usize) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            hue: self.hue.new((0.0, 360.0).into())?,
            block_size: self.block_size.new((0, size).into())?,
            speed: self.speed.new()?,

            time: 0.0,
        });
    }
}

impl Dynamic for AlertNode {
    fn update(&mut self, duration: &Duration) {
        self.block_size.update(duration);
        self.speed.update(duration);

        self.time += duration.as_secs_f64() * self.speed.get();
    }
}

impl <'a> RenderType<'a> for AlertNode {
    type Element = HSVColor;
    type Render = AlertRenderer;
}

impl Node for AlertNode {
    fn render<'a>(&'a self, _renderer: &'a Renderer) -> <Self as RenderType<'a>>::Render {
        return AlertRenderer {
            hue: self.hue.get(),
            block_size: self.block_size.get(),
            value: math::remap(math::clamp(f64::sin(self.time * std::f64::consts::PI), (-1.0, 1.0)),
                               (-1.0, 1.0), (0.0, 1.0)),
        };
    }
}

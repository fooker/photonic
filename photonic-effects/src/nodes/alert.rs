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

pub struct AlertNodeDecl<Hue, Block, Speed> {
    pub hue: Hue,
    pub block: Block,
    pub speed: Speed,
}

pub struct AlertNode<Hue, Block, Speed> {
    hue: Hue,
    block: Block,
    speed: Speed,

    time: f64,
}

impl<Hue, Block, Speed> NodeDecl for AlertNodeDecl<Hue, Block, Speed>
    where Hue: BoundValueDecl<f64>,
          Block: BoundValueDecl<usize>,
          Speed: UnboundValueDecl<f64> {
    type Element = HSVColor;
    type Target = AlertNode<Hue::Value, Block::Value, Speed::Value>;

    fn materialize(self, size: usize, mut builder: SceneBuilder) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            hue: builder.bound_value("hue", self.hue, (0.0, 360.0))?,
            block: builder.bound_value("block", self.block, (0, size))?,
            speed: builder.unbound_value("speed", self.speed)?,

            time: 0.0,
        });
    }
}

impl<'a, Hue, Block, Speed> RenderType<'a> for AlertNode<Hue, Block, Speed> {
    type Element = HSVColor;
    type Render = AlertRenderer;
}

impl<Hue, Block, Speed> Node for AlertNode<Hue, Block, Speed>
    where Hue: Value<f64>,
          Block: Value<usize>,
          Speed: Value<f64> {
    fn update(&mut self, duration: &Duration) {
        self.block.update(duration);
        self.speed.update(duration);

        self.time += duration.as_secs_f64() * self.speed.get();
    }

    fn render<'a>(&'a self, _renderer: &'a Renderer) -> <Self as RenderType<'a>>::Render {
        return AlertRenderer {
            hue: self.hue.get(),
            block_size: self.block.get(),
            value: math::remap(math::clamp(f64::sin(self.time * std::f64::consts::PI), (-1.0, 1.0)),
                               (-1.0, 1.0), (0.0, 1.0)),
        };
    }
}

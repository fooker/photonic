use std::time::Duration;

use anyhow::Error;

use photonic_core::attr::{Attr, BoundAttrDecl, UnboundAttrDecl};
use photonic_core::color::HSVColor;
use photonic_core::math;
use photonic_core::node::{Node, NodeDecl, Render, RenderType};
use photonic_core::scene::NodeBuilder;

pub struct AlertRenderer {
    hue: f64,
    block_size: usize,
    value: f64,
}

impl Render for AlertRenderer {
    type Element = HSVColor;

    fn get(&self, index: usize) -> Self::Element {
        let x = (index / self.block_size) % 2 == 0;

        return HSVColor::new(self.hue, 1.0, if x { self.value } else { 1.0 - self.value });
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
where
    Hue: BoundAttrDecl<f64>,
    Block: BoundAttrDecl<i64>,
    Speed: UnboundAttrDecl<f64>,
{
    type Element = HSVColor;
    type Target = AlertNode<Hue::Target, Block::Target, Speed::Target>;

    fn materialize(self, size: usize, builder: &mut NodeBuilder) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            hue: builder.bound_attr("hue", self.hue, (0.0, 360.0))?,
            block: builder.bound_attr("block", self.block, (0, size as i64))?,
            speed: builder.unbound_attr("speed", self.speed)?,

            time: 0.0,
        });
    }
}

impl<'a, Hue, Block, Speed> RenderType<'a, Self> for AlertNode<Hue, Block, Speed>
where
    Hue: Attr<f64>,
    Block: Attr<i64>,
    Speed: Attr<f64>,
{
    type Render = AlertRenderer;
}

impl<Hue, Block, Speed> Node for AlertNode<Hue, Block, Speed>
where
    Hue: Attr<f64>,
    Block: Attr<i64>,
    Speed: Attr<f64>,
{
    const KIND: &'static str = "alert";

    type Element = HSVColor;

    fn update(&mut self, duration: Duration) {
        self.block.update(duration);
        self.speed.update(duration);

        self.time += duration.as_secs_f64() * self.speed.get();
    }

    fn render(&mut self) -> <Self as RenderType<Self>>::Render {
        return AlertRenderer {
            hue: self.hue.get(),
            block_size: self.block.get() as usize,
            value: math::remap(
                math::clamp(f64::sin(self.time * std::f64::consts::PI), (-1.0, 1.0)),
                (-1.0, 1.0),
                (0.0, 1.0),
            ),
        };
    }
}

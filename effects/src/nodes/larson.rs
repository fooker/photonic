use anyhow::Result;
use palette::Hsv;

use photonic::{BoundAttrDecl, Buffer, Node, NodeBuilder, RenderContext};
use photonic::attr::Attr;
use photonic::decl::{FreeAttrDecl, NodeDecl};

enum Direction {
    Positive,
    Negative,
}

pub struct Larson<Hue, Width, Speed> {
    pub hue: Hue,
    pub width: Width,
    pub speed: Speed,
}

pub struct LarsonNode<Hue, Width, Speed> {
    hue: Hue,
    width: Width,
    speed: Speed,

    position: f32,
    direction: Direction,
}

impl<Hue, Width, Speed> NodeDecl for Larson<Hue, Width, Speed>
    where
        Hue: BoundAttrDecl<Value=f32>,
        Width: BoundAttrDecl<Value=f32>,
        Speed: FreeAttrDecl<Value=f32>,
{
    type Node = LarsonNode<Hue::Attr, Width::Attr, Speed::Attr>;

    async fn materialize(self, builder: &mut NodeBuilder<'_>) -> Result<Self::Node> {
        return Ok(Self::Node {
            hue: builder.bound_attr("hue", self.hue, (0.0, 360.0))?,
            width: builder.bound_attr("width", self.width, (0.0, builder.size as f32))?,
            speed: builder.unbound_attr("speed", self.speed)?,
            position: 0.0,
            direction: Direction::Positive,
        });
    }
}

impl<Hue, Width, Speed> Node for LarsonNode<Hue, Width, Speed>
    where
        Hue: Attr<Value=f32>,
        Width: Attr<Value=f32>,
        Speed: Attr<Value=f32>,
{
    const KIND: &'static str = "solid";

    type Element = Hsv;

    fn update(&mut self, ctx: &RenderContext, out: &mut Buffer<Self::Element>) -> Result<()> {
        let hue = self.hue.update(ctx.duration);
        let width = self.width.update(ctx.duration);
        let speed = self.speed.update(ctx.duration);

        let size = (out.size() - 1) as f32;
        let delta = ctx.duration.as_secs_f32() * speed;

        match self.direction {
            Direction::Positive => {
                self.position += delta;
                if self.position >= size {
                    self.position = size - (self.position - size); // After over-shooting the side, move back for that amount
                    self.direction = Direction::Negative;
                }
            }

            Direction::Negative => {
                self.position -= delta;
                if self.position <= 0.0 {
                    self.position = -self.position; // After over-shooting the side, move back for that amount
                    self.direction = Direction::Positive;
                }
            }
        }

        out.update(|i, _| {
            // Calculate value as the linear distance between the pixel and the current position
            // scaled from 0.0 at ±width/2 to 1.0 at center
            let value = (((width / 2.0) - f32::abs((i as f32) - self.position)) / (width / 2.0)).max(0.0);

            return Hsv::new(hue, 1.0, value);
        });

        return Ok(());
    }
}

#[cfg(feature = "dynamic")]
pub mod dynamic {
    use serde::Deserialize;

    use photonic_dynamic::{BoxedBoundAttrDecl, BoxedFreeAttrDecl, config};
    use photonic_dynamic::factory::Producible;

    use super::*;

    #[derive(Deserialize, Debug)]
    pub struct Config {
        pub hue: config::Attr<f32>,
        pub width: config::Attr<f32>,
        pub speed: config::Attr<f32>,
    }

    impl Producible for Larson<BoxedBoundAttrDecl<f32>, BoxedBoundAttrDecl<f32>, BoxedFreeAttrDecl<f32>> {
        type Config = Config;
    }

    pub fn node<B>(config: Config, builder: &mut B) -> Result<Larson<BoxedBoundAttrDecl<f32>, BoxedBoundAttrDecl<f32>, BoxedFreeAttrDecl<f32>>>
        where
            B: photonic_dynamic::NodeBuilder,
    {
        return Ok(Larson {
            hue: builder.bound_attr("hue", config.hue)?,
            width: builder.bound_attr("width", config.width)?,
            speed: builder.free_attr("speed", config.speed)?,
        });
    }
}

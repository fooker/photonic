use std::ops::Neg;

use anyhow::Result;
use palette::Hsv;

use photonic::{Attr, BoundAttrDecl, Buffer, FreeAttrDecl, math, Node, NodeBuilder, NodeDecl, RenderContext};

pub struct Alert<Hue, Block, Speed> {
    pub hue: Hue,
    pub block: Block,
    pub speed: Speed,
}

pub struct AlertNode<Hue, Block, Speed> {
    hue: Hue,
    block: Block,
    speed: Speed,

    time: f32,
}

impl<Hue, Block, Speed> NodeDecl for Alert<Hue, Block, Speed>
    where
        Hue: BoundAttrDecl<Value=f32>,
        Block: BoundAttrDecl<Value=i64>,
        Speed: FreeAttrDecl<Value=f32>, // TODO: Make speed an attr of duration
{
    type Node = AlertNode<Hue::Attr, Block::Attr, Speed::Attr>;

    async fn materialize(self, builder: &mut NodeBuilder<'_>) -> Result<Self::Node> {
        return Ok(Self::Node {
            hue: builder.bound_attr("hue", self.hue, (0.0, 360.0))?,
            block: builder.bound_attr("block", self.block, (0, builder.size as i64))?,
            speed: builder.unbound_attr("speed", self.speed)?,

            time: 0.0,
        });
    }
}

impl<Hue, Block, Speed> Node for AlertNode<Hue, Block, Speed>
    where
        Hue: Attr<Value=f32>,
        Block: Attr<Value=i64>,
        Speed: Attr<Value=f32>,
{
    const KIND: &'static str = "alert";

    type Element = Hsv;

    fn update(&mut self, ctx: &RenderContext, out: &mut Buffer<Self::Element>) -> Result<()> {
        let hue = self.hue.update(ctx.duration);
        let block = self.block.update(ctx.duration);
        let speed = self.speed.update(ctx.duration);

        self.time += ctx.duration.as_secs_f32() / speed;
        self.time %= 2.0;

        let value = math::clamp(f32::sin(self.time * std::f32::consts::PI), (-1.0, 1.0));

        out.update(|i, _| {
            let x = (i / block as usize) % 2 == 0;

            return Hsv::new(hue, 1.0, if x { value } else { value.neg() }.max(0.0));
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
        pub block: config::Attr<i64>,
        pub speed: config::Attr<f32>,
    }

    impl Producible for Alert<BoxedBoundAttrDecl<f32>, BoxedBoundAttrDecl<i64>, BoxedFreeAttrDecl<f32>> {
        type Config = Config;
    }

    pub fn node<B>(config: Config, builder: &mut B) -> Result<Alert<BoxedBoundAttrDecl<f32>, BoxedBoundAttrDecl<i64>, BoxedFreeAttrDecl<f32>>>
        where
            B: photonic_dynamic::NodeBuilder,
    {
        return Ok(Alert {
            hue: builder.bound_attr("hue", config.hue)?,
            block: builder.bound_attr("block", config.block)?,
            speed: builder.free_attr("speed", config.speed)?,
        });
    }
}

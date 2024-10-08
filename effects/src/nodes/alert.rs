use std::ops::Neg;

use anyhow::Result;
use palette::Hsv;

use photonic::{math, Attr, BoundAttrDecl, Buffer, FreeAttrDecl, Node, NodeBuilder, NodeDecl, RenderContext};

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
    Hue: BoundAttrDecl<f32>,
    Block: BoundAttrDecl<i64>,
    Speed: FreeAttrDecl<f32>, // TODO: Make speed an attr of duration
{
    const KIND: &'static str = "alert";

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
    Hue: Attr<f32>,
    Block: Attr<i64>,
    Speed: Attr<f32>,
{
    type Element = Hsv;

    fn update(&mut self, ctx: &RenderContext, out: &mut Buffer<Self::Element>) -> Result<()> {
        let hue = self.hue.update(ctx);
        let block = self.block.update(ctx);
        let speed = self.speed.update(ctx);

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
    use palette::rgb::Rgb;
    use serde::Deserialize;

    use photonic::boxed::{BoxedBoundAttrDecl, BoxedFreeAttrDecl, DynNodeDecl};
    use photonic_dynamic::factory::Producible;
    use photonic_dynamic::registry::Registry;
    use photonic_dynamic::{builder, config};

    use super::*;

    #[derive(Deserialize, Debug)]
    pub struct Config {
        pub hue: config::Attr<f32>,
        pub block: config::Attr<i64>,
        pub speed: config::Attr<f32>,
    }

    impl Producible<dyn DynNodeDecl<Rgb>> for Config {
        type Product = Alert<BoxedBoundAttrDecl<f32>, BoxedBoundAttrDecl<i64>, BoxedFreeAttrDecl<f32>>;

        fn produce<Reg: Registry>(config: Self, mut builder: builder::NodeBuilder<'_, Reg>) -> Result<Self::Product> {
            return Ok(Alert {
                hue: builder.bound_attr("hue", config.hue)?,
                block: builder.bound_attr("block", config.block)?,
                speed: builder.free_attr("speed", config.speed)?,
            });
        }
    }
}

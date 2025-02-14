use anyhow::Result;
use palette::{FromColor, Hsl};

use photonic::attr::range::Range;
use photonic::attr::{Attr, Bounds};
use photonic::decl::{BoundAttrDecl, FreeAttrDecl, NodeDecl};
use photonic::{Buffer, Node, NodeBuilder, Random, RenderContext};

#[derive(Default)]
struct Raindrop {
    color: Hsl,
    decay: f32,
}

pub struct Raindrops<Rate, Color, Decay> {
    pub rate: Rate,
    pub color: Color,
    pub decay: Decay,
}

pub struct RaindropsNode<Rate, Color, Decay>
where
    Rate: Attr<f32>,
    Color: Attr<Range<Hsl>>,
    Decay: Attr<Range<f32>>,
{
    rate: Rate,
    color: Color,
    decay: Decay,

    drops: Box<[Raindrop]>,

    random: Random,
}

impl<Rate, Color, Decay> NodeDecl for Raindrops<Rate, Color, Decay>
where
    Rate: BoundAttrDecl<f32>,
    Color: FreeAttrDecl<Range<Hsl>>,
    Decay: FreeAttrDecl<Range<f32>>,
{
    const KIND: &'static str = "raindrops";

    type Node = RaindropsNode<Rate::Attr, Color::Attr, Decay::Attr>;

    async fn materialize(self, builder: &mut NodeBuilder<'_>) -> Result<Self::Node> {
        return Ok(Self::Node {
            rate: builder.bound_attr("rate", self.rate, Bounds::normal())?,
            color: builder.unbound_attr("color", self.color)?,
            decay: builder.unbound_attr("decay", self.decay)?,
            drops: (0..builder.size).map(|_| Raindrop::default()).collect::<Vec<_>>().into_boxed_slice(),
            random: Random::new(),
        });
    }
}

impl<Rate, Color, Decay> Node for RaindropsNode<Rate, Color, Decay>
where
    Rate: Attr<f32>,
    Color: Attr<Range<Hsl>>,
    Decay: Attr<Range<f32>>,
{
    type Element = Hsl;

    fn update(&mut self, ctx: &RenderContext, out: &mut Buffer<Self::Element>) -> Result<()> {
        let rate = self.rate.update(ctx);
        let color = self.color.update(ctx);
        let decay = self.decay.update(ctx);

        for (out, drop) in Iterator::zip(out.iter_mut(), self.drops.iter_mut()) {
            if self.random.rate(rate.into(), ctx.duration) {
                drop.color = self.random.mix(Hsl::from_color(color.0), Hsl::from_color(color.1));
                drop.decay = self.random.range(decay.0, decay.1);
            } else {
                let decay = f32::exp(-drop.decay * ctx.duration.as_secs_f32());
                drop.color.lightness *= decay;
            }

            *out = drop.color;
            // TODO: Evaluate
            // Can we inline the drops buffer with the output buffer by setting Element to Raindrop and having an
            // interface that extracts the actual color value?
        }

        return Ok(());
    }
}

#[cfg(feature = "dynamic")]
pub mod dynamic {
    use palette::rgb::Rgb;
    use serde::Deserialize;

    use photonic::attr::FreeAttrDeclExt;
    use photonic::boxed::{BoxedBoundAttrDecl, BoxedFreeAttrDecl, DynNodeDecl};
    use photonic_dynamic::factory::Producible;
    use photonic_dynamic::registry::Registry;
    use photonic_dynamic::{builder, config};

    use super::*;

    #[derive(Deserialize, Debug)]
    pub struct Config {
        pub rate: config::Attr<f32>,
        pub color: config::Attr<Range<Rgb>>,
        pub decay: config::Attr<Range<f32>>,
    }

    type BoxedRaindrops =
        Raindrops<BoxedBoundAttrDecl<f32>, BoxedFreeAttrDecl<Range<Hsl>>, BoxedFreeAttrDecl<Range<f32>>>;

    impl Producible<dyn DynNodeDecl<Rgb>> for Config {
        type Product = BoxedRaindrops;
        fn produce<Reg: Registry>(config: Self, mut builder: builder::NodeBuilder<'_, Reg>) -> Result<Self::Product> {
            return Ok(Raindrops {
                rate: builder.bound_attr("rate", config.rate)?,
                color: Box::new(
                    builder.free_attr::<Range<Rgb>>("color", config.color)?.map(|range| range.map(Hsl::from_color)),
                ), // TODO: Remove box?
                decay: builder.free_attr("decay", config.decay)?,
            });
        }
    }
}

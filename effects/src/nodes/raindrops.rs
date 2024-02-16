use palette::Hsl;
use palette::rgb::Rgb;

use photonic::{Buffer, Context, Node, NodeBuilder, Random};
use photonic::attr::{Bounds, Attr};
use photonic::decl::{BoundAttrDecl, FreeAttrDecl, NodeDecl};
use photonic::attr::range::Range;
use palette::FromColor;

use anyhow::Result;
use photonic_dyn::DynamicNode;

#[derive(Default)]
struct Raindrop {
    color: Hsl,
    decay: f32,
}

#[derive(DynamicNode)]
pub struct Raindrops<Rate, Color, Decay> {
    #[photonic(attr)]
    pub rate: Rate,

    #[photonic(attr)]
    pub color: Color,

    #[photonic(attr)]
    pub decay: Decay,
}

pub struct RaindropsNode<Rate, Color, Decay>
    where Rate: Attr<Value=f32>,
          Color: Attr<Value=Range<Rgb>>,
          Decay: Attr<Value=Range<f32>>,
{
    rate: Rate,
    color: Color,
    decay: Decay,

    drops: Box<[Raindrop]>,

    random: Random,
}

impl<Rate, Color, Decay> NodeDecl for Raindrops<Rate, Color, Decay>
    where Rate: BoundAttrDecl<Value=f32>,
          Color: FreeAttrDecl<Value=Range<Rgb>>,
          Decay: BoundAttrDecl<Value=Range<f32>>,
{
    type Node = RaindropsNode<Rate::Attr, Color::Attr, Decay::Attr>;

    async fn materialize(self, builder: &mut NodeBuilder<'_>) -> Result<Self::Node> {
        return Ok(Self::Node {
            rate: builder.bound_attr("rate", self.rate, Bounds::normal())?,
            color: builder.unbound_attr("color", self.color)?,
            decay: builder.bound_attr("decay", self.decay, Bounds::normal())?,
            drops: (0..builder.size).map(|_| Raindrop::default()).collect::<Vec<_>>().into_boxed_slice(),
            random: Random::new(),
        });
    }
}

impl<Rate, Color, Decay> Node for RaindropsNode<Rate, Color, Decay>
    where Rate: Attr<Value=f32>,
          Color: Attr<Value=Range<Rgb>>,
          Decay: Attr<Value=Range<f32>>,
{
    const KIND: &'static str = "raindrops";

    type Element = Hsl;

    fn update(&mut self, ctx: &Context, out: &mut Buffer<Self::Element>) -> Result<()> {
        let rate = self.rate.update(ctx.duration);
        let color = self.color.update(ctx.duration);
        let decay = self.decay.update(ctx.duration);

        for (out, drop) in Iterator::zip(out.iter_mut(), self.drops.iter_mut()) {
            if self.random.rate(rate.into(), ctx.duration) {
                drop.color = self.random.mix(Hsl::from_color(color.0), Hsl::from_color(color.1));
                drop.decay = self.random.range(decay.0, decay.1);
            } else {
                drop.color.lightness = f32::max(
                    0.0,
                    drop.color.lightness * 1.0 - drop.decay * ctx.duration.as_secs_f32(),
                );
            }

            *out = drop.color;
            // TODO: Evaluate
            // Can we inline the drops buffer with the output buffer by setting Element to Raindrop and having an
            // interface that extracts the actual color value?
        }

        return Ok(());
    }
}
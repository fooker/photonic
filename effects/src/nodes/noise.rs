use anyhow::Result;
use noise::NoiseFn;
use palette::Lch;

use photonic::attr::Attr;
use photonic::decl::{FreeAttrDecl, NodeDecl};
use photonic::{Buffer, Context, Node, NodeBuilder};
use photonic_dyn::DynamicNode;

// TODO: Hue range

#[derive(DynamicNode)]
pub struct Noise<Speed, Stretch, F> {
    #[photonic(attr)]
    pub speed: Speed,

    #[photonic(attr)]
    pub stretch: Stretch,

    pub noise: F,
}

pub struct NoiseNode<Speed, Stretch, F>
where
    Speed: Attr<Value = f32>,
    Stretch: Attr<Value = f32>,
    F: NoiseFn<f64, 2>
{
    speed: Speed,
    stretch: Stretch,

    position: f64,

    noise: F,
}

impl<Speed, Stretch, F> NodeDecl for Noise<Speed, Stretch, F>
where
    Speed: FreeAttrDecl<Value = f32>,
    Stretch: FreeAttrDecl<Value = f32>,
    F: NoiseFn<f64, 2>,
{
    type Node = NoiseNode<Speed::Attr, Stretch::Attr, F>;

    async fn materialize(self, builder: &mut NodeBuilder<'_>) -> Result<Self::Node> {
        return Ok(Self::Node {
            speed: builder.unbound_attr("speed", self.speed)?,
            stretch: builder.unbound_attr("stretch", self.stretch)?,
            position: 0.0,
            noise: self.noise,
        });
    }
}

impl<Speed, Stretch, F> Node for NoiseNode<Speed, Stretch, F>
where
    Speed: Attr<Value = f32>,
    Stretch: Attr<Value = f32>,
    F: NoiseFn<f64, 2>,
{
    const KIND: &'static str = "noise";

    type Element = Lch;

    fn update(&mut self, ctx: &Context, out: &mut Buffer<Self::Element>) -> Result<()> {
        let speed = self.speed.update(ctx.duration);
        let stretch = self.stretch.update(ctx.duration);

        self.position += ctx.duration.as_secs_f64() * speed as f64;

        out.update(|i, _| {
            let hue = self.noise.get([self.position, i as f64 * stretch as f64]) * 360.0;
            Lch::new(50.0, 128.0, hue as f32)
        });

        return Ok(());
    }
}

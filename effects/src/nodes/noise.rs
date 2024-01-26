use anyhow::Result;
use noise::NoiseFn;
use palette::{Hsv, Lch};

use photonic::{Buffer, Context, Node, NodeBuilder};
use photonic::attr::Attr;
use photonic::decl::{FreeAttrDecl, NodeDecl};

pub struct Noise<Speed, Stretch> {
    pub speed: Speed,
    pub stretch: Stretch,
}

pub struct NoiseNode<Speed, Stretch>
    where Speed: Attr<Value=f32>,
          Stretch: Attr<Value=f32>,
{
    speed: Speed,
    stretch: Stretch,

    position: f64,

    noise: noise::Perlin,
}

impl<Speed, Stretch> NodeDecl for Noise<Speed, Stretch>
    where Speed: FreeAttrDecl<Value=f32>,
          Stretch: FreeAttrDecl<Value=f32>,
{
    type Node = NoiseNode<Speed::Attr, Stretch::Attr>;

    async fn materialize(self, builder: &mut NodeBuilder<'_>) -> Result<Self::Node> {
        return Ok(Self::Node {
            speed: builder.unbound_attr("speed", self.speed)?,
            stretch: builder.unbound_attr("stretch", self.stretch)?,
            position: 0.0,
            noise: noise::Perlin::default(),
        });
    }
}

impl<Speed, Stretch> Node for NoiseNode<Speed, Stretch>
    where Speed: Attr<Value=f32>,
          Stretch: Attr<Value=f32>,
{
    const KIND: &'static str = "noise";

    type Element = Lch;

    fn update(&mut self, ctx: &Context, out: &mut Buffer<Self::Element>) -> anyhow::Result<()> {
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
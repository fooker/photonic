use anyhow::Result;
use noise::NoiseFn;
use palette::Lch;
use serde::Deserialize;

use photonic::attr::Attr;
use photonic::decl::{FreeAttrDecl, NodeDecl};
use photonic::{Buffer, Node, NodeBuilder, RenderContext};
use photonic_dynamic::boxed::DynNodeDecl;
use photonic_dynamic::{config, NodeFactory};

// TODO: Color range and noise lerps between colors

pub struct Noise<Speed, Stretch, F> {
    pub speed: Speed,
    pub stretch: Stretch,
    pub noise: F,
}

pub struct NoiseNode<Speed, Stretch, F>
where
    Speed: Attr<Value = f32>,
    Stretch: Attr<Value = f32>,
    F: NoiseFn<f64, 2>,
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

    fn update(&mut self, ctx: &RenderContext, out: &mut Buffer<Self::Element>) -> Result<()> {
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

#[cfg(feature = "dynamic")]
pub fn factory<B>() -> Box<NodeFactory<B>>
where B: photonic_dynamic::NodeBuilder {
    #[derive(Deserialize, Debug)]
    enum Noise {
        Checkerboard,
        Cylinders,
        OpenSimplex,
        Perlin,
        PerlinSurflet,
        Simplex,
        SuperSimplex,
        Value,
        Worley,
    }

    impl Noise {
        fn into(self) -> Box<dyn NoiseFn<f64, 2>> {
            return match self {
                Noise::Checkerboard => Box::new(noise::Checkerboard::default()),
                Noise::Cylinders => Box::new(noise::Cylinders::default()),
                Noise::OpenSimplex => Box::new(noise::OpenSimplex::default()),
                Noise::Perlin => Box::new(noise::Perlin::default()),
                Noise::PerlinSurflet => Box::new(noise::PerlinSurflet::default()),
                Noise::Simplex => Box::new(noise::Simplex::default()),
                Noise::SuperSimplex => Box::new(noise::SuperSimplex::default()),
                Noise::Value => Box::new(noise::Value::default()),
                Noise::Worley => Box::new(noise::Worley::default()),
            };
        }
    }

    #[derive(Deserialize, Debug)]
    struct Config {
        pub speed: config::Attr,
        pub stretch: config::Attr,
        pub noise: Noise,
    }

    return Box::new(|config: config::Anything, builder: &mut B| {
        let config: Config = Deserialize::deserialize(config)?;

        return Ok(Box::new(self::Noise {
            speed: builder.free_attr("speed", config.speed)?,
            stretch: builder.free_attr("stretch", config.stretch)?,
            noise: config.noise.into(),
        }) as Box<dyn DynNodeDecl>);
    });
}

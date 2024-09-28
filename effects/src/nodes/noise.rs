use anyhow::Result;
use noise::NoiseFn;
use palette::Lch;

use photonic::attr::Attr;
use photonic::decl::{FreeAttrDecl, NodeDecl};
use photonic::{Buffer, Node, NodeBuilder, RenderContext};

// TODO: Color range and noise lerps between colors

pub struct Noise<Speed, Stretch, F> {
    pub speed: Speed,
    pub stretch: Stretch,
    pub noise: F,
}

pub struct NoiseNode<Speed, Stretch, F>
where
    Speed: Attr<f32>,
    Stretch: Attr<f32>,
    F: NoiseFn<f64, 2>,
{
    speed: Speed,
    stretch: Stretch,

    position: f64,

    noise: F,
}

impl<Speed, Stretch, F> NodeDecl for Noise<Speed, Stretch, F>
where
    Speed: FreeAttrDecl<f32>,
    Stretch: FreeAttrDecl<f32>,
    F: NoiseFn<f64, 2>,
{
    const KIND: &'static str = "noise";

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
    Speed: Attr<f32>,
    Stretch: Attr<f32>,
    F: NoiseFn<f64, 2>,
{
    type Element = Lch;

    fn update(&mut self, ctx: &RenderContext, out: &mut Buffer<Self::Element>) -> Result<()> {
        let speed = self.speed.update(ctx);
        let stretch = self.stretch.update(ctx);

        self.position += ctx.duration.as_secs_f64() * speed as f64;

        out.update(|i, _| {
            let hue = self.noise.get([self.position, i as f64 * stretch as f64]) * 360.0;
            Lch::new(50.0, 128.0, hue as f32)
        });

        return Ok(());
    }
}

#[cfg(feature = "dynamic")]
pub mod dynamic {
    use serde::Deserialize;

    use photonic::boxed::{BoxedFreeAttrDecl, DynNodeDecl};
    use photonic_dynamic::factory::Producible;
    use photonic_dynamic::registry::Registry;
    use photonic_dynamic::{builder, config};

    use super::*;

    #[derive(Deserialize, Debug)]
    #[serde(rename_all(deserialize = "snake_case"))]
    pub enum Noises {
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

    impl Noises {
        fn into(self) -> Box<dyn NoiseFn<f64, 2>> {
            return match self {
                Noises::Checkerboard => Box::new(noise::Checkerboard::default()),
                Noises::Cylinders => Box::new(noise::Cylinders::default()),
                Noises::OpenSimplex => Box::new(noise::OpenSimplex::default()),
                Noises::Perlin => Box::new(noise::Perlin::default()),
                Noises::PerlinSurflet => Box::new(noise::PerlinSurflet::default()),
                Noises::Simplex => Box::new(noise::Simplex::default()),
                Noises::SuperSimplex => Box::new(noise::SuperSimplex::default()),
                Noises::Value => Box::new(noise::Value::default()),
                Noises::Worley => Box::new(noise::Worley::default()),
            };
        }
    }

    #[derive(Deserialize, Debug)]
    pub struct Config {
        pub speed: config::Attr<f32>,
        pub stretch: config::Attr<f32>,
        pub noise: Noises,
    }

    type BoxedNoise = Noise<BoxedFreeAttrDecl<f32>, BoxedFreeAttrDecl<f32>, Box<dyn NoiseFn<f64, 2>>>;

    impl Producible<dyn DynNodeDecl> for Config {
        type Product = BoxedNoise;
        fn produce<Reg: Registry>(config: Self, mut builder: builder::NodeBuilder<'_, Reg>) -> Result<Self::Product> {
            return Ok(Noise {
                speed: builder.free_attr("speed", config.speed)?,
                stretch: builder.free_attr("stretch", config.stretch)?,
                noise: config.noise.into(),
            });
        }
    }
}

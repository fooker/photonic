use noise::NoiseFn;

use photonic::attr::{Bounded, Bounds};
use photonic::math::Lerp;
use photonic::{Attr, AttrBuilder, AttrValue, BoundAttrDecl, FreeAttrDecl, RenderContext};

pub struct Noise<Speed, F> {
    pub speed: Speed,
    pub noise: F,
}

pub struct NoiseAttr<V, Speed, F>
where
    V: AttrValue + Lerp,
    Speed: Attr<f64>,
    F: NoiseFn<f64, 1>,
{
    speed: Speed,

    bounds: Bounds<V>,

    position: f64,

    noise: F,
}

impl<V, Speed, F> BoundAttrDecl<V> for Noise<Speed, F>
where
    V: AttrValue + Bounded + Lerp,
    Speed: FreeAttrDecl<f64>,
    F: NoiseFn<f64, 1>,
{
    const KIND: &'static str = "noise";

    type Attr = NoiseAttr<V, Speed::Attr, F>;

    fn materialize(self, bounds: Bounds<V>, builder: &mut AttrBuilder) -> anyhow::Result<Self::Attr> {
        return Ok(Self::Attr {
            speed: builder.unbound_attr("speed", self.speed)?,
            position: 0.0,
            noise: self.noise,
            bounds,
        });
    }
}

impl<V, Speed, F> Attr<V> for NoiseAttr<V, Speed, F>
where
    V: AttrValue + PartialEq + Lerp,
    Speed: Attr<f64>,
    F: NoiseFn<f64, 1>,
{
    fn update(&mut self, ctx: &RenderContext) -> V {
        let speed = self.speed.update(ctx);

        self.position += ctx.duration.as_secs_f64() * speed;

        let value = self.noise.get([self.position]);
        let value = V::lerp(self.bounds.min, self.bounds.max, value as f32);

        return value;
    }
}

#[cfg(feature = "dynamic")]
pub mod dynamic {
    use anyhow::bail;
    use photonic::{input, AttrValue};
    use serde::de::DeserializeOwned;
    use serde::Deserialize;

    use photonic::boxed::{BoxedFreeAttrDecl, DynBoundAttrDecl};
    use photonic_dynamic::factory::Producible;
    use photonic_dynamic::registry::Registry;
    use photonic_dynamic::{builder, config};

    use super::*;

    #[derive(Deserialize, Debug)]
    pub struct Config {
        pub speed: config::Attr<f64>,
    }

    type BoxedNoise = Noise<BoxedFreeAttrDecl<f64>, noise::Perlin>;

    impl<V> Producible<dyn DynBoundAttrDecl<V>> for Config
    where V: AttrValue + input::Coerced + DeserializeOwned + Bounded
    {
        default type Product = !;

        default fn produce<Reg: Registry>(
            _config: Self,
            _builder: builder::AttrBuilder<'_, Reg>,
        ) -> anyhow::Result<Self::Product> {
            bail!("Attribute 'noise' no available for value type {}", std::any::type_name::<V>());
        }
    }

    impl<V> Producible<dyn DynBoundAttrDecl<V>> for Config
    where V: AttrValue + input::Coerced + DeserializeOwned + Bounded + Lerp
    {
        type Product = BoxedNoise;
        fn produce<Reg: Registry>(
            config: Self,
            mut builder: builder::AttrBuilder<'_, Reg>,
        ) -> anyhow::Result<Self::Product> {
            return Ok(Self::Product {
                speed: builder.free_attr("speed", config.speed)?,
                noise: noise::Perlin::default(),
            });
        }
    }
}

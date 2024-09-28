use anyhow::Result;
use num_traits::Float;

use photonic::attr::{Bounded, Bounds};
use photonic::{scene, Attr, AttrBuilder, AttrValue, BoundAttrDecl, FreeAttrDecl};

pub struct PeakAttr<V, Input>
where
    V: AttrValue + PartialEq + Float,
    Input: Attr<V>,
{
    input: Input,
    falloff: V,

    peak: V,
}

impl<V, Input> Attr<V> for PeakAttr<V, Input>
where
    V: AttrValue + PartialEq + Float,
    Input: Attr<V>,
{
    fn update(&mut self, ctx: &scene::RenderContext) -> V {
        let curr = self.input.update(ctx);

        self.peak = self.peak - self.falloff * V::from(ctx.duration.as_secs_f64()).expect("Can cast");
        self.peak = self.peak.max(curr);

        return self.peak;
    }
}

pub struct Peak<Input, V> {
    pub input: Input,
    pub falloff: V,
}

impl<Input, V> BoundAttrDecl<V> for Peak<Input, V>
where
    V: AttrValue + PartialEq + Float + Bounded,
    Input: BoundAttrDecl<V>,
{
    const KIND: &'static str = "fader";

    type Attr = PeakAttr<V, Input::Attr>;

    fn materialize(self, bounds: Bounds<V>, builder: &mut AttrBuilder) -> Result<Self::Attr> {
        let input = builder.bound_attr("input", self.input, bounds)?;

        return Ok(PeakAttr {
            input,
            falloff: self.falloff,
            peak: V::zero(),
        });
    }
}

impl<Input, V> FreeAttrDecl<V> for Peak<Input, V>
where
    V: AttrValue + PartialEq + Float,
    Input: FreeAttrDecl<V>,
{
    const KIND: &'static str = "fader";

    type Attr = PeakAttr<V, Input::Attr>;

    fn materialize(self, builder: &mut AttrBuilder) -> Result<Self::Attr> {
        let input = builder.unbound_attr("input", self.input)?;

        return Ok(PeakAttr {
            input,
            falloff: self.falloff,
            peak: V::zero(),
        });
    }
}

#[cfg(feature = "dynamic")]
pub mod dynamic {
    use anyhow::bail;

    use serde::de::DeserializeOwned;
    use serde::Deserialize;

    use photonic::boxed::{BoxedBoundAttrDecl, BoxedFreeAttrDecl, DynBoundAttrDecl, DynFreeAttrDecl};
    use photonic::input;
    use photonic_dynamic::factory::Producible;
    use photonic_dynamic::registry::Registry;
    use photonic_dynamic::{builder, config};

    use super::*;

    #[derive(Deserialize, Debug)]
    pub struct Config<V>
    where V: AttrValue
    {
        pub input: config::Attr<V>,
        pub falloff: V,
    }

    impl<V> Producible<dyn DynFreeAttrDecl<V>> for Config<V>
    where V: AttrValue + input::Coerced + DeserializeOwned
    {
        default type Product = !;

        default fn produce<Reg: Registry>(
            _config: Self,
            _builder: builder::AttrBuilder<'_, Reg>,
        ) -> Result<Self::Product> {
            bail!("Attribute 'fader' no available for value type {}", std::any::type_name::<V>());
        }
    }

    impl<V> Producible<dyn DynFreeAttrDecl<V>> for Config<V>
    where V: AttrValue + input::Coerced + DeserializeOwned + Float
    {
        type Product = Peak<BoxedFreeAttrDecl<V>, V>;

        fn produce<Reg: Registry>(config: Self, mut builder: builder::AttrBuilder<'_, Reg>) -> Result<Self::Product> {
            return Ok(Peak {
                input: builder.free_attr("input", config.input)?,
                falloff: config.falloff,
            });
        }
    }

    impl<V> Producible<dyn DynBoundAttrDecl<V>> for Config<V>
    where V: AttrValue + input::Coerced + DeserializeOwned + Bounded
    {
        default type Product = !;

        default fn produce<Reg: Registry>(
            _config: Self,
            _builder: builder::AttrBuilder<'_, Reg>,
        ) -> Result<Self::Product> {
            bail!("Attribute 'fader' no available for value type {}", std::any::type_name::<V>());
        }
    }

    impl<V> Producible<dyn DynBoundAttrDecl<V>> for Config<V>
    where V: AttrValue + input::Coerced + DeserializeOwned + Bounded + Float
    {
        type Product = Peak<BoxedBoundAttrDecl<V>, V>;

        fn produce<Reg: Registry>(config: Self, mut builder: builder::AttrBuilder<'_, Reg>) -> Result<Self::Product> {
            return Ok(Peak {
                input: builder.bound_attr("input", config.input)?,
                falloff: config.falloff,
            });
        }
    }
}

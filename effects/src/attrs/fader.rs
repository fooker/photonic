use anyhow::Result;

use photonic::attr::{Bounded, Bounds};
use photonic::math::Lerp;
use photonic::{scene, Attr, AttrBuilder, AttrValue, BoundAttrDecl, FreeAttrDecl};

use crate::easing::Easing;

pub struct FaderAttr<V, Input>
where
    V: AttrValue + PartialEq + Lerp,
    Input: Attr<V>,
{
    input: Input,

    last: Option<V>,
    next: Option<V>,

    fade: f32,

    easing: Easing<f32>,
}

impl<V, Input> Attr<V> for FaderAttr<V, Input>
where
    V: AttrValue + PartialEq + Lerp,
    Input: Attr<V>,
{
    const KIND: &'static str = "fader";

    fn update(&mut self, ctx: &scene::RenderContext) -> V {
        let curr = self.input.update(ctx);

        let Some(last) = self.last else {
            // First cycle - set initial current value
            self.last = Some(curr);
            return curr;
        };

        if let Some(next) = self.next {
            // In transition
            self.fade += ctx.duration.as_secs_f32() / self.easing.speed.as_secs_f32();

            if self.fade >= 1.0 {
                // Transition finished
                self.last = Some(next);
                self.next = None;

                return next;
            }

            return Lerp::lerp(last, next, (self.easing.func)(self.fade));
        }

        if curr != last {
            // Start transition
            self.next = Some(curr);
            self.fade = 0.0;
        }

        return last;
    }
}

pub struct Fader<Input> {
    pub input: Input,

    pub easing: Easing<f32>,
}

impl<Input, V> BoundAttrDecl<V> for Fader<Input>
where
    V: AttrValue + PartialEq + Lerp + Bounded,
    Input: BoundAttrDecl<V>,
{
    type Attr = FaderAttr<V, Input::Attr>;

    fn materialize(self, bounds: Bounds<V>, builder: &mut AttrBuilder) -> Result<Self::Attr> {
        let input = builder.bound_attr("input", self.input, bounds)?;

        return Ok(FaderAttr {
            input,
            last: None,
            next: None,
            fade: 1.0,
            easing: self.easing,
        });
    }
}

impl<Input, V> FreeAttrDecl<V> for Fader<Input>
where
    V: AttrValue + PartialEq + Lerp,
    Input: FreeAttrDecl<V>,
{
    type Attr = FaderAttr<V, Input::Attr>;

    fn materialize(self, builder: &mut AttrBuilder) -> Result<Self::Attr> {
        let input = builder.unbound_attr("input", self.input)?;

        return Ok(FaderAttr {
            input,
            last: None,
            next: None,
            fade: 1.0,
            easing: self.easing,
        });
    }
}

#[cfg(feature = "dynamic")]
pub mod dynamic {
    use anyhow::bail;

    use photonic::input;
    use serde::de::DeserializeOwned;
    use serde::Deserialize;

    use photonic_dynamic::factory::Producible;
    use photonic_dynamic::registry::Registry;
    use photonic_dynamic::{builder, config, BoxedBoundAttrDecl, BoxedFreeAttrDecl, DynBoundAttrDecl, DynFreeAttrDecl};

    use super::*;

    #[derive(Deserialize, Debug)]
    pub struct Config<V>
    where V: AttrValue
    {
        pub input: config::Attr<V>,
        pub easing: Easing<f32>,
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
    where V: AttrValue + input::Coerced + DeserializeOwned + Lerp
    {
        type Product = Fader<BoxedFreeAttrDecl<V>>;

        fn produce<Reg: Registry>(config: Self, mut builder: builder::AttrBuilder<'_, Reg>) -> Result<Self::Product> {
            return Ok(Fader {
                input: builder.free_attr("input", config.input)?,
                easing: config.easing,
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
    where V: AttrValue + input::Coerced + DeserializeOwned + Bounded + Lerp
    {
        type Product = Fader<BoxedBoundAttrDecl<V>>;

        fn produce<Reg: Registry>(config: Self, mut builder: builder::AttrBuilder<'_, Reg>) -> Result<Self::Product> {
            return Ok(Fader {
                input: builder.bound_attr("input", config.input)?,
                easing: config.easing,
            });
        }
    }
}

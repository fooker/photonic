use std::time::Duration;

use anyhow::Result;

use photonic::attr::{Bounded, Bounds};
use photonic::math::Lerp;
use photonic::{scene, Attr, AttrBuilder, AttrValue, BoundAttrDecl, FreeAttrDecl};

use crate::easing::Easing;

pub struct FaderAttr<V, Input>
where
    V: AttrValue + PartialEq + Lerp,
    Input: Attr<Value = V>,
{
    input: Input,

    last: Option<V>,
    next: Option<V>,

    fade: f32,

    easing: Easing<f32>,
}

impl<V, Input> Attr for FaderAttr<V, Input>
where
    V: AttrValue + PartialEq + Lerp,
    Input: Attr<Value = V>,
{
    type Value = V;

    const KIND: &'static str = "fader";

    fn update(&mut self, ctx: &scene::RenderContext) -> Self::Value {
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

impl<Input, V> BoundAttrDecl for Fader<Input>
where
    V: AttrValue + PartialEq + Lerp + Bounded,
    Input: BoundAttrDecl<Value = V>,
{
    type Value = V;
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

impl<Input, V> FreeAttrDecl for Fader<Input>
where
    V: AttrValue + PartialEq + Lerp,
    Input: FreeAttrDecl<Value = V>,
{
    type Value = V;
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
    use photonic::input;
    use serde::de::DeserializeOwned;
    use serde::Deserialize;

    use photonic_dynamic::factory::Producible;
    use photonic_dynamic::{config, BoxedBoundAttrDecl, BoxedFreeAttrDecl};

    use crate::easing::Easings;

    use super::*;

    #[derive(Deserialize, Debug)]
    pub struct Config<V>
    where V: AttrValue
    {
        pub input: config::Attr<V>,
        pub easing_function: Easings,
        pub easing_duration: Duration,
    }

    impl<V> Producible for Fader<BoxedFreeAttrDecl<V>>
    where V: AttrValue + DeserializeOwned
    {
        type Config = Config<V>;
    }

    impl<V> Producible for Fader<BoxedBoundAttrDecl<V>>
    where V: AttrValue + DeserializeOwned + Bounded
    {
        type Config = Config<V>;
    }

    #[allow(dead_code)]
    pub fn free_attr<V, B>(config: Config<V>, builder: &mut B) -> Result<Fader<BoxedFreeAttrDecl<V>>>
    where
        B: photonic_dynamic::AttrBuilder,
        V: AttrValue + input::Coerced + DeserializeOwned,
    {
        return Ok(Fader {
            input: builder.free_attr("input", config.input)?,
            easing: config.easing_function.with_speed(config.easing_duration),
        });
    }

    #[allow(dead_code)]
    pub fn bound_attr<V, B>(config: Config<V>, builder: &mut B) -> Result<Fader<BoxedBoundAttrDecl<V>>>
    where
        B: photonic_dynamic::AttrBuilder,
        V: AttrValue + input::Coerced + DeserializeOwned + Bounded,
    {
        return Ok(Fader {
            input: builder.bound_attr("input", config.input)?,
            easing: config.easing_function.with_speed(config.easing_duration),
        });
    }
}

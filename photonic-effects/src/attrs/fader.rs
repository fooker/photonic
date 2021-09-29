use std::time::Duration;

use anyhow::Error;

use photonic_core::animation::{Animation, Easing, Transition};
use photonic_core::attr::{
    Attr, AttrValue, BoundAttrDecl, Bounded, Bounds, UnboundAttrDecl, Update,
};
use photonic_core::math::Lerp;
use photonic_core::scene::AttrBuilder;

pub struct Fader<Input, V>
    where
        V: AttrValue + Lerp,
        Input: Attr<Element=V>,
{
    input: Input,

    current: V,

    easing: Easing<f64>,
    transition: Animation<V>,
}

impl<Input, V> Attr for Fader<Input, V>
    where
        V: AttrValue + Lerp,
        Input: Attr<Element=V>,
{
    type Element = V;

    const KIND: &'static str = "fader";

    fn get(&self) -> V {
        self.current
    }

    fn update(&mut self, duration: Duration) -> Update<V> {
        if let Update::Changed(next) = self.input.update(duration) {
            self.transition.start(self.easing, self.current, next);
        }

        if let Transition::Running(value) = self.transition.update(duration) {
            self.current = value;
            return Update::Changed(self.current);
        } else {
            return Update::Idle(self.current);
        }
    }
}

pub struct FaderDecl<Input> {
    pub input: Input,
    pub easing: Easing<f64>,
}

impl<Input, V> BoundAttrDecl for FaderDecl<Input>
    where
        V: AttrValue + Lerp + Bounded,
        Input: BoundAttrDecl<Element=V>,
{
    type Element = V;
    type Target = Fader<Input::Target, V>;
    fn materialize(
        self,
        bounds: Bounds<V>,
        builder: &mut AttrBuilder,
    ) -> Result<Self::Target, Error> {
        let input = builder.bound_attr("input", self.input, bounds)?;

        let current = input.get();

        return Ok(Fader {
            input,
            current,
            easing: self.easing,
            transition: Animation::idle(),
        });
    }
}

impl<Input, V: AttrValue> UnboundAttrDecl for FaderDecl<Input>
    where
        V: Lerp,
        Input: UnboundAttrDecl<Element=V>,
{
    type Element = V;
    type Target = Fader<Input::Target, V>;
    fn materialize(self, builder: &mut AttrBuilder) -> Result<Self::Target, Error> {
        let input = builder.unbound_attr("input", self.input)?;

        let current = input.get();

        return Ok(Fader {
            input,
            current,
            easing: self.easing,
            transition: Animation::idle(),
        });
    }
}

#[cfg(feature = "dyn")]
pub mod model {
    use std::time::Duration;

    use anyhow::{format_err, Result};
    use serde::Deserialize;

    use photonic_core::animation;
    use photonic_core::attr::Bounded;
    use photonic_core::boxed::{BoxedBoundAttrDecl, BoxedUnboundAttrDecl, Wrap};
    use photonic_core::math::Lerp;
    use photonic_dyn::builder::AttrBuilder;
    use photonic_dyn::config;
    use photonic_dyn::model::{AttrValueFactory, BoundAttrModel, UnboundAttrModel};

    #[derive(Deserialize)]
    pub struct EasingModel {
        pub func: String,

        #[serde(with = "humantime_serde")]
        pub speed: Duration,
    }

    impl EasingModel {
        fn func<F: animation::Float>(func: &str) -> Option<fn(F) -> F> {
            return match func {
                "linear" => Some(animation::linear),
                "quad_in" => Some(animation::quad_in),
                "quad_out" => Some(animation::quad_out),
                "quad_inout" => Some(animation::quad_inout),
                "cubic_in" => Some(animation::cubic_in),
                "cubic_out" => Some(animation::cubic_out),
                "cubic_inout" => Some(animation::cubic_inout),
                "quart_in" => Some(animation::quart_in),
                "quart_out" => Some(animation::quart_out),
                "quart_inout" => Some(animation::quart_inout),
                "quint_in" => Some(animation::quint_in),
                "quint_out" => Some(animation::quint_out),
                "quint_inout" => Some(animation::quint_inout),
                "sine_in" => Some(animation::sine_in),
                "sine_out" => Some(animation::sine_out),
                "sine_inout" => Some(animation::sine_inout),
                "circ_in" => Some(animation::circ_in),
                "circ_out" => Some(animation::circ_out),
                "circ_inout" => Some(animation::circ_inout),
                "expo_in" => Some(animation::expo_in),
                "expo_out" => Some(animation::expo_out),
                "expo_inout" => Some(animation::expo_inout),
                "elastic_in" => Some(animation::elastic_in),
                "elastic_out" => Some(animation::elastic_out),
                "elastic_inout" => Some(animation::elastic_inout),
                "back_in" => Some(animation::back_in),
                "back_out" => Some(animation::back_out),
                "back_inout" => Some(animation::back_inout),
                "bounce_in" => Some(animation::bounce_in),
                "bounce_out" => Some(animation::bounce_out),
                "bounce_inout" => Some(animation::bounce_inout),
                _ => None,
            };
        }

        pub fn resemble<F>(self) -> Result<animation::Easing<F>>
            where
                F: animation::Float,
        {
            return Ok(animation::Easing {
                func: Self::func(&self.func)
                    .ok_or_else(|| format_err!("Unknown easing function: {}", self.func))?,
                speed: self.speed,
            });
        }
    }

    #[derive(Deserialize)]
    pub struct FaderModel {
        pub input: config::Attr,
        pub easing: EasingModel,
    }

    impl<V> UnboundAttrModel<V> for FaderModel
        where
            V: AttrValueFactory,
    {
        default fn assemble(self, _builder: &mut impl AttrBuilder) -> Result<BoxedUnboundAttrDecl<V>> {
            return Err(format_err!(
            "Fader is not supported for Attributes of Type {}",
            std::any::type_name::<V>()
        ));
        }
    }

    impl<V> UnboundAttrModel<V> for FaderModel
        where
            V: AttrValueFactory + Lerp,
    {
        fn assemble(self, builder: &mut impl AttrBuilder) -> Result<BoxedUnboundAttrDecl<V>> {
            return Ok(BoxedUnboundAttrDecl::wrap(
                super::FaderDecl {
                    input: builder.unbound_attr("input", self.input)?,
                    easing: self.easing.resemble()?,
                },
            ));
        }
    }

    impl<V> BoundAttrModel<V> for FaderModel
        where
            V: AttrValueFactory + Bounded,
    {
        default fn assemble(self, _builder: &mut impl AttrBuilder) -> Result<BoxedBoundAttrDecl<V>> {
            return Err(format_err!(
            "Fader is not supported for Attributes of Type {}",
            std::any::type_name::<V>()
        ));
        }
    }

    impl<V> BoundAttrModel<V> for FaderModel
        where
            V: AttrValueFactory + Bounded + Lerp,
    {
        fn assemble(self, builder: &mut impl AttrBuilder) -> Result<BoxedBoundAttrDecl<V>> {
            return Ok(BoxedBoundAttrDecl::wrap(
                super::FaderDecl {
                    input: builder.bound_attr("input", self.input)?,
                    easing: self.easing.resemble()?,
                },
            ));
        }
    }
}

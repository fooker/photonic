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
    Input: Attr<V>,
{
    input: Input,

    current: V,

    easing: Easing<f64>,
    transition: Animation<V>,
}

impl<Input, V> Attr<V> for Fader<Input, V>
where
    V: AttrValue + Lerp,
    Input: Attr<V>,
{
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

impl<Input, V> BoundAttrDecl<V> for FaderDecl<Input>
where
    V: AttrValue + Lerp + Bounded,
    Input: BoundAttrDecl<V>,
{
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

impl<Input, V: AttrValue> UnboundAttrDecl<V> for FaderDecl<Input>
where
    V: Lerp,
    Input: UnboundAttrDecl<V>,
{
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

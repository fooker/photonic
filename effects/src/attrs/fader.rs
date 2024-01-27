use std::time::Duration;

use anyhow::Result;

use photonic::{Attr, AttrBuilder, AttrValue, BoundAttrDecl, FreeAttrDecl};
use photonic::attr::{Bounded, Bounds};
use photonic::math::Lerp;

use crate::easing::Easing;

pub struct FaderAttr<V, Input>
    where V: AttrValue + PartialEq + Lerp,
          Input: Attr<Value=V>,
{
    input: Input,

    last: V,
    next: V,

    fade: f32,

    easing: Easing<f32>,
}

impl<V, Input> Attr for FaderAttr<V, Input>
    where V: AttrValue + PartialEq + Lerp,
          Input: Attr<Value=V>,
{
    type Value = V;

    const KIND: &'static str = "fader";

    fn update(&mut self, duration: Duration) -> Self::Value {
        let next = self.input.update(duration);
        if next != self.next {
            // Calculate current value and use as previous to accommodate for
            // value changes while a transition is still in progress
            self.last = Lerp::lerp(self.last, self.next, (self.easing.func)(self.fade));
            self.next = next;
            self.fade = 0.0;
        }

        if self.fade < 1.0 {
            self.fade += duration.as_secs_f32() / self.easing.speed.as_secs_f32();
            self.fade = self.fade.min(1.0);
        }

        return Lerp::lerp(self.last, self.next, (self.easing.func)(self.fade));
    }
}

pub struct Fader<Input> {
    pub input: Input,
    pub easing: Easing<f32>,
}

impl<Input, V> BoundAttrDecl for Fader<Input>
    where V: AttrValue + PartialEq + Lerp + Bounded,
          Input: BoundAttrDecl<Value=V>,
{
    type Value = V;
    type Attr = FaderAttr<V, Input::Attr>;

    fn materialize(self, bounds: Bounds<V>, builder: &mut AttrBuilder) -> Result<Self::Attr> {
        let mut input = builder.bound_attr("input", self.input, bounds)?;

        let current = input.update(Duration::ZERO);

        return Ok(FaderAttr {
            input,
            last: current,
            next: current,
            fade: 1.0,
            easing: self.easing,
        });
    }
}

impl<Input, V> FreeAttrDecl for Fader<Input>
    where
        V: AttrValue + PartialEq + Lerp,
        Input: FreeAttrDecl<Value=V>,
{
    type Value = V;
    type Attr = FaderAttr<V, Input::Attr>;

    fn materialize(self, builder: &mut AttrBuilder) -> Result<Self::Attr> {
        let mut input = builder.unbound_attr("input", self.input)?;

        let current = input.update(Duration::ZERO);

        return Ok(FaderAttr {
            input,
            last: current,
            next: current,
            fade: 1.0,
            easing: self.easing,
        });
    }
}

use std::time::Duration;
use photonic::{Attr, AttrValue};
use photonic::math::Lerp;
use crate::easing::Easing;

pub struct Fader<V, Input>
    where V: AttrValue + Lerp,
          Input: Attr<Value = V>,
{
    input: Input,

    current: V,

    easing: Easing<f64>,
    //transition: Animation<V>,
}

impl<V, Input> Attr for Fader<V, Input>
    where V: AttrValue + Lerp,
          Input: Attr<Value = V>,
{
    type Value = V;

    const KIND: &'static str = "fader";

    fn update(&mut self, duration: Duration) -> Self::Value {
        todo!()
    }
}
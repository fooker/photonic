use std::time::Duration;

use anyhow::Error;
use num::traits::Num;

use photonic_core::attr::{Attr, AttrValue, BoundAttrDecl, Bounded, Bounds, Update};
use photonic_core::input::{Input, Poll};
use photonic_core::scene::{AttrBuilder, InputHandle};

pub struct Looper<V>
where
    V: AttrValue + Num,
{
    min: V,
    max: V,
    step: V,

    current: V,

    trigger: Input<()>,
}

impl<V> Attr<V> for Looper<V>
where
    V: AttrValue + Num,
{
    const KIND: &'static str = "looper";

    fn get(&self) -> V {
        return self.current;
    }

    fn update(&mut self, _duration: Duration) -> Update<V> {
        if let Poll::Ready(()) = self.trigger.poll() {
            self.current = self.min + (self.current + self.step - self.min) % (self.max - self.min);
            return Update::Changed(self.current);
        } else {
            return Update::Idle(self.current);
        }
    }
}

pub struct LooperDecl<V>
where
    V: AttrValue,
{
    pub step: V,
    pub trigger: InputHandle<()>,
}

impl<V> BoundAttrDecl<V> for LooperDecl<V>
where
    V: AttrValue + Bounded + Num + PartialOrd,
{
    type Target = Looper<V>;
    fn materialize(
        self,
        bounds: Bounds<V>,
        builder: &mut AttrBuilder,
    ) -> Result<Self::Target, Error> {
        let (min, max) = bounds.into();

        let step = if self.step >= V::zero() {
            self.step
        } else {
            (self.step % (max - min + V::one())) + (max - min + V::one())
        };

        let max = max + V::one();

        return Ok(Looper {
            min,
            max,
            step,
            current: min,
            trigger: builder.input("trigger", self.trigger)?,
        });
    }
}

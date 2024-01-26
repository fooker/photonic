use std::time::Duration;

use anyhow::Error;
use num_traits::Num;
use photonic::{Attr, AttrBuilder, AttrValue, BoundAttrDecl};
use photonic::attr::{Bounded, Bounds};
use photonic::input::{Input, Poll};
use photonic::scene::InputHandle;


pub struct LooperAttr<V>
    where V: AttrValue + Num,
{
    min: V,
    max: V,
    step: V,

    current: V,

    trigger: Input<()>,
}

impl<V> Attr for LooperAttr<V>
    where V: AttrValue + Num,
{
    type Value = V;
    const KIND: &'static str = "looper";

    fn update(&mut self, _duration: Duration) -> V {
        if let Poll::Update(()) = self.trigger.poll() {
            self.current = self.min + (self.current + self.step - self.min) % (self.max - self.min);
        }

        return self.current;
    }
}

pub struct Looper<V>
    where V: AttrValue,
{
    pub step: V,
    pub trigger: InputHandle<()>,
}

impl<V> BoundAttrDecl for Looper<V>
    where V: AttrValue + Bounded + Num + PartialOrd,
{
    type Value = V;
    type Attr = LooperAttr<V>;
    fn materialize(self, bounds: Bounds<V>, builder: &mut AttrBuilder) -> Result<Self::Attr, Error> {

        let step = if self.step >= V::zero() {
            self.step
        } else {
            (self.step % (bounds.max - bounds.min + V::one())) + (bounds.max - bounds.min + V::one())
        };

        let min = bounds.min;
        let max = bounds.max + V::one();

        return Ok(LooperAttr {
            min,
            max,
            step,
            current: bounds.min,
            trigger: builder.input("trigger", self.trigger)?,
        });
    }
}

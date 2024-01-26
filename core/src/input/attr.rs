use std::time::Duration;
use anyhow::Result;

use crate::{Attr, AttrBuilder, AttrValue};
use crate::attr::{BoundAttrDecl, Bounded, Bounds, FreeAttrDecl};
use crate::input::{Input, InputValue, Poll};
use crate::scene::InputHandle;

pub struct InputAttrDecl<V>
    where V: InputValue + AttrValue,
{
    input: InputHandle<V>,
    initial: V,
}

impl<V> BoundAttrDecl for InputAttrDecl<V>
    where
        V: AttrValue + InputValue + Bounded,
{
    type Value = V;
    type Target = BoundInputAttr<V>;

    fn materialize(self, bounds: Bounds<V>, builder: &mut AttrBuilder) -> Result<Self::Target> {
        let input = builder.input("input", self.input)?;

        let initial = bounds.ensure(self.initial)?;

        return Ok(Self::Target {
            bounds,
            input,
            current: initial,
        });
    }
}

impl<V> FreeAttrDecl for InputAttrDecl<V>
    where V: AttrValue + InputValue,
{
    type Value = V;
    type Target = UnboundInputAttr<V>;

    fn materialize(self, builder: &mut AttrBuilder) -> Result<Self::Target> {
        let input = builder.input("value", self.input)?;

        return Ok(Self::Target {
            input,
            current: self.initial,
        });
    }
}

pub struct BoundInputAttr<V>
    where V: AttrValue + InputValue + Bounded,
{
    bounds: Bounds<V>,

    input: Input<V>,
    current: V,
}

impl<V> Attr for BoundInputAttr<V>
    where V: AttrValue + InputValue + Bounded,
{
    type Value = V;
    const KIND: &'static str = "input";

    fn update(&mut self, _duration: Duration) -> Self::Value {
        if let Poll::Update(update) = self.input.poll() {
            if let Ok(update) = self.bounds.ensure(update) {
                self.current = update;
            }
        }

        return self.current;
    }
}

pub struct UnboundInputAttr<V>
    where V: AttrValue + InputValue,
{
    input: Input<V>,
    current: V,
}

impl<V> Attr for UnboundInputAttr<V>
    where V: AttrValue + InputValue,
{
    type Value = V;

    const KIND: &'static str = "input";

    fn update(&mut self, _duration: Duration) -> Self::Value {
        if let Poll::Update(update) = self.input.poll() {
            self.current = update;
        }

        return self.current;
    }
}

impl<V> InputHandle<V>
    where V: InputValue + AttrValue,
{
    pub fn attr(self, initial: V) -> InputAttrDecl<V> {
        return InputAttrDecl {
            input: self,
            initial,
        };
    }
}

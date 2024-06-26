use std::time::Duration;

use anyhow::Result;

use crate::attr::{Attr, AttrValue, Bounded, Bounds};
use crate::decl::{BoundAttrDecl, FreeAttrDecl};
use crate::input::{Input, InputValue, Poll};
use crate::scene::{AttrBuilder, InputHandle};

#[derive(Debug)]
pub struct InputAttrDecl<I, A>
where
    I: InputValue,
    A: AttrValue + TryFrom<I>,
{
    input: InputHandle<I>,
    initial: A,
}

impl<I, A> BoundAttrDecl for InputAttrDecl<I, A>
where
    I: InputValue,
    A: AttrValue + Bounded + TryFrom<I>,
{
    type Value = A;
    type Attr = BoundInputAttr<I, A>;

    fn materialize(self, bounds: Bounds<A>, builder: &mut AttrBuilder) -> Result<Self::Attr> {
        let input = builder.input("input", self.input)?;

        let initial = bounds.ensure(self.initial)?;

        return Ok(Self::Attr {
            bounds,
            input,
            current: initial,
        });
    }
}

impl<I, A> FreeAttrDecl for InputAttrDecl<I, A>
where
    I: InputValue,
    A: AttrValue + TryFrom<I>,
{
    type Value = A;
    type Attr = FreeInputAttr<I, A>;

    fn materialize(self, builder: &mut AttrBuilder) -> Result<Self::Attr> {
        let input = builder.input("value", self.input)?;

        return Ok(Self::Attr {
            input,
            current: self.initial,
        });
    }
}

#[derive(Debug)]
pub struct BoundInputAttr<I, A>
where
    I: InputValue,
    A: AttrValue + Bounded,
{
    input: Input<I>,
    current: A,

    bounds: Bounds<A>,
}

impl<I, A> Attr for BoundInputAttr<I, A>
where
    I: InputValue,
    A: AttrValue + TryFrom<I> + Bounded,
{
    type Value = A;
    const KIND: &'static str = "input";

    fn update(&mut self, _duration: Duration) -> Self::Value {
        if let Poll::Update(update) = self.input.poll() {
            // TODO: This needs error handling - best idea for now is to couple inputs and attrs more tightly to allow
            // InputAttrs to report errors to the input they are feeding from by moving the atomic value latch to the
            // InputAttr and have all direct users of Inputs converted to Attrs (check if possible first)
            if let Ok(update) = update.try_into() {
                if let Ok(update) = self.bounds.ensure(update) {
                    self.current = update;
                }
            }
        }

        return self.current;
    }
}

#[derive(Debug)]
pub struct FreeInputAttr<I, A>
where
    I: InputValue,
    A: AttrValue,
{
    input: Input<I>,
    current: A,
}

impl<I, A> Attr for FreeInputAttr<I, A>
where
    I: InputValue,
    A: AttrValue + TryFrom<I>,
{
    type Value = A;

    const KIND: &'static str = "input";

    fn update(&mut self, _duration: Duration) -> Self::Value {
        if let Poll::Update(update) = self.input.poll() {
            if let Ok(update) = update.try_into() {
                self.current = update;
            }
        }

        return self.current;
    }
}

impl<V> InputHandle<V>
where V: InputValue
{
    pub fn attr<A>(self, initial: A) -> InputAttrDecl<V, A>
    where A: AttrValue + TryFrom<V> {
        return InputAttrDecl {
            input: self,
            initial,
        };
    }
}

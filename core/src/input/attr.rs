use anyhow::Result;

use crate::attr::{Attr, AttrValue, Bounded, Bounds};
use crate::decl::{BoundAttrDecl, FreeAttrDecl};
use crate::input::{Input, InputValue, Poll};
use crate::scene::{AttrBuilder, InputHandle};
use crate::{input, scene};

#[derive(Debug)]
pub struct InputAttrDecl<A>
where A: AttrValue + input::Coerced
{
    input: InputHandle<A::Input>,
    initial: A,
}

impl<A> BoundAttrDecl for InputAttrDecl<A>
where A: AttrValue + input::Coerced + Bounded
{
    type Value = A;
    type Attr = BoundInputAttr<A>;

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

impl<A> FreeAttrDecl for InputAttrDecl<A>
where A: AttrValue + input::Coerced
{
    type Value = A;
    type Attr = FreeInputAttr<A>;

    fn materialize(self, builder: &mut AttrBuilder) -> Result<Self::Attr> {
        let input = builder.input("value", self.input)?;

        return Ok(Self::Attr {
            input,
            current: self.initial,
        });
    }
}

#[derive(Debug)]
pub struct BoundInputAttr<A>
where A: AttrValue + input::Coerced + Bounded
{
    input: Input<A::Input>,
    current: A,

    bounds: Bounds<A>,
}

impl<A> Attr for BoundInputAttr<A>
where A: AttrValue + input::Coerced + Bounded
{
    type Value = A;
    const KIND: &'static str = "input";

    fn update(&mut self, _ctx: &scene::RenderContext) -> Self::Value {
        if let Poll::Update(update) = self.input.poll() {
            // TODO: This needs error handling - best idea for now is to couple inputs and attrs more tightly to allow
            // InputAttrs to report errors to the input they are feeding from by moving the atomic value latch to the
            // InputAttr and have all direct users of Inputs converted to Attrs (check if possible first)
            if let Ok(update) = A::try_from_input(update) {
                if let Ok(update) = self.bounds.ensure(update) {
                    self.current = update;
                }
            }
        }

        return self.current;
    }
}

#[derive(Debug)]
pub struct FreeInputAttr<A>
where A: AttrValue + input::Coerced
{
    input: Input<A::Input>,
    current: A,
}

impl<A> Attr for FreeInputAttr<A>
where A: AttrValue + input::Coerced
{
    type Value = A;

    const KIND: &'static str = "input";

    fn update(&mut self, _ctx: &scene::RenderContext) -> Self::Value {
        if let Poll::Update(update) = self.input.poll() {
            if let Ok(update) = A::try_from_input(update) {
                self.current = update;
            }
        }

        return self.current;
    }
}

impl<I> InputHandle<I>
where I: InputValue
{
    pub fn attr<A>(self, initial: A) -> InputAttrDecl<A>
    where A: AttrValue + input::Coerced<Input = I> {
        return InputAttrDecl {
            input: self,
            initial,
        };
    }
}

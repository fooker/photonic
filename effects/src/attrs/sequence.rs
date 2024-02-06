use std::time::Duration;

use anyhow::Error;

use photonic::{Attr, AttrBuilder, AttrValue, BoundAttrDecl, FreeAttrDecl};
use photonic::attr::{Bounded, Bounds};
use photonic::input::{Input, Poll};
use photonic::scene::InputHandle;
use photonic_dyn::DynamicAttr;

pub struct SequenceAttr<V>
    where V: AttrValue,
{
    values: Vec<V>,

    position: usize,

    next: Option<Input<()>>,
    prev: Option<Input<()>>,
}

impl<V> Attr for SequenceAttr<V>
    where
        V: AttrValue,
{
    type Value = V;
    const KIND: &'static str = "sequence";

    fn update(&mut self, _duration: Duration) -> V {
        let next = self.next.as_mut().map_or(Poll::Pending, Input::poll);
        let prev = self.prev.as_mut().map_or(Poll::Pending, Input::poll);

        return match (next, prev) {
            (Poll::Update(()), Poll::Update(())) | (Poll::Pending, Poll::Pending) => {
                self.values[self.position]
            }
            (Poll::Update(()), Poll::Pending) => {
                self.position = (self.position + self.values.len() + 1) % self.values.len();
                self.values[self.position]
            }
            (Poll::Pending, Poll::Update(())) => {
                self.position = (self.position + self.values.len() - 1) % self.values.len();
                self.values[self.position]
            }
        };
    }
}

pub struct Sequence<V>
    where V: AttrValue,
{
    pub values: Vec<V>,

    pub next: Option<InputHandle<()>>,
    pub prev: Option<InputHandle<()>>,
}

impl<V> BoundAttrDecl for Sequence<V>
    where V: AttrValue + Bounded,
{
    type Value = V;
    type Attr = SequenceAttr<V>;

    fn materialize(self, bounds: Bounds<V>, builder: &mut AttrBuilder) -> Result<Self::Attr, Error> {
        let values =
            self.values.into_iter().map(|v| bounds.ensure(v)).collect::<Result<Vec<_>, Error>>()?;

        let next = self.next.map(|input| builder.input("next", input)).transpose()?;
        let prev = self.prev.map(|input| builder.input("prev", input)).transpose()?;

        return Ok(SequenceAttr {
            values,
            position: 0,
            next,
            prev,
        });
    }
}

impl<V> FreeAttrDecl for Sequence<V>
    where V: AttrValue,
{
    type Value = V;
    type Attr = SequenceAttr<V>;

    fn materialize(self, builder: &mut AttrBuilder) -> Result<Self::Attr, Error> {
        let next = self.next.map(|input| builder.input("next", input)).transpose()?;
        let prev = self.prev.map(|input| builder.input("prev", input)).transpose()?;

        return Ok(SequenceAttr {
            values: self.values,
            position: 0,
            next,
            prev,
        });
    }
}

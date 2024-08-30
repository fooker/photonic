use anyhow::Result;

use photonic::attr::{Bounded, Bounds};
use photonic::input::{Input, Poll};
use photonic::scene::InputHandle;
use photonic::{scene, Attr, AttrBuilder, AttrValue, BoundAttrDecl, FreeAttrDecl};

pub struct SequenceAttr<V>
where V: AttrValue
{
    values: Vec<V>,

    position: usize,

    next: Option<Input<()>>,
    prev: Option<Input<()>>,
}

impl<V> Attr for SequenceAttr<V>
where V: AttrValue
{
    type Value = V;
    const KIND: &'static str = "sequence";

    fn update(&mut self, _ctx: &scene::RenderContext) -> Self::Value {
        let next = self.next.as_mut().map_or(Poll::Pending, Input::poll);
        let prev = self.prev.as_mut().map_or(Poll::Pending, Input::poll);

        return match (next, prev) {
            (Poll::Update(()), Poll::Update(())) | (Poll::Pending, Poll::Pending) => self.values[self.position],
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
where V: AttrValue
{
    pub values: Vec<V>,

    pub next: Option<InputHandle<()>>,

    pub prev: Option<InputHandle<()>>,
}

impl<V> BoundAttrDecl for Sequence<V>
where V: AttrValue + Bounded
{
    type Value = V;
    type Attr = SequenceAttr<V>;

    fn materialize(self, bounds: Bounds<V>, builder: &mut AttrBuilder) -> Result<Self::Attr> {
        let values = self.values.into_iter().map(|v| bounds.ensure(v)).collect::<Result<Vec<_>>>()?;

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
where V: AttrValue
{
    type Value = V;
    type Attr = SequenceAttr<V>;

    fn materialize(self, builder: &mut AttrBuilder) -> Result<Self::Attr> {
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

#[cfg(feature = "dynamic")]
pub mod dynamic {
    use serde::de::DeserializeOwned;
    use serde::Deserialize;

    use photonic_dynamic::config;
    use photonic_dynamic::factory::Producible;

    use super::*;

    #[derive(Deserialize, Debug)]
    pub struct Config<V> {
        pub values: Vec<V>,
        pub next: Option<config::Input>,
        pub prev: Option<config::Input>,
    }

    impl<V> Producible for Sequence<V>
    where V: AttrValue + DeserializeOwned
    {
        type Config = Config<V>;
    }

    pub fn free_attr<V, B>(config: Config<V>, builder: &mut B) -> Result<Sequence<V>>
    where
        B: photonic_dynamic::AttrBuilder,
        V: AttrValue + DeserializeOwned,
    {
        return Ok(Sequence {
            values: config.values,
            next: config.next.map(|config| builder.input(config)).transpose()?,
            prev: config.prev.map(|config| builder.input(config)).transpose()?,
        });
    }

    pub fn bound_attr<V, B>(config: Config<V>, builder: &mut B) -> Result<Sequence<V>>
    where
        B: photonic_dynamic::AttrBuilder,
        V: AttrValue + DeserializeOwned + Bounded,
    {
        return Ok(Sequence {
            values: config.values,
            next: config.next.map(|config| builder.input(config)).transpose()?,
            prev: config.prev.map(|config| builder.input(config)).transpose()?,
        });
    }
}

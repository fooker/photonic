use std::time::Duration;

use anyhow::Error;

use photonic_core::attr::{
    Attr, AttrValue, BoundAttrDecl, Bounded, Bounds, UnboundAttrDecl, Update,
};
use photonic_core::input::{Input, Poll};
use photonic_core::scene::{AttrBuilder, InputHandle};

pub struct Sequence<V>
where
    V: AttrValue,
{
    values: Vec<V>,

    position: usize,

    next: Option<Input<()>>,
    prev: Option<Input<()>>,
}

impl<V> Attr for Sequence<V>
where
    V: AttrValue,
{
    type Value = V;
    const KIND: &'static str = "sequence";

    fn get(&self) -> V {
        self.values[self.position]
    }

    fn update(&mut self, _duration: Duration) -> Update<V> {
        let next = self.next.as_mut().map_or(Poll::Pending, Input::poll);
        let prev = self.prev.as_mut().map_or(Poll::Pending, Input::poll);

        return match (next, prev) {
            (Poll::Ready(()), Poll::Ready(())) | (Poll::Pending, Poll::Pending) => {
                Update::Idle(self.values[self.position])
            }
            (Poll::Ready(()), Poll::Pending) => {
                self.position = (self.position + self.values.len() + 1) % self.values.len();
                Update::Changed(self.values[self.position])
            }
            (Poll::Pending, Poll::Ready(())) => {
                self.position = (self.position + self.values.len() - 1) % self.values.len();
                Update::Changed(self.values[self.position])
            }
        };
    }
}

pub struct SequenceDecl<V>
where
    V: AttrValue,
{
    pub values: Vec<V>,
    pub next: Option<InputHandle<()>>,
    pub prev: Option<InputHandle<()>>,
}

impl<V> BoundAttrDecl for SequenceDecl<V>
where
    V: AttrValue + Bounded,
{
    type Value = V;
    type Target = Sequence<V>;
    fn materialize(
        self,
        bounds: Bounds<V>,
        builder: &mut AttrBuilder,
    ) -> Result<Self::Target, Error> {
        let values =
            self.values.into_iter().map(|v| bounds.ensure(v)).collect::<Result<Vec<_>, Error>>()?;

        let next = self.next.map(|input| builder.input("next", input)).transpose()?;
        let prev = self.prev.map(|input| builder.input("prev", input)).transpose()?;

        return Ok(Sequence {
            values,
            position: 0,
            next,
            prev,
        });
    }
}

impl<V> UnboundAttrDecl for SequenceDecl<V>
where
    V: AttrValue,
{
    type Value = V;
    type Target = Sequence<V>;
    fn materialize(self, builder: &mut AttrBuilder) -> Result<Self::Target, Error> {
        let next = self.next.map(|input| builder.input("next", input)).transpose()?;
        let prev = self.prev.map(|input| builder.input("prev", input)).transpose()?;

        return Ok(Sequence {
            values: self.values,
            position: 0,
            next,
            prev,
        });
    }
}

#[cfg(feature = "dyn")]
pub mod model {

    use anyhow::Result;
    use serde::Deserialize;

    use photonic_core::attr::Bounded;
    use photonic_core::boxed::{BoxedBoundAttrDecl, BoxedUnboundAttrDecl, Wrap};
    use photonic_dyn::builder::AttrBuilder;
    use photonic_dyn::config;
    use photonic_dyn::model::{AttrValueFactory, BoundAttrModel, UnboundAttrModel};

    #[derive(Deserialize)]
    pub struct SequenceModel<V>
    where
        V: AttrValueFactory,
    {
        pub values: Vec<V::Model>,
        pub next: Option<config::Input>,
        pub prev: Option<config::Input>,
    }

    impl<V> UnboundAttrModel<V> for SequenceModel<V>
    where
        V: AttrValueFactory,
    {
        fn assemble(self, builder: &mut impl AttrBuilder) -> Result<BoxedUnboundAttrDecl<V>> {
            return Ok(BoxedUnboundAttrDecl::wrap(super::SequenceDecl {
                values: self.values.into_iter().map(V::assemble).collect::<Result<Vec<_>>>()?,
                next: self.next.map(|i| builder.input(i)).transpose()?,
                prev: self.prev.map(|i| builder.input(i)).transpose()?,
            }));
        }
    }

    impl<V> BoundAttrModel<V> for SequenceModel<V>
    where
        V: AttrValueFactory + Bounded,
    {
        fn assemble(self, builder: &mut impl AttrBuilder) -> Result<BoxedBoundAttrDecl<V>> {
            return Ok(BoxedBoundAttrDecl::wrap(super::SequenceDecl {
                values: self.values.into_iter().map(V::assemble).collect::<Result<Vec<_>>>()?,
                next: self.next.map(|i| builder.input(i)).transpose()?,
                prev: self.prev.map(|i| builder.input(i)).transpose()?,
            }));
        }
    }
}

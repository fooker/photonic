use std::time::Duration;

use failure::Error;

use photonic_core::input::{Input, InputHandle, Poll};
use photonic_core::attr::{AttrValue, Attr, Update, BoundAttrDecl, Bounded, Bounds, UnboundAttrDecl};
use photonic_core::scene::AttrBuilder;

pub struct Sequence<V>
    where V: AttrValue {
    values: Vec<V>,

    position: usize,

    trigger: Input<()>,
}

impl<V> Attr<V> for Sequence<V>
    where V: AttrValue {
    const KIND: &'static str = "sequence";

    fn get(&self) -> V {
        self.values[self.position]
    }

    fn update(&mut self, _duration: &Duration) -> Update<V> {
        if let Poll::Ready(()) = self.trigger.poll() {
            self.position = (self.position + 1) % self.values.len();
            return Update::Changed(self.values[self.position]);
        } else {
            return Update::Idle;
        }
    }
}

pub struct SequenceDecl<V>
    where V: AttrValue {
    pub values: Vec<V>,
    pub trigger: InputHandle<()>,
}

impl<V> BoundAttrDecl<V> for SequenceDecl<V>
    where V: AttrValue + Bounded {
    type Target = Sequence<V>;
    fn materialize(self, bounds: Bounds<V>, builder: &mut AttrBuilder) -> Result<Self::Target, Error> {
        let values = self.values.into_iter()
            .map(|v| bounds.ensure(v))
            .collect::<Result<Vec<_>, Error>>()?;

        let trigger = builder.input("trigger", self.trigger)?;

        return Ok(Sequence {
            values,
            position: 0,
            trigger,
        });
    }
}

impl<V> UnboundAttrDecl<V> for SequenceDecl<V>
    where V: AttrValue {
    type Target = Sequence<V>;
    fn materialize(self, builder: &mut AttrBuilder) -> Result<Self::Target, Error> {
        let values = self.values.into_iter()
            .map(|v| v.into())
            .collect();

        let trigger = builder.input("trigger", self.trigger)?;

        return Ok(Sequence {
            values,
            position: 0,
            trigger,
        });
    }
}

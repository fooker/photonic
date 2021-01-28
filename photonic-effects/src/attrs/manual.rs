use std::time::Duration;

use failure::Error;

use photonic_core::input::{Input, Poll, InputValue};
use photonic_core::attr::*;
use photonic_core::core::{NodeBuilder, AttrBuilder};

pub struct BoundManual<V>
    where V: AttrValue + InputValue + Bounded {
    bounds: Bounds<V>,

    value: Input<V>,
    current: V,
}

impl<V> Attr<V> for BoundManual<V>
    where V: AttrValue + InputValue + Bounded {
    const KIND: &'static str = "manual";

    fn get(&self) -> V {
        self.current
    }

    fn update(&mut self, _duration: &Duration) -> Update<V> {
        if let Poll::Ready(update) = self.value.poll() {
            if let Ok(update) = self.bounds.ensure(update) {
                self.current = update;
                return Update::Changed(self.current);
            } else {
                return Update::Idle;
            }
        } else {
            return Update::Idle;
        }
    }
}

pub struct UnboundManual<V>
    where V: AttrValue + InputValue {
    value: Input<V>,
    current: V,
}

impl<V> Attr<V> for UnboundManual<V>
    where V: AttrValue + InputValue {
    const KIND: &'static str = "manual";

    fn get(&self) -> V {
        self.current
    }

    fn update(&mut self, _duration: &Duration) -> Update<V> {
        if let Poll::Ready(update) = self.value.poll() {
            self.current = update;
            return Update::Changed(self.current);
        } else {
            return Update::Idle;
        }
    }
}

pub struct ManualDecl<V>
    where V: AttrValue + InputValue {
    pub value: Input<V>,
}

impl<V> From<Input<V>> for ManualDecl<V>
    where V: AttrValue + InputValue {
    fn from(value: Input<V>) -> Self {
        return Self { value };
    }
}

impl<V> BoundAttrDecl<V> for ManualDecl<V>
    where V: AttrValue + InputValue + Bounded {
    type Target = BoundManual<V>;

    fn materialize(self, bounds: Bounds<V>, builder: &mut AttrBuilder) -> Result<Self::Target, Error> {
        let value = builder.input("value", self.value)?;

        let current = bounds.min;

        return Ok(BoundManual {
            bounds,
            value,
            current,
        });
    }
}

impl<V> UnboundAttrDecl<V> for ManualDecl<V>
    where V: AttrValue + InputValue + Default {
    type Target = UnboundManual<V>;

    fn materialize(self, builder: &mut AttrBuilder) -> Result<Self::Target, Error> {
        let value = builder.input("value", self.value)?;

        let current = V::default();

        return Ok(UnboundManual {
            value,
            current,
        });
    }
}

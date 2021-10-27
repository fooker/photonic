use std::time::Duration;

use anyhow::Error;

use photonic_core::attr::{
    Attr, AttrValue, BoundAttrDecl, Bounded, Bounds, UnboundAttrDecl, Update,
};
use photonic_core::input::{Input, Poll};
use photonic_core::scene::{AttrBuilder, InputHandle};

#[derive(Clone, Copy, Debug)]
enum State {
    Released,
    Pressed(Duration),
}

impl State {
    fn update(self, duration: Duration) -> Self {
        if let State::Pressed(remaining) = self {
            if remaining > duration {
                return State::Pressed(remaining - duration);
            } else {
                return State::Released;
            }
        } else {
            return State::Released;
        }
    }

    pub fn pressed(&self) -> bool {
        return match self {
            State::Released => false,
            State::Pressed(_) => true,
        };
    }
}

pub struct Button<V>
where
    V: AttrValue,
{
    value_released: V,
    value_pressed: V,

    hold_time: Duration,

    state: State,

    trigger: Input<()>,
}

impl<V> Attr for Button<V>
where
    V: AttrValue,
{
    type Value = V;
    const KIND: &'static str = "button";

    fn get(&self) -> V {
        return match self.state {
            State::Released => self.value_released,
            State::Pressed(_) => self.value_pressed,
        };
    }

    fn update(&mut self, duration: Duration) -> Update<V> {
        let state_old = self.state.pressed();

        if let Poll::Ready(()) = self.trigger.poll() {
            self.state = State::Pressed(self.hold_time)
        };

        self.state = self.state.update(duration);

        let state_new = self.state.pressed();

        return match (state_old, state_new) {
            (false, true) => Update::Changed(self.value_pressed),
            (true, false) => Update::Changed(self.value_released),

            (false, false) => Update::Idle(self.value_released),
            (true, true) => Update::Idle(self.value_pressed),
        };
    }
}

pub struct ButtonDecl<V>
where
    V: AttrValue,
{
    pub value: (V, V),
    pub hold_time: Duration,
    pub trigger: InputHandle<()>,
}

impl<V> BoundAttrDecl for ButtonDecl<V>
where
    V: AttrValue + Bounded,
{
    type Value = V;
    type Target = Button<V>;
    fn materialize(
        self,
        bounds: Bounds<V>,
        builder: &mut AttrBuilder,
    ) -> Result<Self::Target, Error> {
        return Ok(Button {
            value_released: bounds.ensure(self.value.0)?,
            value_pressed: bounds.ensure(self.value.1)?,
            hold_time: self.hold_time,
            state: State::Released,
            trigger: builder.input("trigger", self.trigger)?,
        });
    }
}

impl<V> UnboundAttrDecl for ButtonDecl<V>
where
    V: AttrValue,
{
    type Value = V;
    type Target = Button<V>;
    fn materialize(self, builder: &mut AttrBuilder) -> Result<Self::Target, Error> {
        return Ok(Button {
            value_released: self.value.0,
            value_pressed: self.value.1,
            hold_time: self.hold_time,
            state: State::Released,
            trigger: builder.input("trigger", self.trigger)?,
        });
    }
}

#[cfg(feature = "dyn")]
pub mod model {
    use std::time::Duration;

    use anyhow::Result;
    use serde::Deserialize;

    use photonic_core::attr::Bounded;
    use photonic_core::boxed::{BoxedBoundAttrDecl, BoxedUnboundAttrDecl, Wrap};
    use photonic_dyn::builder::AttrBuilder;
    use photonic_dyn::config;
    use photonic_dyn::model::{AttrValueFactory, BoundAttrModel, UnboundAttrModel};

    #[derive(Deserialize)]
    pub struct ButtonModel<V>
    where
        V: AttrValueFactory,
    {
        pub value: (V::Model, V::Model),
        pub hold_time: Duration,
        pub trigger: config::Input,
    }

    impl<V> UnboundAttrModel<V> for ButtonModel<V>
    where
        V: AttrValueFactory,
    {
        fn assemble(self, builder: &mut impl AttrBuilder) -> Result<BoxedUnboundAttrDecl<V>> {
            return Ok(BoxedUnboundAttrDecl::wrap(super::ButtonDecl {
                value: (V::assemble(self.value.0)?, V::assemble(self.value.1)?),
                hold_time: self.hold_time,
                trigger: builder.input(self.trigger)?,
            }));
        }
    }

    impl<V> BoundAttrModel<V> for ButtonModel<V>
    where
        V: AttrValueFactory + Bounded,
    {
        fn assemble(self, builder: &mut impl AttrBuilder) -> Result<BoxedBoundAttrDecl<V>> {
            return Ok(BoxedBoundAttrDecl::wrap(super::ButtonDecl {
                value: (V::assemble(self.value.0)?, V::assemble(self.value.1)?),
                hold_time: self.hold_time,
                trigger: builder.input(self.trigger)?,
            }));
        }
    }
}

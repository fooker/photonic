use std::time::Duration;

use failure::Error;

use photonic_core::scene::{AttrBuilder, InputHandle};
use photonic_core::input::{Input, Poll};
use photonic_core::attr::{AttrValue, Attr, Update, BoundAttrDecl, Bounded, Bounds, UnboundAttrDecl};

#[derive(Clone, Copy, Debug)]
enum State {
    Released,
    Pressed(Duration),
}

impl State {
    fn update(self, duration: &Duration) -> Self {
        if let State::Pressed(remaining) = self {
            if remaining > *duration {
                return State::Pressed(remaining - *duration);
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
    where V: AttrValue {
    value_released: V,
    value_pressed: V,

    hold_time: Duration,

    state: State,

    trigger: Input<()>,
}

impl<V> Attr<V> for Button<V>
    where V: AttrValue {
    const KIND: &'static str = "button";

    fn get(&self) -> V {
        return match self.state {
            State::Released => self.value_released,
            State::Pressed(_) => self.value_pressed,
        };
    }

    fn update(&mut self, duration: &Duration) -> Update<V> {
        let state_old = self.state.pressed();

        if let Poll::Ready(()) = self.trigger.poll() {
            self.state = State::Pressed(self.hold_time)
        };

        self.state = self.state.update(duration);

        let state_new = self.state.pressed();

        return match (state_old, state_new) {
            (false, true) => Update::Changed(self.value_pressed),
            (true, false) => Update::Changed(self.value_released),
            _ => Update::Idle,
        };
    }
}

pub struct ButtonDecl<V>
    where V: AttrValue {
    pub value: (V, V),
    pub hold_time: Duration,
    pub trigger: InputHandle<()>,
}

impl<V> BoundAttrDecl<V> for ButtonDecl<V>
    where V: AttrValue + Bounded {
    type Target = Button<V>;
    fn materialize(self, bounds: Bounds<V>, builder: &mut AttrBuilder) -> Result<Self::Target, Error> {
        return Ok(Button {
            value_released: bounds.ensure(self.value.0)?,
            value_pressed: bounds.ensure(self.value.1)?,
            hold_time: self.hold_time,
            state: State::Released,
            trigger: builder.input("trigger", self.trigger)?,
        });
    }
}

impl<V> UnboundAttrDecl<V> for ButtonDecl<V>
    where V: AttrValue {
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

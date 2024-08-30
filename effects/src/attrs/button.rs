use std::time::Duration;

use anyhow::Result;

use photonic::attr::{Bounded, Bounds};
use photonic::input::{Input, Poll};
use photonic::scene::InputHandle;
use photonic::{scene, Attr, AttrBuilder, AttrValue, BoundAttrDecl, FreeAttrDecl};

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

pub struct ButtonAttr<V>
where V: AttrValue
{
    value_released: V,
    value_pressed: V,

    hold_time: Duration,

    state: State,

    trigger: Input<()>,
}

impl<V> Attr for ButtonAttr<V>
where V: AttrValue
{
    type Value = V;
    const KIND: &'static str = "button";

    fn update(&mut self, ctx: &scene::RenderContext) -> Self::Value {
        if let Poll::Update(()) = self.trigger.poll() {
            self.state = State::Pressed(self.hold_time)
        };

        self.state = self.state.update(ctx.duration);

        return match self.state.pressed() {
            true => self.value_pressed,
            false => self.value_released,
        };
    }
}

pub struct Button<V>
where V: AttrValue
{
    pub value_release: V,
    pub value_pressed: V,

    pub hold_time: Duration,

    pub trigger: InputHandle<()>,
}

impl<V> BoundAttrDecl for Button<V>
where V: AttrValue + Bounded
{
    type Value = V;
    type Attr = ButtonAttr<V>;

    fn materialize(self, bounds: Bounds<V>, builder: &mut AttrBuilder) -> Result<Self::Attr> {
        return Ok(ButtonAttr {
            value_released: bounds.ensure(self.value_release)?,
            value_pressed: bounds.ensure(self.value_pressed)?,
            hold_time: self.hold_time,
            state: State::Released,
            trigger: builder.input("trigger", self.trigger)?,
        });
    }
}

impl<V> FreeAttrDecl for Button<V>
where V: AttrValue
{
    type Value = V;
    type Attr = ButtonAttr<V>;

    fn materialize(self, builder: &mut AttrBuilder) -> Result<Self::Attr> {
        return Ok(ButtonAttr {
            value_released: self.value_release,
            value_pressed: self.value_pressed,
            hold_time: self.hold_time,
            state: State::Released,
            trigger: builder.input("trigger", self.trigger)?,
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
    pub struct Config<V: AttrValue> {
        #[serde(bound(deserialize = "V: Deserialize<'de>"))]
        pub value_release: V,
        #[serde(bound(deserialize = "V: Deserialize<'de>"))]
        pub value_pressed: V,
        pub hold_time: Duration,
        pub trigger: config::Input,
    }

    impl<V> Producible for Button<V>
    where V: AttrValue + DeserializeOwned
    {
        type Config = Config<V>;
    }

    pub fn free_attr<V, B>(config: Config<V>, builder: &mut B) -> Result<Button<V>>
    where
        B: photonic_dynamic::AttrBuilder,
        V: AttrValue + DeserializeOwned,
    {
        return Ok(Button {
            value_release: config.value_release,
            value_pressed: config.value_pressed,
            hold_time: config.hold_time,
            trigger: builder.input(config.trigger)?,
        });
    }

    pub fn bound_attr<V, B>(config: Config<V>, builder: &mut B) -> Result<Button<V>>
    where
        B: photonic_dynamic::AttrBuilder,
        V: AttrValue + DeserializeOwned + Bounded,
    {
        return Ok(Button {
            value_release: config.value_release,
            value_pressed: config.value_pressed,
            hold_time: config.hold_time,
            trigger: builder.input(config.trigger)?,
        });
    }
}

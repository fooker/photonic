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

impl<V> Attr<V> for ButtonAttr<V>
where V: AttrValue
{
    fn update(&mut self, ctx: &scene::RenderContext) -> V {
        if let Poll::Update(()) = self.trigger.poll(anyhow::Ok) {
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

impl<V> BoundAttrDecl<V> for Button<V>
where V: AttrValue + Bounded
{
    const KIND: &'static str = "button";

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

impl<V> FreeAttrDecl<V> for Button<V>
where V: AttrValue
{
    const KIND: &'static str = "button";

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

    use photonic::boxed::{DynBoundAttrDecl, DynFreeAttrDecl};
    use photonic_dynamic::factory::Producible;
    use photonic_dynamic::registry::Registry;
    use photonic_dynamic::{builder, config};

    use super::*;

    #[derive(Deserialize, Debug)]
    pub struct Config<V: AttrValue> {
        #[serde(bound(deserialize = "V: Deserialize<'de>"))]
        pub value_release: V,
        #[serde(bound(deserialize = "V: Deserialize<'de>"))]
        pub value_pressed: V,
        #[serde(with = "humantime_serde")]
        pub hold_time: Duration,
        pub trigger: config::Input,
    }

    impl<V> Producible<dyn DynFreeAttrDecl<V>> for Config<V>
    where V: AttrValue + DeserializeOwned
    {
        type Product = Button<V>;

        fn produce<Reg: Registry>(config: Self, mut builder: builder::AttrBuilder<'_, Reg>) -> Result<Self::Product> {
            return Ok(Button {
                value_release: config.value_release,
                value_pressed: config.value_pressed,
                hold_time: config.hold_time,
                trigger: builder.input(config.trigger)?,
            });
        }
    }

    impl<V> Producible<dyn DynBoundAttrDecl<V>> for Config<V>
    where V: AttrValue + DeserializeOwned + Bounded
    {
        type Product = Button<V>;

        fn produce<Reg: Registry>(config: Self, mut builder: builder::AttrBuilder<'_, Reg>) -> Result<Self::Product> {
            return Ok(Button {
                value_release: config.value_release,
                value_pressed: config.value_pressed,
                hold_time: config.hold_time,
                trigger: builder.input(config.trigger)?,
            });
        }
    }
}

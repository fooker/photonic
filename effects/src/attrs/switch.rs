use anyhow::Result;

use photonic::attr::{Bounded, Bounds};
use photonic::input::{Input, Poll};
use photonic::scene::InputHandle;
use photonic::{scene, Attr, AttrBuilder, AttrValue, BoundAttrDecl, FreeAttrDecl};

pub struct SwitchAttr<V>
where V: AttrValue
{
    value_released: V,
    value_pressed: V,

    pressed: bool,

    input: Input<bool>,
}

impl<V> Attr for SwitchAttr<V>
where V: AttrValue
{
    const KIND: &'static str = "button";
    type Value = V;

    fn update(&mut self, _ctx: &scene::RenderContext) -> Self::Value {
        if let Poll::Update(pressed) = self.input.poll() {
            self.pressed = pressed;
        };

        return match self.pressed {
            true => self.value_pressed,
            false => self.value_released,
        };
    }
}

pub struct Switch<V>
where V: AttrValue
{
    pub value_release: V,
    pub value_pressed: V,

    pub input: InputHandle<bool>,
}

impl<V> BoundAttrDecl for Switch<V>
where V: AttrValue + Bounded
{
    type Value = V;
    type Attr = SwitchAttr<V>;

    fn materialize(self, bounds: Bounds<V>, builder: &mut AttrBuilder) -> Result<Self::Attr> {
        return Ok(SwitchAttr {
            value_released: bounds.ensure(self.value_release)?,
            value_pressed: bounds.ensure(self.value_pressed)?,
            pressed: false,
            input: builder.input("input", self.input)?,
        });
    }
}

impl<V> FreeAttrDecl for Switch<V>
where V: AttrValue
{
    type Value = V;
    type Attr = SwitchAttr<V>;

    fn materialize(self, builder: &mut AttrBuilder) -> Result<Self::Attr> {
        return Ok(SwitchAttr {
            value_released: self.value_release,
            value_pressed: self.value_pressed,
            pressed: false,
            input: builder.input("input", self.input)?,
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

        pub input: config::Input,
    }

    impl<V> Producible for Switch<V>
    where V: AttrValue + DeserializeOwned
    {
        type Config = Config<V>;
    }

    pub fn free_attr<V, B>(config: Config<V>, builder: &mut B) -> Result<Switch<V>>
    where
        B: photonic_dynamic::AttrBuilder,
        V: AttrValue + DeserializeOwned,
    {
        return Ok(Switch {
            value_release: config.value_release,
            value_pressed: config.value_pressed,
            input: builder.input(config.input)?,
        });
    }

    pub fn bound_attr<V, B>(config: Config<V>, builder: &mut B) -> Result<Switch<V>>
    where
        B: photonic_dynamic::AttrBuilder,
        V: AttrValue + DeserializeOwned + Bounded,
    {
        return Ok(Switch {
            value_release: config.value_release,
            value_pressed: config.value_pressed,
            input: builder.input(config.input)?,
        });
    }
}
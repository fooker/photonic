use anyhow::Result;

use photonic::attr::{Bounded, Bounds};
use photonic::{scene, Attr, AttrBuilder, AttrValue, BoundAttrDecl, FreeAttrDecl};

pub struct SwitchAttr<V, I>
where
    V: AttrValue,
    I: Attr<bool>,
{
    value_released: V,
    value_pressed: V,

    pressed: I,
}

impl<V, I> Attr<V> for SwitchAttr<V, I>
where
    V: AttrValue,
    I: Attr<bool>,
{
    fn update(&mut self, ctx: &scene::RenderContext) -> V {
        let pressed = self.pressed.update(ctx);
        return match pressed {
            true => self.value_pressed,
            false => self.value_released,
        };
    }
}

pub struct Switch<V, I>
where V: AttrValue
{
    pub value_release: V,
    pub value_pressed: V,

    pub pressed: I,
}

impl<V, I> BoundAttrDecl<V> for Switch<V, I>
where
    V: AttrValue + Bounded,
    I: FreeAttrDecl<bool>,
{
    const KIND: &'static str = "button";

    type Attr = SwitchAttr<V, I::Attr>;

    fn materialize(self, bounds: Bounds<V>, builder: &mut AttrBuilder) -> Result<Self::Attr> {
        return Ok(SwitchAttr {
            value_released: bounds.ensure(self.value_release)?,
            value_pressed: bounds.ensure(self.value_pressed)?,
            pressed: builder.unbound_attr("pressed", self.pressed)?,
        });
    }
}

impl<V, I> FreeAttrDecl<V> for Switch<V, I>
where
    V: AttrValue,
    I: FreeAttrDecl<bool>,
{
    const KIND: &'static str = "button";

    type Attr = SwitchAttr<V, I::Attr>;

    fn materialize(self, builder: &mut AttrBuilder) -> Result<Self::Attr> {
        return Ok(SwitchAttr {
            value_released: self.value_release,
            value_pressed: self.value_pressed,
            pressed: builder.unbound_attr("input", self.pressed)?,
        });
    }
}

#[cfg(feature = "dynamic")]
pub mod dynamic {
    use serde::de::DeserializeOwned;
    use serde::Deserialize;

    use photonic::boxed::{BoxedFreeAttrDecl, DynBoundAttrDecl, DynFreeAttrDecl};
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

        pub pressed: config::Attr<bool>,
    }

    impl<V> Producible<dyn DynFreeAttrDecl<V>> for Config<V>
    where V: AttrValue + DeserializeOwned
    {
        type Product = Switch<V, BoxedFreeAttrDecl<bool>>;

        fn produce<Reg: Registry>(config: Self, mut builder: builder::AttrBuilder<'_, Reg>) -> Result<Self::Product> {
            return Ok(Switch {
                value_release: config.value_release,
                value_pressed: config.value_pressed,
                pressed: builder.free_attr("pressed", config.pressed)?,
            });
        }
    }

    impl<V> Producible<dyn DynBoundAttrDecl<V>> for Config<V>
    where V: AttrValue + DeserializeOwned + Bounded
    {
        type Product = Switch<V, BoxedFreeAttrDecl<bool>>;

        fn produce<Reg: Registry>(config: Self, mut builder: builder::AttrBuilder<'_, Reg>) -> Result<Self::Product> {
            return Ok(Switch {
                value_release: config.value_release,
                value_pressed: config.value_pressed,
                pressed: builder.free_attr("pressed", config.pressed)?,
            });
        }
    }
}

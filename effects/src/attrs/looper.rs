use anyhow::Result;
use num_traits::Num;

use photonic::attr::{Bounded, Bounds};
use photonic::input::{Input, Poll};
use photonic::scene::InputHandle;
use photonic::{scene, Attr, AttrBuilder, AttrValue, BoundAttrDecl};

pub struct LooperAttr<V>
where V: AttrValue + Num
{
    min: V,
    max: V,
    step: V,

    current: V,

    trigger: Input<()>,
}

impl<V> Attr<V> for LooperAttr<V>
where V: AttrValue + Num
{
    fn update(&mut self, _ctx: &scene::RenderContext) -> V {
        if let Poll::Update(()) = self.trigger.poll(anyhow::Ok) {
            self.current = self.min + (self.current + self.step - self.min) % (self.max - self.min);
        }

        return self.current;
    }
}

pub struct Looper<V>
where V: AttrValue
{
    pub step: V,

    pub trigger: InputHandle<()>,
}

impl<V> BoundAttrDecl<V> for Looper<V>
where V: AttrValue + Bounded + Num + PartialOrd
{
    const KIND: &'static str = "looper";

    type Attr = LooperAttr<V>;
    fn materialize(self, bounds: Bounds<V>, builder: &mut AttrBuilder) -> Result<Self::Attr> {
        let step = if self.step >= V::zero() {
            self.step
        } else {
            (self.step % (bounds.max - bounds.min + V::one())) + (bounds.max - bounds.min + V::one())
        };

        let min = bounds.min;
        let max = bounds.max + V::one();

        return Ok(LooperAttr {
            min,
            max,
            step,
            current: bounds.min,
            trigger: builder.input("trigger", self.trigger)?,
        });
    }
}

#[cfg(feature = "dynamic")]
pub mod dynamic {
    use anyhow::bail;
    use serde::de::DeserializeOwned;
    use serde::Deserialize;

    use photonic::boxed::DynBoundAttrDecl;
    use photonic_dynamic::factory::Producible;
    use photonic_dynamic::registry::Registry;
    use photonic_dynamic::{builder, config};

    use super::*;

    #[derive(Deserialize, Debug)]
    pub struct Config<V> {
        pub step: V,
        pub trigger: config::Input,
    }

    impl<V> Producible<dyn DynBoundAttrDecl<V>> for Config<V>
    where V: AttrValue + DeserializeOwned + Bounded
    {
        default type Product = !;

        default fn produce<Reg: Registry>(
            _config: Self,
            _builder: builder::AttrBuilder<'_, Reg>,
        ) -> Result<Self::Product> {
            bail!("Attribute 'looper' no available for value type {}", std::any::type_name::<V>());
        }
    }

    impl<V> Producible<dyn DynBoundAttrDecl<V>> for Config<V>
    where V: AttrValue + DeserializeOwned + Bounded + Num + PartialOrd
    {
        type Product = Looper<V>;

        fn produce<Reg: Registry>(config: Self, mut builder: builder::AttrBuilder<'_, Reg>) -> Result<Self::Product> {
            return Ok(Looper {
                step: config.step,
                trigger: builder.input(config.trigger)?,
            });
        }
    }
}

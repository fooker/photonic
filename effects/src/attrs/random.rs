use std::marker::PhantomData;

use anyhow::Result;
use rand::distr::uniform::SampleUniform;
use rand::distr::{Distribution, Uniform};
use rand::rngs::SmallRng;
use rand::SeedableRng;

use photonic::attr::{Bounded, Bounds};
use photonic::input::{Input, Poll, Trigger};
use photonic::scene::InputHandle;
use photonic::{scene, Attr, AttrBuilder, AttrValue, BoundAttrDecl};

pub struct RandomAttr<V>
where V: AttrValue + SampleUniform + Bounded
{
    uniform: Uniform<V>,
    random: SmallRng,

    current: V,

    trigger: Input<Trigger>,
}

impl<V> Attr<V> for RandomAttr<V>
where V: AttrValue + SampleUniform + Bounded
{
    fn update(&mut self, _ctx: &scene::RenderContext) -> V {
        if let Poll::Update(_) = self.trigger.poll(anyhow::Ok) {
            self.current = self.uniform.sample(&mut self.random);
        }

        return self.current;
    }
}

pub struct Random<V> {
    pub trigger: InputHandle<Trigger>,

    phantom: PhantomData<V>,
}

impl<V> BoundAttrDecl<V> for Random<V>
where V: AttrValue + SampleUniform + Bounded
{
    const KIND: &'static str = "random";

    type Attr = RandomAttr<V>;

    fn materialize(self, bounds: Bounds<V>, builder: &mut AttrBuilder) -> Result<Self::Attr> {
        let mut random = SmallRng::from_os_rng();
        let uniform = Uniform::new_inclusive(bounds.min, bounds.max)?;

        // Generate a random initial value
        let current = uniform.sample(&mut random);

        let trigger = builder.input("trigger", self.trigger)?;

        return Ok(RandomAttr {
            uniform,
            random,
            current,
            trigger,
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
    pub struct Config {
        pub trigger: config::Input,
    }

    impl<V> Producible<dyn DynBoundAttrDecl<V>> for Config
    where V: AttrValue + DeserializeOwned + Bounded
    {
        default type Product = !;

        default fn produce<Reg: Registry>(
            _config: Self,
            _builder: builder::AttrBuilder<'_, Reg>,
        ) -> Result<Self::Product> {
            bail!("Attribute 'random' no available for value type {}", std::any::type_name::<V>());
        }
    }

    impl<V> Producible<dyn DynBoundAttrDecl<V>> for Config
    where V: AttrValue + DeserializeOwned + Bounded + SampleUniform
    {
        type Product = Random<V>;

        fn produce<Reg: Registry>(config: Self, mut builder: builder::AttrBuilder<'_, Reg>) -> Result<Self::Product> {
            return Ok(Random {
                trigger: builder.input(config.trigger)?,
                phantom: Default::default(),
            });
        }
    }
}

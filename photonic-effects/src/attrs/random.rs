use std::time::Duration;

use anyhow::Error;
use rand::distributions::uniform::SampleUniform;
use rand::distributions::{Distribution, Uniform};
use rand::rngs::SmallRng;
use rand::SeedableRng;

use photonic_core::attr::{Attr, AttrValue, BoundAttrDecl, Bounded, Bounds, Update};
use photonic_core::input::{Input, Poll};
use photonic_core::scene::{AttrBuilder, InputHandle};
use std::marker::PhantomData;

pub struct Random<V>
where
    V: AttrValue + SampleUniform + Bounded,
{
    uniform: Uniform<V>,
    random: SmallRng,

    current: V,

    trigger: Input<()>,
}

impl<V> Attr for Random<V>
where
    V: AttrValue + SampleUniform + Bounded,
{
    type Element = V;
    const KIND: &'static str = "random";

    fn get(&self) -> V {
        self.current
    }

    fn update(&mut self, _duration: Duration) -> Update<V> {
        if let Poll::Ready(()) = self.trigger.poll() {
            self.current = self.uniform.sample(&mut self.random);
            return Update::Changed(self.current);
        } else {
            return Update::Idle(self.current);
        }
    }
}

pub struct RandomDecl<V> {
    pub trigger: InputHandle<()>,
    phantom: PhantomData<V>,
}

impl<V> BoundAttrDecl for RandomDecl<V>
where
    V: AttrValue + SampleUniform + Bounded,
{
    type Element = V;
    type Target = Random<V>;
    fn materialize(
        self,
        bounds: Bounds<V>,
        builder: &mut AttrBuilder,
    ) -> Result<Self::Target, Error> {
        let mut random = SmallRng::from_entropy();
        let uniform = Uniform::new_inclusive(bounds.min, bounds.max);

        // Generate a random initial value
        let current = uniform.sample(&mut random);

        let trigger = builder.input("trigger", self.trigger)?;

        return Ok(Random {
            uniform,
            random,
            current,
            trigger,
        });
    }
}

#[cfg(feature = "dyn")]
pub mod model {

    use anyhow::{format_err, Result};
    use serde::Deserialize;

    use photonic_core::attr::Bounded;
    use photonic_core::boxed::{BoxedBoundAttrDecl, Wrap};
    use photonic_dyn::builder::AttrBuilder;
    use photonic_dyn::config;
    use photonic_dyn::model::{AttrValueFactory, BoundAttrModel};
    use rand::distributions::uniform::SampleUniform;

    #[derive(Deserialize)]
    pub struct RandomModel {
        pub trigger: config::Input,
    }

    impl<V> BoundAttrModel<V> for RandomModel
    where
        V: AttrValueFactory + Bounded,
    {
        default fn assemble(
            self,
            _builder: &mut impl AttrBuilder,
        ) -> Result<BoxedBoundAttrDecl<V>> {
            return Err(format_err!(
                "Random is not supported for Attributes of Type {}",
                std::any::type_name::<V>()
            ));
        }
    }

    impl<V> BoundAttrModel<V> for RandomModel
    where
        V: AttrValueFactory + Bounded + SampleUniform,
    {
        fn assemble(self, builder: &mut impl AttrBuilder) -> Result<BoxedBoundAttrDecl<V>> {
            return Ok(BoxedBoundAttrDecl::wrap(super::RandomDecl {
                trigger: builder.input(self.trigger)?,
                phantom: Default::default(),
            }));
        }
    }
}

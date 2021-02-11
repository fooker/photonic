use std::time::Duration;

use failure::Error;
use rand::distributions::uniform::SampleUniform;
use rand::prelude::{FromEntropy, Rng, SmallRng};

use photonic_core::scene::{AttrBuilder, InputHandle};
use photonic_core::input::{Input, Poll};
use photonic_core::attr::{AttrValue, Attr, Update, BoundAttrDecl, Bounded, Bounds};

pub struct Random<V>
    where V: AttrValue + Bounded {
    bounds: Bounds<V>,

    current: V,

    random: SmallRng,

    trigger: Input<()>,
}

impl<V> Attr<V> for Random<V>
    where V: AttrValue + SampleUniform + Bounded {
    const KIND: &'static str = "random";

    fn get(&self) -> V {
        self.current
    }

    fn update(&mut self, _duration: &Duration) -> Update<V> {
        if let Poll::Ready(()) = self.trigger.poll() {
            self.current = self.random.gen_range(self.bounds.min, self.bounds.max);
            return Update::Changed(self.current);
        } else {
            return Update::Idle(self.current);
        }
    }
}

pub struct RandomDecl {
    pub trigger: InputHandle<()>,
}

impl<V> BoundAttrDecl<V> for RandomDecl
    where V: AttrValue + SampleUniform + Bounded {
    type Target = Random<V>;
    fn materialize(self, bounds: Bounds<V>, builder: &mut AttrBuilder) -> Result<Self::Target, Error> {
        let mut random = SmallRng::from_entropy();

        // Generate a random initial value
        let current = random.gen_range(bounds.min, bounds.max);

        let trigger = builder.input("trigger", self.trigger)?;

        return Ok(Random {
            bounds,
            current,
            random,
            trigger,
        });
    }
}

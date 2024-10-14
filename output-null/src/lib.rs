use anyhow::Result;
use std::marker::PhantomData;

use photonic::{BufferReader, Output, OutputDecl};

#[derive(Default)]
pub struct Null<E> {
    size: usize,
    phantom: PhantomData<E>,
}

pub struct NullOutput<E> {
    size: usize,
    phantom: PhantomData<E>,
}

impl<E> Null<E> {
    pub fn with_size(size: usize) -> Self {
        return Self {
            size,
            phantom: PhantomData,
        };
    }
}

impl<E> OutputDecl for Null<E> {
    const KIND: &'static str = "null";
    type Output = NullOutput<E>;

    async fn materialize(self) -> Result<Self::Output>
    where Self::Output: Sized {
        return Ok(Self::Output {
            size: self.size,
            phantom: self.phantom,
        });
    }
}

impl<E> Output for NullOutput<E> {
    const KIND: &'static str = "null";

    type Element = E;

    async fn render(&mut self, _: impl BufferReader<Element = Self::Element>) -> Result<()> {
        return Ok(());
    }

    fn size(&self) -> usize {
        return self.size;
    }
}

#[cfg(feature = "dynamic")]
pub mod dynamic {
    use anyhow::Result;
    use palette::rgb::Rgb;
    use serde::Deserialize;

    use photonic::boxed::DynOutputDecl;
    use photonic_dynamic::factory::{factory, OutputFactory, Producible};
    use photonic_dynamic::{builder, registry};

    use crate::Null;

    #[derive(Deserialize)]
    pub struct Config {
        size: usize,
    }

    impl Producible<dyn DynOutputDecl> for Config {
        type Product = Null<Rgb>;

        fn produce<Reg: registry::Registry>(
            config: Self,
            _builder: builder::OutputBuilder<'_, Reg>,
        ) -> Result<Self::Product> {
            return Ok(Null::with_size(config.size));
        }
    }

    pub struct Registry;

    impl registry::Registry for Registry {
        fn output<Reg: registry::Registry>(kind: &str) -> Option<OutputFactory<Reg>> {
            return match kind {
                "null" => Some(factory::<Config>()),
                _ => return None,
            };
        }
    }
}

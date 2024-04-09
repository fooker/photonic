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
            phantom: PhantomData::default(),
        };
    }
}

impl<E> OutputDecl for Null<E> {
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

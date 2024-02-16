use anyhow::Result;
use std::marker::PhantomData;

use photonic::{BufferReader, Output, OutputDecl};

#[derive(Default)]
pub struct Null<E> {
    phantom: PhantomData<E>,
}

pub struct NullOutput<E> {
    phantom: PhantomData<E>,
}

impl<E> OutputDecl for Null<E> {
    type Output = NullOutput<E>;

    async fn materialize(self, _size: usize) -> Result<Self::Output>
    where Self::Output: Sized {
        return Ok(Self::Output {
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
}

use std::future::Future;
use std::marker::PhantomData;

use anyhow::Result;
use async_trait::async_trait;
use palette::rgb::Rgb;
use palette::{FromColor, IntoColor};

use crate::config::Anything;
use photonic::{BufferReader, Output, OutputDecl};
use photonic_dynamic_registry::Producible;

#[async_trait(?Send)]
pub trait DynOutputDecl {
    async fn materialize(self: Box<Self>) -> Result<BoxedOutput>;
}

#[async_trait(?Send)]
impl<T> DynOutputDecl for T
where
    T: OutputDecl + 'static,
    <<T as OutputDecl>::Output as Output>::Element: Copy + FromColor<Rgb>,
{
    async fn materialize(self: Box<Self>) -> Result<BoxedOutput> {
        let output = <T as OutputDecl>::materialize(*self).await?;
        return Ok(Box::new(output) as Box<dyn DynOutput>);
    }
}

pub type BoxedOutputDecl = Box<dyn DynOutputDecl>;

impl Producible for BoxedOutputDecl {
    type Config = Anything;
}

impl OutputDecl for BoxedOutputDecl {
    type Output = BoxedOutput;

    fn materialize(self) -> impl Future<Output = Result<Self::Output>> {
        return DynOutputDecl::materialize(self);
    }
}

#[async_trait(? Send)]
pub trait DynOutput {
    async fn render(&mut self, out: &dyn BufferReader<Element = Rgb>) -> Result<()>;

    fn size(&self) -> usize;
}

#[async_trait(? Send)]
impl<T> DynOutput for T
where
    T: Output,
    <T as Output>::Element: Copy + FromColor<Rgb>,
{
    async fn render(&mut self, out: &dyn BufferReader<Element = Rgb>) -> Result<()> {
        return Output::render(self, OutputBuffer::wrap(out)).await;
    }

    fn size(&self) -> usize {
        return Output::size(self);
    }
}

pub type BoxedOutput = Box<dyn DynOutput>;

impl Output for BoxedOutput {
    const KIND: &'static str = todo!();

    type Element = Rgb;

    async fn render(&mut self, out: impl BufferReader<Element = Self::Element>) -> Result<()> {
        return DynOutput::render(self.as_mut(), &out).await;
    }

    fn size(&self) -> usize {
        return DynOutput::size(self.as_ref());
    }
}

struct OutputBuffer<'a, E> {
    buffer: &'a dyn BufferReader<Element = Rgb>,
    phantom: PhantomData<E>,
}

impl<'a, E> OutputBuffer<'a, E> {
    pub fn wrap(buffer: &'a dyn BufferReader<Element = Rgb>) -> Self {
        return Self {
            buffer,
            phantom: PhantomData::default(),
        };
    }
}

impl<'a, E> BufferReader for OutputBuffer<'a, E>
where E: Copy + FromColor<Rgb>
{
    type Element = E;

    fn get(&self, index: usize) -> Self::Element {
        return self.buffer.get(index).into_color();
    }

    fn size(&self) -> usize {
        return self.buffer.size();
    }
}

use std::future::Future;

use crate::BufferReader;

pub trait Output: Sized
{
    const KIND: &'static str;

    type Element;

    fn render(&mut self, out: impl BufferReader<Element=Self::Element>) -> impl Future<Output=anyhow::Result<()>>;
}

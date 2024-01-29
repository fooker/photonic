use std::future::Future;

use crate::BufferReader;
use anyhow::Result;

pub trait Output: Sized
{
    const KIND: &'static str;

    type Element;

    fn render(&mut self, out: impl BufferReader<Element=Self::Element>) -> impl Future<Output=Result<()>>;
}

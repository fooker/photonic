use std::usize;
use super::BufferReader;

pub struct IMap<'a, B, F>
{
    buffer: &'a B,
    mapper: F,
}

impl<'a, B, F> IMap<'a, B, F>
{
    pub(super) fn new(buffer: &'a B, mapper: F) -> Self {
        return IMap {
            buffer,
            mapper,
        };
    }
}

impl<B, R, F> BufferReader for IMap<'_, B, F>
    where B: BufferReader,
          F: Fn(usize, B::Element) -> R,
{
    type Element = R;

    fn get(&self, index: usize) -> R {
        return (self.mapper)(index, self.buffer.get(index));
    }

    fn size(&self) -> usize {
        return self.buffer.size();
    }
}

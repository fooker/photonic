use super::BufferReader;
use std::ops::Range;

pub struct MapRange<'a, 'r, B, F> {
    buffer: &'a B,
    range: &'r Range<usize>,
    mapper: F,
}

impl<'a, 'r, B, F> MapRange<'a, 'r, B, F> {
    pub(super) fn new(buffer: &'a B, range: &'r Range<usize>, mapper: F) -> Self {
        return MapRange {
            buffer,
            range,
            mapper,
        };
    }
}

impl<B, F> BufferReader for MapRange<'_, '_, B, F>
where
    B: BufferReader,
    F: Fn(B::Element) -> B::Element,
{
    type Element = B::Element;

    fn get(&self, index: usize) -> Self::Element {
        let value = self.buffer.get(index);
        return if self.range.contains(&index) { (self.mapper)(value) } else { value };
    }

    fn size(&self) -> usize {
        return self.buffer.size();
    }
}

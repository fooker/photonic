use super::BufferReader;

pub struct Map<'a, B, F>
{
    buffer: &'a B,
    mapper: F,
}

impl<'a, B, F> Map<'a, B, F>
{
    pub(super) fn map(buffer: &'a B, mapper: F) -> Self {
        return Map {
            buffer,
            mapper,
        };
    }
}

impl<B, R, F> BufferReader for Map<'_, B, F>
    where B: BufferReader,
          F: Fn(B::Element) -> R,
          R: Copy,
{
    type Element = R;

    fn get(&self, index: usize) -> R {
        return (self.mapper)(self.buffer.get(index));
    }

    fn size(&self) -> usize {
        return self.buffer.size();
    }
}

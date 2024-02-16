use super::BufferReader;

pub struct Lerp<'a, 'b, A, B>
{
    a: &'a A,
    b: &'b B,
    i: f32,
}

impl<'a, 'b, A, B> Lerp<'a, 'b, A, B>
{
    pub(super) fn new(a: &'a A, b: &'b B, i: f32) -> Self {
        return Lerp { a, b, i };
    }
}

impl<A, B, E> BufferReader for Lerp<'_, '_, A, B>
    where A: BufferReader<Element=E>,
          B: BufferReader<Element=E>,
          E: crate::math::Lerp,
{
    type Element = E;

    fn get(&self, index: usize) -> Self::Element {
        let a = self.a.get(index);
        let b = self.b.get(index);
        return E::lerp(a, b, self.i);
    }

    fn size(&self) -> usize {
        return usize::min(self.a.size(), self.b.size());
    }
}

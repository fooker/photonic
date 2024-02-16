use std::ops::{Deref, DerefMut, Range};

use anyhow::Result;
use crate::math::Lerp;

mod map;
mod imap;
mod map_range;
mod lerp;

/// A buffer for data
pub struct Buffer<E> {
    data: Box<[E]>,
}

impl<E> Buffer<E> {
    /// Generate a buffer with the given size by calling the generator for each element.
    ///
    /// The generator will be called for each element in the buffer with the index.
    pub fn from_generator<F, R>(size: usize, generator: F) -> Self
        where F: Fn(usize) -> R,
              R: Into<E>,
    {
        let data = (0..size)
            .map(generator)
            .map(Into::into)
            .collect();

        return Self { data };
    }

    /// Returns the size of the buffer.
    pub fn size(&self) -> usize {
        return self.data.len();
    }

    pub fn get(&self, index: usize) -> &E {
        return &self.data[index % self.data.len()];
    }

    pub fn set(&mut self, index: usize, value: E) {
        self.data[index % self.data.len()] = value;
    }

    pub fn iter(&self) -> impl Iterator<Item=&E> {
        return self.data.iter();
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut E> {
        return self.data.iter_mut();
    }

    /// Update the buffer by calling `f` for each element in the buffer.
    ///
    /// The provided function is called with the index of the element to update and current element
    /// in buffer. The value returned by the function will be the new value stored in the buffer.
    pub fn update(&mut self, f: impl Fn(usize, &E) -> E) {
        self.data.iter_mut()
            .enumerate()
            .for_each(|(i, e)| *e = f(i, e));
    }

    /// Update the buffer by calling `f` for each element in the buffer.
    ///
    /// The provided function is called with the index of the element to update and current element
    /// in buffer. On successful return of the function, the value will be stored in the buffer
    /// until the first  call fails.
    pub fn try_update<F>(&mut self, f: impl Fn(usize, &E) -> Result<E>) -> Result<()> {
        self.data = self.data.into_iter()
            .enumerate()
            .map(|(i, e)| f(i, e))
            .collect::<Result<_>>()?;
        return Ok(());
    }

    pub fn blit_from(&mut self, source: impl BufferReader<Element=E>) {
        assert_eq!(self.size(), source.size());

        for i in 0..self.size() {
            self.data[i] = source.get(i);
        }
    }
}

impl<E> Buffer<E>
    where E: Default {
    /// Create a buffer and initialize it with the default value.
    pub fn with_default(size: usize) -> Self {
        return Self::from_generator(size, |_| E::default());
    }
}

impl<E> Deref for Buffer<E> {
    type Target = [E];

    fn deref(&self) -> &Self::Target {
        return &self.data;
    }
}

impl<E> DerefMut for Buffer<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.data;
    }
}

impl<E> AsRef<[E]> for Buffer<E> {
    fn as_ref(&self) -> &[E] {
        return self.data.as_ref();
    }
}

impl<E> AsMut<[E]> for Buffer<E> {
    fn as_mut(&mut self) -> &mut [E] {
        return self.data.as_mut();
    }
}

pub trait BufferReader {
    type Element;

    fn get(&self, index: usize) -> Self::Element;

    fn size(&self) -> usize;

    fn iter(&self) -> impl Iterator<Item=Self::Element>
        where Self: Sized,
    {
        return (0..self.size()).map(|i| self.get(i));
    }

    fn map<R, F>(&self, f: F) -> impl BufferReader<Element=R>
        where Self: Sized,
              F: Fn(Self::Element) -> R,
    {
        return map::Map::new(self, f);
    }

    fn imap<R, F>(&self, f: F) -> impl BufferReader<Element=R>
        where Self: Sized,
              F: Fn(usize, Self::Element) -> R,
    {
        return imap::IMap::new(self, f);
    }

    fn map_range<F>(&self, range: &Range<usize>, f: F) -> impl BufferReader<Element=Self::Element>
        where Self: Sized,
              F: Fn(Self::Element) -> Self::Element,
    {
        return map_range::MapRange::new(self, range, f);
    }

    fn lerp<R>(&self, other: &R, i: f32) -> impl BufferReader<Element=Self::Element>
        where Self: Sized,
              Self::Element: Lerp,
              R: BufferReader<Element=Self::Element>,
    {
        return lerp::Lerp::new(self, other, i);
    }
}

impl<E> BufferReader for Buffer<E>
    where E: Copy,
{
    type Element = E;

    fn get(&self, index: usize) -> Self::Element {
        return *self.get(index);
    }

    fn size(&self) -> usize {
        return self.size();
    }
}

impl<E> BufferReader for &Buffer<E>
    where E: Copy,
{
    type Element = E;

    fn get(&self, index: usize) -> Self::Element {
        return self.get(index);
    }

    fn size(&self) -> usize {
        return self.size();
    }
}

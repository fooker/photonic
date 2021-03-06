use std::time::Duration;

use crate::color::Black;
use crate::node::{Node, Render, RenderType};

pub struct Buffer<E> {
    data: Box<[E]>,
}

impl<E> Buffer<E> {
    pub fn update<F>(&mut self, f: F)
    where
        F: Fn(usize, &E) -> E,
    {
        for (i, e) in self.data.iter_mut().enumerate() {
            *e = f(i, e);
        }
    }
}

impl<E> Buffer<E>
where
    E: Default,
{
    pub fn new(size: usize) -> Self {
        Self::from_generator(size, |_| E::default())
    }
}

impl<E> Buffer<E>
where
    E: Black,
{
    pub fn black(size: usize) -> Self {
        Self::from_generator(size, |_| E::black())
    }
}

impl<E> Buffer<E> {
    pub fn from_generator<F, R>(size: usize, generator: F) -> Self
    where
        F: FnMut(usize) -> R,
        R: Into<E>,
    {
        Self {
            data: (0..size).map(generator).map(|v| v.into()).collect(),
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, index: usize) -> &E {
        return &self.data[index % self.data.len()];
    }

    pub fn set(&mut self, index: usize, value: E) {
        self.data[index % self.data.len()] = value
    }

    pub fn iter(&self) -> impl Iterator<Item = &E> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut E> {
        self.data.iter_mut()
    }
}

impl<'a, E> RenderType<'a, Self> for Buffer<E>
where
    E: Copy + 'static,
{
    type Render = &'a Self;
}

impl<E> Node for Buffer<E>
where
    E: Copy + 'static,
{
    type Element = E;

    const KIND: &'static str = "buffer";

    fn update(&mut self, _duration: Duration) {}

    fn render(&mut self) -> <Self as RenderType<Self>>::Render {
        return self;
    }
}

impl<'a, E> Render for Buffer<E>
where
    E: Copy,
{
    type Element = E;

    fn get(&self, index: usize) -> Self::Element {
        return *Buffer::get(self, index);
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

impl<'a, E> Render for &'a Buffer<E>
where
    E: Copy,
{
    type Element = E;

    fn get(&self, index: usize) -> Self::Element {
        return *Buffer::get(self, index);
    }
}

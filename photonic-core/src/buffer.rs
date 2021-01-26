use std::time::Duration;

use crate::color::*;
use crate::core::*;

pub struct Buffer<E> {
    data: Box<[E]>,
}

impl<E> Buffer<E>
    where E: Default {
    pub fn new(size: usize) -> Self {
        Self::from_generator(size, |_| E::default())
    }
}

impl<E> Buffer<E>
    where E: Black {
    pub fn black(size: usize) -> Self {
        Self::from_generator(size, |_| E::black())
    }
}

impl<E> Buffer<E> {
    pub fn from_generator<F, R>(size: usize, generator: F) -> Self
        where F: FnMut(usize) -> R,
              R: Into<E> {
        Self {
            data: (0..size)
                .map(generator)
                .map(|v| v.into())
                .collect(),
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

    pub fn iter(&self) -> impl Iterator<Item=&E> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut E> {
        self.data.iter_mut()
    }
}

impl<'a, E> RenderType<'a> for Buffer<E>
    where E: Copy + 'a {
    type Element = E;
    type Render = &'a Self;
}

impl<E> Node for Buffer<E>
    where E: Copy + 'static {
    const TYPE: &'static str = "buffer";

    fn render<'a>(&'a self, _renderer: &'a Renderer) -> <Self as RenderType<'a>>::Render {
        return self;
    }

    fn update(&mut self, _duration: &Duration) {}
}

impl<'a, E> Render for Buffer<E>
    where E: Copy {
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
    where E: Copy {
    type Element = E;
    fn get(&self, index: usize) -> Self::Element {
        return *Buffer::get(self, index);
    }
}

use crate::color::*;
use crate::core::*;

pub struct Buffer<C>
    where C: Color + Copy {
    data: Vec<C>,
}

impl<C> Buffer<C>
    where C: Color + Copy + Default {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![C::default(); size],
        }
    }
}

impl<C> Buffer<C>
    where C: Color + Copy + Black {
    pub fn black(size: usize) -> Self {
        Self {
            data: vec![C::black(); size],
        }
    }
}

impl<C> Buffer<C>
    where C: Color + Copy {
    pub fn from_generator<F, R>(size: usize, generator: F) -> Self
        where F: FnMut(usize) -> R,
              R: Color {
        Self {
            data: (0..size)
                .map(generator)
                .map(|v| v.convert())
                .collect(),
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, index: usize) -> C {
        return self.data[index];
    }

    pub fn set(&mut self, index: usize, value: C) {
        self.data[index] = value
    }
}

impl<C> Render for Buffer<C>
    where C: Color + Copy {
    fn get(&self, index: usize) -> MainColor {
        // FIXME: Do not convert for same color type (maybe use into instead?)
        Buffer::get(self, index).convert()
    }
}

impl<'a, C> Render for &'a Buffer<C>
    where C: Color + Copy {
    fn get(&self, index: usize) -> MainColor {
        // FIXME: Do not convert for same color type (maybe use into instead?)
        Buffer::get(self, index).convert()
    }
}

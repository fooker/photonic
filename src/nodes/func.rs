use buffer::*;
use core::*;
use color::*;
use std::time::Duration;


pub struct BufferNode(Buffer<MainColor>);

impl BufferNode {
    pub fn generate<C, F>(size: usize, generator: F) -> Self
        where C: Color,
              F: FnMut(usize) -> C {
        return Self(Buffer::from_generator(size, generator));
    }
}

impl Node for BufferNode {
    fn render<'a>(&'a self) -> Box<Renderer + 'a> {
        Box::new(&self.0)
    }
}

impl Dynamic for BufferNode {
    fn update(&mut self, duration: Duration) {}
}


pub struct GeneratorNode<C, F>
    where C: Color,
          F: Fn(usize) -> C {
    size: usize,
    generator: F,
}

impl<C, F> GeneratorNode<C, F>
    where C: Color,
          F: Fn(usize) -> C {
    pub fn new(size: usize, generator: F) -> Self {
        return Self {
            size,
            generator,
        };
    }
}

impl<C, F> Node for GeneratorNode<C, F>
    where C: Color,
          F: Fn(usize) -> C {
    fn render(&self) -> Box<Renderer> {
        Box::new(Buffer::<MainColor>::from_generator(self.size, |i| (self.generator)(i)))
    }
}

impl<C, F> Dynamic for GeneratorNode<C, F>
    where C: Color,
          F: Fn(usize) -> C {
    fn update(&mut self, duration: Duration) {}
}

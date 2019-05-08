use std::time::Duration;

use failure::Error;

use photonic_core::core::*;

// TODO: Use references instead of cloning

struct SolidRenderer<'a, E>(&'a E)
    where E: Clone;

impl<'a, E> Render for SolidRenderer<'a, E>
    where E: Clone {
    type Element = E;

    fn get(&self, _index: usize) -> Self::Element {
        return self.0.clone();
    }
}

pub struct SolidNodeDecl<E>
    where E: Clone {
    pub solid: E,
}

impl<E> NodeDecl for SolidNodeDecl<E>
    where E: Clone {
    type Element = E;
    type Target = SolidNode<Self::Element>;

    fn new(self, _size: usize) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            solid: self.solid,
        });
    }
}

pub struct SolidNode<E>
    where E: Clone {
    solid: E,
}

impl<E> Dynamic for SolidNode<E>
    where E: Clone {
    fn update(&mut self, _duration: &Duration) {}
}

impl<E> Node for SolidNode<E>
    where E: Clone {
    type Element = E;

    fn render<'a>(&'a self, _renderer: &'a Renderer) -> Box<Render<Element=Self::Element> + 'a> {
        return Box::new(SolidRenderer(&self.solid));
    }
}

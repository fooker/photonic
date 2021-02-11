use std::time::Duration;

use failure::Error;

use crate::scene::NodeBuilder;

pub trait Render {
    type Element;

    fn get(&self, index: usize) -> Self::Element;
}

impl<E, F> Render for F
    where F: Fn(usize) -> E {
    type Element = E;

    fn get(&self, index: usize) -> Self::Element {
        return self(index);
    }
}

pub trait NodeDecl {
    type Element;
    type Target: Node<Element=Self::Element> + 'static;

    fn materialize(self, size: usize, builder: &mut NodeBuilder) -> Result<Self::Target, Error>
        where Self::Target: std::marker::Sized;
}

pub trait RenderType<'a> {
    type Element;
    type Render: Render<Element=Self::Element> + 'a;
}

pub trait Node: for<'a> RenderType<'a> {
    const KIND: &'static str;

    fn update(&mut self, duration: &Duration);
    fn render(&mut self) -> <Self as RenderType>::Render;
}

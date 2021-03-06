use std::time::Duration;

use anyhow::Error;

use photonic_core::node::{Node, NodeDecl, Render, RenderType};
use photonic_core::scene::NodeBuilder;

pub struct SolidRenderer<'a, E>(&'a E);

impl<'a, E> Render for SolidRenderer<'a, E>
where
    E: Copy,
{
    type Element = E;

    fn get(&self, _index: usize) -> Self::Element {
        return *self.0;
    }
}

pub struct SolidNodeDecl<E>
where
    E: Clone,
{
    pub solid: E,
}

impl<E> NodeDecl for SolidNodeDecl<E>
where
    E: Copy + 'static,
{
    type Element = E;
    type Target = SolidNode<Self::Element>;

    fn materialize(self, _size: usize, _builder: &mut NodeBuilder) -> Result<Self::Target, Error> {
        return Ok(Self::Target { solid: self.solid });
    }
}

pub struct SolidNode<E> {
    solid: E,
}

impl<'a, E> RenderType<'a, Self> for SolidNode<E>
where
    E: Copy + 'static,
{
    type Render = SolidRenderer<'a, E>;
}

impl<E> Node for SolidNode<E>
where
    E: Copy + 'static,
{
    const KIND: &'static str = "solid";

    type Element = E;

    fn update(&mut self, _duration: Duration) {}

    fn render(&mut self) -> <Self as RenderType<Self>>::Render {
        return SolidRenderer(&self.solid);
    }
}

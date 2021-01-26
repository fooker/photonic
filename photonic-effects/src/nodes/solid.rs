use std::time::Duration;

use failure::Error;

use photonic_core::core::*;

pub struct SolidRenderer<'a, E>(&'a E);

impl<'a, E> Render for SolidRenderer<'a, E>
    where E: Copy {
    type Element = E;

    fn get(&self, _index: usize) -> Self::Element {
        return *self.0;
    }
}

pub struct SolidNodeDecl<E>
    where E: Clone {
    pub solid: E,
}

impl<E> NodeDecl for SolidNodeDecl<E>
    where E: Copy + 'static {
    type Element = E;
    type Target = SolidNode<Self::Element>;

    fn materialize(self, _size: usize, _builder: &mut SceneBuilder) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            solid: self.solid,
        });
    }
}

pub struct SolidNode<E> {
    solid: E,
}

impl<'a, E> RenderType<'a> for SolidNode<E>
    where E: Copy + 'static {
    type Element = E;
    type Render = SolidRenderer<'a, E>;
}

impl<E> Node for SolidNode<E>
    where E: Copy + 'static {
    const TYPE: &'static str = "solid";

    fn update(&mut self, _duration: &Duration) {}

    fn render<'a>(&'a self, _renderer: &'a Renderer) -> <Self as RenderType<'a>>::Render {
        return SolidRenderer(&self.solid);
    }
}

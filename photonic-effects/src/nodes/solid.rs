use std::time::Duration;

use failure::Error;

use photonic_core::color::Color;
use photonic_core::core::*;

struct SolidRenderer(MainColor);

impl Render for SolidRenderer {
    fn get(&self, _index: usize) -> MainColor {
        return self.0;
    }
}

pub struct SolidNodeDecl<C>
    where C: Color {
    pub color: C,
}

impl<C> NodeDecl for SolidNodeDecl<C>
    where C: Color {
    type Target = SolidNode;

    fn new(self, _size: usize) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            color: self.color.convert(),
        });
    }
}

pub struct SolidNode{
    color: MainColor,
}

impl Node for SolidNode {
    fn update(&mut self, _duration: &Duration) {}

    fn render<'a>(&'a self, _renderer: &'a Renderer<'a>) -> Box<Render> {
        return Box::new(SolidRenderer(self.color));
    }
}

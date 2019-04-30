use std::time::Duration;

use failure::Error;

use crate::core::*;
use crate::math;
use crate::math::Lerp;
use crate::values::*;

struct RotationRenderer<'a> {
    source: Box<Render + 'a>,
    size: usize,
    offset: f64,
}

impl<'a> Render for RotationRenderer<'a> {
    fn get(&self, index: usize) -> MainColor {
        let index = math::wrap((index as f64) - self.offset, self.size);
        let index = (index.trunc() as usize, index.fract());

        let c1 = self.source.get((index.0 + 0) % self.size);
        let c2 = self.source.get((index.0 + 1) % self.size);

        return MainColor::lerp(c1, c2, index.1);
    }
}

pub struct RotationNodeDecl<Source: Node> {
    source: Handle<Source>,
    speed: Box<UnboundValueDecl<f64>>,
}

pub struct RotationNode<Source: Node> {
    size: usize,

    source: Handle<Source>,

    speed: Box<Value<f64>>,

    offset: f64,
}

impl<Source: Node> NodeDecl for RotationNodeDecl<Source> {
    type Target = RotationNode<Source>;

    fn new(self, size: usize) -> Result<Self::Target, Error> {
        let speed = self.speed.new()?;

        return Ok(Self::Target {
            size,
            source: self.source,
            speed,
            offset: 0.0,
        });
    }
}

impl<Source: Node> Node for RotationNode<Source> {
    fn update(&mut self, duration: &Duration) {
        self.speed.update(duration);
        self.offset += self.speed.get() * duration.as_secs_f64();
    }

    fn render<'a>(&'a self, renderer: &'a Renderer) -> Box<Render + 'a> {
        Box::new(RotationRenderer {
            source: renderer.render(&self.source),
            size: self.size,
            offset: self.offset,
        })
    }
}

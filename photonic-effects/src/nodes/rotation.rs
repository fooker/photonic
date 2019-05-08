use std::time::Duration;

use failure::Error;

use photonic_core::core::*;
use photonic_core::math;
use photonic_core::math::Lerp;
use photonic_core::value::*;

struct RotationRenderer<'a, E> {
    source: Box<Render<Element=E> + 'a>,
    size: usize,
    offset: f64,
}

impl<'a, E> Render for RotationRenderer<'a, E>
    where E: Lerp {
    type Element = E;

    fn get(&self, index: usize) -> E {
        let index = math::wrap((index as f64) - self.offset, self.size);
        let index = (index.trunc() as usize, index.fract());

        let c1 = self.source.get((index.0 + 0) % self.size);
        let c2 = self.source.get((index.0 + 1) % self.size);

        return E::lerp(c1, c2, index.1);
    }
}

pub struct RotationNodeDecl<Source> {
    source: Handle<Source>,
    speed: Box<UnboundValueDecl<f64>>,
}

pub struct RotationNode<Source> {
    size: usize,

    source: Handle<Source>,

    speed: Box<Value<f64>>,

    offset: f64,
}

impl<Source, E> NodeDecl for RotationNodeDecl<Source>
    where Source: Node<Element=E>,
          E: Lerp + 'static {
    type Element = E;
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

impl<Source> Dynamic for RotationNode<Source> {
    fn update(&mut self, duration: &Duration) {
        self.speed.update(duration);
        self.offset += self.speed.get() * duration.as_secs_f64();
    }
}

impl<Source, E> Node for RotationNode<Source>
    where Source: Node<Element=E>,
          E: Lerp + 'static {
    type Element = E;
    fn render<'a>(&'a self, renderer: &'a Renderer) -> Box<Render<Element=Self::Element> + 'a> {
        Box::new(RotationRenderer {
            source: renderer.render(&self.source),
            size: self.size,
            offset: self.offset,
        })
    }
}

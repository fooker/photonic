use std::time::Duration;

use failure::Error;

use photonic_core::core::*;
use photonic_core::math;
use photonic_core::math::Lerp;
use photonic_core::value::*;

pub struct RotationRenderer<Source> {
    source: Source,
    size: usize,
    offset: f64,
}

impl<Source> Render for RotationRenderer<Source>
    where Source: Render,
          Source::Element: Lerp {
    type Element = Source::Element;

    fn get(&self, index: usize) -> Self::Element {
        let index = math::wrap((index as f64) - self.offset, self.size);
        let index = (index.trunc() as usize, index.fract());

        let c1 = self.source.get((index.0 + 0) % self.size);
        let c2 = self.source.get((index.0 + 1) % self.size);

        return Self::Element::lerp(c1, c2, index.1);
    }
}

pub struct RotationNodeDecl<Source, Speed> {
    source: Handle<Source>,
    speed: Speed,
}

pub struct RotationNode<Source, Speed> {
    size: usize,

    source: Handle<Source>,
    speed: Speed,

    offset: f64,
}

impl<Source, Speed, E> NodeDecl for RotationNodeDecl<Source, Speed>
    where Source: Node<Element=E>,
          Speed: UnboundValueDecl<f64>,
          E: Lerp {
    type Element = E;
    type Target = RotationNode<Source, Speed::Value>;

    fn new(self, size: usize) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            size,
            source: self.source,
            speed: self.speed.new()?,
            offset: 0.0,
        });
    }
}

impl<Source, Speed> Dynamic for RotationNode<Source, Speed>
    where Speed: Value<f64> {
    fn update(&mut self, duration: &Duration) {
        self.speed.update(duration);
        self.offset += self.speed.get() * duration.as_secs_f64();
    }
}

impl<'a, Source, Speed> RenderType<'a> for RotationNode<Source, Speed>
    where Source: RenderType<'a>,
          Source::Element: Lerp {
    type Element = Source::Element;
    type Render = RotationRenderer<Source::Render>;
}

impl<Source, Speed, E> Node for RotationNode<Source, Speed>
    where Source: Node<Element=E>,
          Speed: Value<f64>,
          E: Lerp {
    fn render<'a>(&'a self, renderer: &'a Renderer) -> <Self as RenderType<'a>>::Render {
        return RotationRenderer {
            source: renderer.render(&self.source),
            size: self.size,
            offset: self.offset,
        };
    }
}

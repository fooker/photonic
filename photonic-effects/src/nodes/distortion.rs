use std::time::Duration;

use failure::Error;

use photonic_core::core::*;
use photonic_core::math::Lerp;
use photonic_core::value::*;

struct Distortion<'a, F, E>
    where F: Fn(&E, f64) -> E {
    source: Box<Render<Element=E> + 'a>,
    distortion: &'a F,
    value: f64,
    time: f64,
}

impl<'a, F, E> Render for Distortion<'a, F, E>
    where F: Fn(&E, f64) -> E,
          E: Lerp {
    type Element = E;

    fn get(&self, index: usize) -> Self::Element {
        let c1: E = self.source.get(index);
        let c2: E = (self.distortion)(&c1, self.time);
        return E::lerp(c1, c2, self.value);
    }
}

pub struct DistortionNodeDecl<Source, F> {
    pub source: Handle<Source>,
    pub value: Box<BoundValueDecl<f64>>,
    pub distortion: F,
}

pub struct DistortionNode<Source, F> {
    source: Handle<Source>,
    value: Box<Value<f64>>,
    distortion: F,

    time: f64,
}

impl<Source, F, E> NodeDecl for DistortionNodeDecl<Source, F>
    where Source: Node<Element=E>,
          F: Fn(&E, f64) -> E,
          E: Lerp + 'static {
    type Element = E;
    type Target = DistortionNode<Source, F>;

    fn new(self, _size: usize) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            source: self.source,
            value: self.value.new(Bounds::norm())?,
            distortion: self.distortion,
            time: 0.0,
        });
    }
}

impl<Source, F> Dynamic for DistortionNode<Source, F> {
    fn update(&mut self, duration: &Duration) {
        self.value.update(duration);

        self.time += duration.as_secs_f64();
    }
}

impl<Source, F, E> Node for DistortionNode<Source, F>
    where Source: Node<Element=E>,
          F: Fn(&E, f64) -> E,
          E: Lerp + 'static {
    type Element = E;

    fn render<'a>(&'a self, renderer: &'a Renderer) -> Box<Render<Element=Self::Element> + 'a> {
        return Box::new(Distortion {
            source: renderer.render(&self.source),
            distortion: &self.distortion,
            value: self.value.get(),
            time: self.time,
        });
    }
}

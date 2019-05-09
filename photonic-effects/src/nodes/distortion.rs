use std::time::Duration;

use failure::Error;

use photonic_core::core::*;
use photonic_core::math::Lerp;
use photonic_core::value::*;

pub struct DistortionRenderer<'a, Source, F>
    where Source: Render,
          F: Fn(&Source::Element, f64) -> Source::Element {
    source: Source,
    distortion: &'a F,
    value: f64,
    time: f64,
}

impl<'a, Source, F> Render for DistortionRenderer<'a, Source, F>
    where Source: Render,
          Source::Element: Lerp,
          F: Fn(&Source::Element, f64) -> Source::Element {
    type Element = Source::Element;

    fn get(&self, index: usize) -> Self::Element {
        let c1 = self.source.get(index);
        let c2 = (self.distortion)(&c1, self.time);
        return Self::Element::lerp(c1, c2, self.value);
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
          E: Lerp,
          F: Fn(&E, f64) -> E + 'static {
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

impl<'a, Source, F> RenderType<'a> for DistortionNode<Source, F>
    where Source: RenderType<'a>,
          Source::Element: Lerp,
          F: Fn(&Source::Element, f64) -> Source::Element + 'a {
    type Element = Source::Element;
    type Render = DistortionRenderer<'a, Source::Render, F>;
}

impl<Source, E, F> Node for DistortionNode<Source, F>
    where Source: Node<Element=E>,
          E: Lerp,
          F: Fn(&E, f64) -> E + 'static {
    fn render<'a>(&'a self, renderer: &'a Renderer) -> <Self as RenderType<'a>>::Render {
        return DistortionRenderer {
            source: renderer.render(&self.source),
            distortion: &self.distortion,
            value: self.value.get(),
            time: self.time,
        };
    }
}

use std::time::Duration;

use failure::Error;

use photonic_core::color::Black;
use photonic_core::core::*;
use photonic_core::math::Lerp;
use photonic_core::value::*;

struct PartialBlackoutRenderer<'a, E>
    where E: Lerp + Black {
    source: Box<Render<Element=E> + 'a>,

    range: (usize, usize),
    value: f64,
}

impl<'a, E> Render for PartialBlackoutRenderer<'a, E>
    where E: Lerp + Black {
    type Element = E;

    fn get(&self, index: usize) -> Self::Element {
        let value = self.source.get(index);

        if self.range.0 < index && index < self.range.1 {
            return E::lerp(value,
                           E::black(),
                           self.value);
        } else {
            return value;
        }
    }
}

struct FullBlackoutRenderer<'a, E>
    where E: Lerp + Black {
    source: Box<Render<Element=E> + 'a>,

    value: f64,
}

impl<'a, E> Render for FullBlackoutRenderer<'a, E>
    where E: Lerp + Black {
    type Element = E;
    fn get(&self, index: usize) -> E {
        return E::lerp(self.source.get(index),
                       E::black(),
                       self.value);
    }
}

pub struct BlackoutNodeDecl<Source> {
    pub source: Handle<Source>,
    pub value: Box<BoundValueDecl<f64>>,
    pub range: Option<(usize, usize)>,
}

pub struct BlackoutNode<Source> {
    source: Handle<Source>,

    value: Box<Value<f64>>,

    range: Option<(usize, usize)>,
}

impl<Source, E> NodeDecl for BlackoutNodeDecl<Source>
    where Source: Node<Element=E>,
          E: Lerp + Black + 'static {
    type Element = E;
    type Target = BlackoutNode<Source>;

    fn new(self, _size: usize) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            source: self.source,
            value: self.value.new(Bounds::norm())?,
            range: self.range,
        });
    }
}

impl<Source> Dynamic for BlackoutNode<Source> {
    fn update(&mut self, duration: &Duration) {
        self.value.update(duration);
    }
}

impl<Source, E> Node for BlackoutNode<Source>
    where Source: Node<Element=E>,
          E: Lerp + Black + 'static {
    type Element = E;

    fn render<'a>(&'a self, renderer: &'a Renderer) -> Box<Render<Element=Self::Element> + 'a> {
        let source = renderer.render(&self.source);
        let value = self.value.get();

        if let Some(range) = self.range {
            return Box::new(PartialBlackoutRenderer {
                source,
                value,
                range,
            });
        } else {
            return Box::new(FullBlackoutRenderer {
                source,
                value,
            });
        }
    }
}

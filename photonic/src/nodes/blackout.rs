use std::time::Duration;

use failure::Error;

use crate::color::Black;
use crate::core::*;
use crate::math::Lerp;
use crate::values::*;

struct PartialBlackoutRenderer<'a> {
    source: Box<Render + 'a>,

    range: (usize, usize),
    value: f64,
}

impl<'a> Render for PartialBlackoutRenderer<'a> {
    fn get(&self, index: usize) -> MainColor {
        let value = self.source.get(index);

        if self.range.0 < index && index < self.range.1 {
            return MainColor::lerp(value,
                                   MainColor::black(),
                                   self.value);
        } else {
            return value;
        }
    }
}

struct FullBlackoutRenderer<'a> {
    source: Box<Render + 'a>,

    value: f64,
}

impl<'a> Render for FullBlackoutRenderer<'a> {
    fn get(&self, index: usize) -> MainColor {
        return MainColor::lerp(self.source.get(index),
                               MainColor::black(),
                               self.value);
    }
}

pub struct BlackoutNodeDecl<Source: Node> {
    pub source: Handle<Source>,
    pub value: Box<BoundValueDecl<f64>>,
    pub range: Option<(usize, usize)>,
}

pub struct BlackoutNode<Source: Node> {
    source: Handle<Source>,

    value: Box<Value<f64>>,

    range: Option<(usize, usize)>,
}

impl<Source: Node> NodeDecl for BlackoutNodeDecl<Source> {
    type Target = BlackoutNode<Source>;

    fn new(self, _size: usize) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            source: self.source,
            value: self.value.new(Bounds::norm())?,
            range: self.range,
        });
    }
}

impl<Source: Node> Node for BlackoutNode<Source> {
    fn update(&mut self, duration: &Duration) {
        self.value.update(duration);
    }

    fn render<'a>(&'a self, renderer: &'a Renderer) -> Box<Render + 'a> {
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

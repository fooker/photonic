use color::Black;
use core::*;
use core::Renderer;
use core::Source;
use math::Lerp;
use std::time::Duration;

struct PartialBlackoutRenderer<'a> {
    source: Box<Renderer + 'a>,

    range: (usize, usize),
    value: f64,
}

impl<'a> Renderer for PartialBlackoutRenderer<'a> {
    fn size(&self) -> usize {
        self.source.size()
    }

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
    source: Box<Renderer + 'a>,

    value: f64,
}

impl<'a> Renderer for FullBlackoutRenderer<'a> {
    fn size(&self) -> usize {
        self.source.size()
    }

    fn get(&self, index: usize) -> MainColor {
        return MainColor::lerp(self.source.get(index),
                               MainColor::black(),
                               self.value);
    }
}

pub struct BlackoutNode {
    source: Box<Node>,
    value: Box<Value>,

    range: Option<(usize, usize)>,
}

impl BlackoutNode {
    pub fn new(source: Box<Node>,
               value: Box<Value>,
               range: Option<(usize, usize)>) -> Self {
        Self {
            source,
            range,
            value,
        }
    }
}

impl Node for BlackoutNode {}

impl Source for BlackoutNode {
    fn render<'a>(&'a self) -> Box<Renderer + 'a> {
        if let Some(range) = self.range {
            return Box::new(PartialBlackoutRenderer {
                source: self.source.render(),
                value: self.value.get(),
                range,
            });
        } else {
            return Box::new(FullBlackoutRenderer {
                source: self.source.render(),
                value: self.value.get(),
            });
        }
    }
}

impl Dynamic for BlackoutNode {
    fn update(&mut self, duration: Duration) {
        self.source.update(duration);
        self.value.update(duration);
    }
}


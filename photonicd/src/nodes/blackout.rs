use photonic::color::Black;
use photonic::core::*;
use photonic::math::Lerp;
use photonic::values::*;
use std::time::Duration;

struct PartialBlackoutRenderer<'a> {
    source: Box<Renderer + 'a>,

    range: (usize, usize),
    value: f64,
}

impl<'a> Renderer for PartialBlackoutRenderer<'a> {
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
    fn get(&self, index: usize) -> MainColor {
        return MainColor::lerp(self.source.get(index),
                               MainColor::black(),
                               self.value);
    }
}

#[derive(Inspection)]
pub struct BlackoutNode {
    #[node()] source: Box<Node>,

    #[value()] value: FloatValue,

    range: Option<(usize, usize)>,
}

impl BlackoutNode {
    const CLASS: &'static str = "blackout";

    pub fn new(source: Box<Node>,
               range: Option<(usize, usize)>,
               value: FloatValueFactory) -> Result<Self, String> {
        Ok(Self {
            source,
            range,
            value: value(FloatValueDecl { name: "value", min: Some(0.0), max: Some(1.0)})?,
        })
    }
}

impl Node for BlackoutNode {
    fn class(&self) -> &'static str {
        Self::CLASS
    }
}

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
    fn update(&mut self, duration: &Duration) {
        self.source.update(duration);
        self.value.update(duration);
    }
}


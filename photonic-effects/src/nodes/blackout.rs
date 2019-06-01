use std::time::Duration;

use failure::Error;

use photonic_core::color::Black;
use photonic_core::core::*;
use photonic_core::math::Lerp;
use photonic_core::value::*;

pub struct BlackoutRenderer<Source> {
    source: Source,

    range: (usize, usize),
    value: f64,
}

impl<Source> Render for BlackoutRenderer<Source>
    where Source: Render,
          Source::Element: Lerp + Black {
    type Element = Source::Element;

    fn get(&self, index: usize) -> Self::Element {
        let value = self.source.get(index);

        if self.range.0 < index && index < self.range.1 {
            return Self::Element::lerp(value,
                                       Self::Element::black(),
                                       self.value);
        } else {
            return value;
        }
    }
}

pub struct BlackoutNodeDecl<Source, Value> {
    pub source: Handle<Source>,
    pub value: Value,
    pub range: Option<(usize, usize)>,
}

pub struct BlackoutNode<Source, Value> {
    source: Handle<Source>,
    value: Value,
    range: (usize, usize),
}

impl<Source, Value, E> NodeDecl for BlackoutNodeDecl<Source, Value>
    where Source: Node<Element=E>,
          Value: BoundValueDecl<f64>,
          E: Lerp + Black {
    type Element = E;
    type Target = BlackoutNode<Source, Value::Value>;

    fn new(self, size: usize) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            source: self.source,
            value: self.value.new(Bounds::norm())?,
            range: self.range.unwrap_or((0, size)),
        });
    }
}

impl<Source, Value> Dynamic for BlackoutNode<Source, Value>
    where Value: self::Value<f64> {
    fn update(&mut self, duration: &Duration) {
        self.value.update(duration);
    }
}

impl<'a, Source, Value> RenderType<'a> for BlackoutNode<Source, Value>
    where Source: RenderType<'a>,
          Source::Element: Lerp + Black {
    type Element = Source::Element;
    type Render = BlackoutRenderer<Source::Render>;
}

impl<Source, Value, E> Node for BlackoutNode<Source, Value>
    where Source: Node<Element=E>,
          Value: self::Value<f64>,
          E: Lerp + Black {
    fn render<'a>(&'a self, renderer: &'a Renderer) -> <Self as RenderType<'a>>::Render {
        return BlackoutRenderer {
            source: renderer.render(&self.source),
            value: self.value.get(),
            range: self.range,
        };
    }
}

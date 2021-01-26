use std::time::Duration;

use failure::Error;

use photonic_core::color::Black;
use photonic_core::core::*;
use photonic_core::math::Lerp;
use photonic_core::attr::*;

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

pub struct BlackoutNodeDecl<Source, Value>
    where Source: NodeDecl {
    pub source: NodeRef<Source>,
    pub value: Value,
    pub range: Option<(usize, usize)>,
}

pub struct BlackoutNode<Source, Value> {
    source: NodeHandle<Source>,
    value: Value,
    range: (usize, usize),
}

impl<Source, Value, E> NodeDecl for BlackoutNodeDecl<Source, Value>
    where Source: NodeDecl<Element=E>,
          Value: BoundAttrDecl<f64>,
          E: Lerp + Black {
    type Element = E;
    type Target = BlackoutNode<Source::Target, Value::Target>;

    fn materialize(self, size: usize, builder: &mut SceneBuilder) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            source: builder.node("source", self.source)?,
            value: builder.bound_attr("value", self.value, Bounds::normal())?,
            range: self.range.unwrap_or((0, size)),
        });
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
          Value: self::Attr<f64>,
          E: Lerp + Black {
    const TYPE: &'static str = "blackout";

    fn update(&mut self, duration: &Duration) {
        self.value.update(duration);
    }

    fn render<'a>(&'a self, renderer: &'a Renderer) -> <Self as RenderType<'a>>::Render {
        return BlackoutRenderer {
            source: renderer.render(&self.source),
            value: self.value.get(),
            range: self.range,
        };
    }
}

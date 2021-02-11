use std::time::Duration;

use failure::Error;

use photonic_core::color::Black;
use photonic_core::scene::{NodeBuilder, NodeHandle};
use photonic_core::math::Lerp;
use photonic_core::attr::{BoundAttrDecl, UnboundAttrDecl, Attr, Bounds};
use photonic_core::node::{RenderType, Node, NodeDecl, Render};

pub struct BrightnessRenderer<Source> {
    source: Source,

    range: (usize, usize),
    brightness: f64,
}

impl<Source> Render for BrightnessRenderer<Source>
    where Source: Render,
          Source::Element: Lerp + Black {
    type Element = Source::Element;

    fn get(&self, index: usize) -> Self::Element {
        let value = self.source.get(index);

        if self.range.0 <= index && index <= self.range.1 {
            return Self::Element::lerp(Self::Element::black(), value, self.brightness);
        } else {
            return value;
        }
    }
}

pub struct BrightnessNodeDecl<Source, Brightness>
    where Source: NodeDecl,
          Brightness: BoundAttrDecl<f64> {
    pub source: NodeHandle<Source>,
    pub brightness: Brightness,
    pub range: Option<(usize, usize)>,
}

pub struct BrightnessNode<Source, Brightness> {
    source: Source,
    brightness: Brightness,
    range: (usize, usize),
}

impl<Source, Brightness, E> NodeDecl for BrightnessNodeDecl<Source, Brightness>
    where Source: NodeDecl<Element=E>,
          Brightness: BoundAttrDecl<f64>,
          E: Lerp + Black {
    type Element = E;
    type Target = BrightnessNode<Source::Target, Brightness::Target>;

    fn materialize(self, size: usize, builder: &mut NodeBuilder) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            source: builder.node("source", self.source)?,
            brightness: builder.bound_attr("brightness", self.brightness, Bounds::normal())?,
            range: self.range.unwrap_or((0, size - 1)),
        });
    }
}

impl<'a, Source, Brightness> RenderType<'a> for BrightnessNode<Source, Brightness>
    where Source: RenderType<'a>,
          Source::Element: Lerp + Black {
    type Element = Source::Element;
    type Render = BrightnessRenderer<Source::Render>;
}

impl<Source, Brightness, E> Node for BrightnessNode<Source, Brightness>
    where Source: Node<Element=E>,
          Brightness: self::Attr<f64>,
          E: Lerp + Black {
    const KIND: &'static str = "brightness";

    fn update(&mut self, duration: &Duration) {
        self.brightness.update(duration);
    }

    fn render(&mut self) -> <Self as RenderType>::Render {
        return BrightnessRenderer {
            source: self.source.render(),
            brightness: self.brightness.get(),
            range: self.range,
        };
    }
}

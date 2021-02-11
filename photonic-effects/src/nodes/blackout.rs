use std::time::Duration;

use failure::Error;

use photonic_core::color::Black;
use photonic_core::scene::{NodeBuilder, NodeHandle};
use photonic_core::math::Lerp;
use photonic_core::attr::{UnboundAttrDecl, Attr};
use photonic_core::node::{RenderType, Node, NodeDecl, Render};

pub struct BlackoutRenderer<Source> {
    source: Source,

    range: (usize, usize),
    active: bool,
}

impl<Source> Render for BlackoutRenderer<Source>
    where Source: Render,
          Source::Element: Lerp + Black {
    type Element = Source::Element;

    fn get(&self, index: usize) -> Self::Element {
        return if self.range.0 <= index && index <= self.range.1 && self.active {
            Self::Element::black()
        } else {
            self.source.get(index)
        }
    }
}

pub struct BlackoutNodeDecl<Source, Active>
    where Source: NodeDecl,
          Active: UnboundAttrDecl<bool> {
    pub source: NodeHandle<Source>,
    pub active: Active,
    pub range: Option<(usize, usize)>,
}

pub struct BlackoutNode<Source, Active> {
    source: Source,
    active: Active,
    range: (usize, usize),
}

impl<Source, Active, E> NodeDecl for BlackoutNodeDecl<Source, Active>
    where Source: NodeDecl<Element=E>,
          Active: UnboundAttrDecl<bool>,
          E: Lerp + Black {
    type Element = E;
    type Target = BlackoutNode<Source::Target, Active::Target>;

    fn materialize(self, size: usize, builder: &mut NodeBuilder) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            source: builder.node("source", self.source)?,
            active: builder.unbound_attr("active", self.active)?,
            range: self.range.unwrap_or((0, size - 1)),
        });
    }
}

impl<'a, Source, Active> RenderType<'a> for BlackoutNode<Source, Active>
    where Source: RenderType<'a>,
          Source::Element: Lerp + Black {
    type Element = Source::Element;
    type Render = BlackoutRenderer<Source::Render>;
}

impl<Source, Active, E> Node for BlackoutNode<Source, Active>
    where Source: Node<Element=E>,
          Active: self::Attr<bool>,
          E: Lerp + Black {
    const KIND: &'static str = "blackout";

    fn update(&mut self, duration: &Duration) {
        self.source.update(duration);

        self.active.update(duration);
    }

    fn render(&mut self) -> <Self as RenderType>::Render {
        return BlackoutRenderer {
            source: self.source.render(),
            active: self.active.get(),
            range: self.range,
        };
    }
}

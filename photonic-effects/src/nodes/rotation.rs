use std::time::Duration;

use failure::Error;

use photonic_core::scene::{NodeBuilder, NodeHandle};
use photonic_core::math;
use photonic_core::math::Lerp;
use photonic_core::attr::{UnboundAttrDecl, Attr};
use photonic_core::node::{RenderType, Node, NodeDecl, Render};

pub struct RotationRenderer<Source> {
    source: Source,
    size: usize,
    offset: f64,
}

impl<Source> Render for RotationRenderer<Source>
    where Source: Render,
          Source::Element: Lerp {
    type Element = Source::Element;

    fn get(&self, index: usize) -> Self::Element {
        let index = math::wrap((index as f64) - self.offset, self.size);
        let index = (index.trunc() as usize, index.fract());

        let c1 = self.source.get((index.0 + 0) % self.size);
        let c2 = self.source.get((index.0 + 1) % self.size);

        return Self::Element::lerp(c1, c2, index.1);
    }
}

pub struct RotationNodeDecl<Source, Speed>
    where Source: NodeDecl {
    source: NodeHandle<Source>,
    speed: Speed,
}

pub struct RotationNode<Source, Speed> {
    size: usize,

    source: Source,
    speed: Speed,

    offset: f64,
}

impl<Source, Speed, E> NodeDecl for RotationNodeDecl<Source, Speed>
    where Source: NodeDecl<Element=E>,
          Speed: UnboundAttrDecl<f64>,
          E: Lerp {
    type Element = E;
    type Target = RotationNode<Source::Target, Speed::Target>;

    fn materialize(self, size: usize, builder: &mut NodeBuilder) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            size,
            source: builder.node("source", self.source)?,
            speed: builder.unbound_attr("speed", self.speed)?,
            offset: 0.0,
        });
    }
}

impl<'a, Source, Speed> RenderType<'a> for RotationNode<Source, Speed>
    where Source: RenderType<'a>,
          Source::Element: Lerp {
    type Element = Source::Element;
    type Render = RotationRenderer<Source::Render>;
}

impl<Source, Speed, E> Node for RotationNode<Source, Speed>
    where Source: Node<Element=E>,
          Speed: Attr<f64>,
          E: Lerp {
    const KIND: &'static str = "rotation";

    fn update(&mut self, duration: Duration) {
        self.source.update(duration);

        self.offset += self.speed.update(duration).value() * duration.as_secs_f64();
    }

    fn render(&mut self) -> <Self as RenderType>::Render {
        return RotationRenderer {
            source: self.source.render(),
            size: self.size,
            offset: self.offset,
        };
    }
}

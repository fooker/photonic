use std::time::Duration;

use failure::Error;

use photonic_core::attr::{Attr, BoundAttrDecl, Bounds};
use photonic_core::buffer::Buffer;
use photonic_core::color::{Black, Shade};
use photonic_core::node::{Node, NodeDecl, RenderType, Render};
use photonic_core::scene::{NodeBuilder, NodeHandle};

pub struct AfterglowNodeDecl<Source, Decay>
    where Source: NodeDecl,
          Decay: BoundAttrDecl<f64> {
    pub source: NodeHandle<Source>,
    pub decay: Decay,
}

pub struct AfterglowNode<Source, Decay, E>
    where Source: Node<Element=E> {
    source: Source,
    decay: Decay,

    buffer: Buffer<E>,
}

impl<Source, Decay> NodeDecl for AfterglowNodeDecl<Source, Decay>
    where Source: NodeDecl,
          Decay: BoundAttrDecl<f64>,
          Source::Element: Black + Shade + Ord + Copy + 'static {
    type Element = Source::Element;
    type Target = AfterglowNode<Source::Target, Decay::Target, Source::Element>;

    fn materialize(self, size: usize, builder: &mut NodeBuilder) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            source: builder.node("source", self.source)?,
            decay: builder.bound_attr("decay", self.decay, Bounds::normal())?,
            buffer: Buffer::black(size),
        });
    }
}

impl<'a, Source, Decay, E> RenderType<'a> for AfterglowNode<Source, Decay, E>
    where Source: Node<Element=E>,
          E: Black + Shade + Ord + Copy + 'static{
    type Element = E;
    type Render = &'a Buffer<E>;
}

impl<Source, Decay, E> Node for AfterglowNode<Source, Decay, E>
    where Source: Node<Element=E>,
          Decay: self::Attr<f64>,
          E: Black + Shade + Ord + Copy + 'static {
    const KIND: &'static str = "afterglow";

    fn update(&mut self, duration: &Duration) {
        self.source.update(duration);

        let decay = self.decay.update(duration).value() * duration.as_secs_f64();

        self.buffer.update(|_, e| {
            e.darken(decay)
        });
    }

    fn render(&mut self) -> <Self as RenderType>::Render {
        let source = self.source.render();

        self.buffer.update(|i, e| {
            std::cmp::max(*e, source.get(i))
        });

        return &self.buffer;
    }
}

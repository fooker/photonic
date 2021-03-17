use std::time::Duration;

use anyhow::Error;

use photonic_core::attr::{Attr, BoundAttrDecl, Bounds};
use photonic_core::buffer::Buffer;
use photonic_core::color::{Black, Shade, ComponentWise};
use photonic_core::node::{Node, NodeDecl, RenderType, Render};
use photonic_core::scene::{NodeBuilder, NodeHandle};

pub struct AfterglowNodeDecl<Source, Decay>
    where Source: NodeDecl,
          Decay: BoundAttrDecl<f64> {
    pub source: NodeHandle<Source>,
    pub decay: Decay,
}

pub struct AfterglowNode<Source, Decay>
    where Source: Node {
    source: Source,
    decay: Decay,

    buffer: Buffer<Source::Element>,
}

impl<Source, Decay> NodeDecl for AfterglowNodeDecl<Source, Decay>
    where Source: NodeDecl,
          Decay: BoundAttrDecl<f64>,
          Source::Element: Black + Shade + ComponentWise + Copy + 'static {
    type Element = Source::Element;
    type Target = AfterglowNode<Source::Target, Decay::Target>;

    fn materialize(self, size: usize, builder: &mut NodeBuilder) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            source: builder.node("source", self.source)?,
            decay: builder.bound_attr("decay", self.decay, Bounds::normal())?,
            buffer: Buffer::black(size),
        });
    }
}

impl<'a, Source, Decay> RenderType<'a, Self> for AfterglowNode<Source, Decay>
    where Source: Node,
          Decay: self::Attr<f64>,
          Source::Element: Black + Shade + ComponentWise + Copy + 'static{
    type Render = &'a Buffer<Source::Element>;
}

impl<Source, Decay> Node for AfterglowNode<Source, Decay>
    where Source: Node,
          Decay: self::Attr<f64>,
          Source::Element: Black + Shade + ComponentWise + Copy + 'static {
    const KIND: &'static str = "afterglow";

    type Element = Source::Element;

    fn update(&mut self, duration: Duration) {
        self.source.update(duration);

        let decay = self.decay.update(duration).value() * duration.as_secs_f64();

        self.buffer.update(|_, e| {
            e.darken(decay)
        });
    }

    fn render(&mut self) -> <Self as RenderType<Self>>::Render {
        let source = self.source.render();

        self.buffer.update(|i, e| {
            return source.get(i)
                .component_wise(e, |a, b| { f64::max(a, b) });
        });

        return &self.buffer;
    }
}

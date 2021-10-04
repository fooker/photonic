use std::time::Duration;

use anyhow::Result;

use photonic_core::attr::{Attr, BoundAttrDecl, Bounds};
use photonic_core::buffer::Buffer;
use photonic_core::color::{Black, palette::{ComponentWise, Shade}};
use photonic_core::node::{Node, NodeDecl, Render, RenderType};
use photonic_core::scene::{NodeBuilder, NodeHandle};

pub struct AfterglowNodeDecl<Source, Decay>
where
    Source: NodeDecl,
    Decay: BoundAttrDecl<Element=f64>,
{
    pub source: NodeHandle<Source>,
    pub decay: Decay,
}

pub struct AfterglowNode<Source, Decay>
where
    Source: Node,
{
    source: Source,
    decay: Decay,

    buffer: Buffer<Source::Element>,
}

impl<Source, Decay> NodeDecl for AfterglowNodeDecl<Source, Decay>
where
    Source: NodeDecl,
    Decay: BoundAttrDecl<Element=f64>,
    Source::Element: Black + Shade<Scalar=f64> + ComponentWise<Scalar=f64> + Copy + 'static,
{
    type Element = Source::Element;
    type Target = AfterglowNode<Source::Target, Decay::Target>;

    fn materialize(self, size: usize, builder: &mut NodeBuilder) -> Result<Self::Target> {
        return Ok(Self::Target {
            source: builder.node("source", self.source)?,
            decay: builder.bound_attr("decay", self.decay, Bounds::normal())?,
            buffer: Buffer::black(size),
        });
    }
}

impl<'a, Source, Decay> RenderType<'a, Self> for AfterglowNode<Source, Decay>
where
    Source: Node,
    Decay: self::Attr<Element=f64>,
    Source::Element: Black + Shade<Scalar=f64> + ComponentWise<Scalar=f64> + Copy + 'static,
{
    type Render = &'a Buffer<Source::Element>;
}

impl<Source, Decay> Node for AfterglowNode<Source, Decay>
where
    Source: Node,
    Decay: self::Attr<Element=f64>,
    Source::Element: Black + Shade<Scalar=f64> + ComponentWise<Scalar=f64> + Copy + 'static,
{
    const KIND: &'static str = "afterglow";

    type Element = Source::Element;

    fn update(&mut self, duration: Duration) -> Result<()> {
        self.source.update(duration)?;

        let decay = self.decay.update(duration).value() * duration.as_secs_f64();
        self.buffer.update(|_, e| e.darken(decay));

        return Ok(());
    }

    fn render(&mut self) -> Result<<Self as RenderType<Self>>::Render> {
        let source = self.source.render()?;

        for (i, e) in self.buffer.iter_mut().enumerate() {
            *e = source.get(i)?.component_wise(e, f64::max);
        }

        return Ok(&self.buffer);
    }
}

#[cfg(feature = "dyn")]
pub mod model {
    use photonic_dyn::config;
    use photonic_dyn::model::NodeModel;
    use photonic_dyn::builder::NodeBuilder;
    use photonic_core::boxed::{BoxedNodeDecl, Wrap};
    use photonic_core::color;

    use anyhow::Result;
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct AfterglowConfig {
        pub source: config::Node,
        pub decay: config::Attr,
    }

    impl NodeModel for AfterglowConfig {
        fn assemble(self, builder: &mut impl NodeBuilder) -> Result<BoxedNodeDecl<color::RGBColor>> {
            return Ok(BoxedNodeDecl::wrap(
                super::AfterglowNodeDecl {
                    source: builder.node("source", self.source)?,
                    decay: builder.bound_attr("decay", self.decay)?,
                },
            ));
        }
    }
}

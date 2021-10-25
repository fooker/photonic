use std::time::Duration;

use anyhow::Result;

use photonic_core::attr::{Attr, UnboundAttrDecl};
use photonic_core::math;
use photonic_core::math::Lerp;
use photonic_core::node::{Node, NodeDecl, Render, RenderType};
use photonic_core::scene::{NodeBuilder, NodeHandle};

pub struct RotationRenderer<Source> {
    source: Source,
    size: usize,
    offset: f64,
}

impl<Source> Render for RotationRenderer<Source>
where
    Source: Render,
    Source::Element: Lerp,
{
    type Element = Source::Element;

    fn get(&self, index: usize) -> Result<Self::Element> {
        let index = math::wrap((index as f64) - self.offset, self.size);
        let index = (index.trunc() as usize, index.fract());

        let c1 = self.source.get(index.0)?;
        let c2 = self.source.get((index.0 + 1) % self.size)?;

        return Ok(Self::Element::lerp(c1, c2, index.1));
    }
}

pub struct RotationNodeDecl<Source, Speed>
where
    Source: NodeDecl,
{
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
where
    Source: NodeDecl<Element = E>,
    Speed: UnboundAttrDecl<Element = f64>,
    E: Lerp,
{
    type Element = E;
    type Target = RotationNode<Source::Target, Speed::Target>;

    fn materialize(self, size: usize, builder: &mut NodeBuilder) -> Result<Self::Target> {
        return Ok(Self::Target {
            size,
            source: builder.node("source", self.source)?,
            speed: builder.unbound_attr("speed", self.speed)?,
            offset: 0.0,
        });
    }
}

impl<'a, Source, Speed> RenderType<'a, Self> for RotationNode<Source, Speed>
where
    Source: Node,
    Speed: Attr<Element = f64>,
    Source::Element: Lerp,
{
    type Render = RotationRenderer<<Source as RenderType<'a, Source>>::Render>;
}

impl<Source, Speed> Node for RotationNode<Source, Speed>
where
    Source: Node,
    Speed: Attr<Element = f64>,
    Source::Element: Lerp,
{
    const KIND: &'static str = "rotation";

    type Element = Source::Element;

    fn update(&mut self, duration: Duration) -> Result<()> {
        self.source.update(duration)?;

        self.offset += self.speed.update(duration).value() * duration.as_secs_f64();

        return Ok(());
    }

    fn render(&self) -> Result<<Self as RenderType<Self>>::Render> {
        return Ok(RotationRenderer {
            source: self.source.render()?,
            size: self.size,
            offset: self.offset,
        });
    }
}

#[cfg(feature = "dyn")]
pub mod model {
    use photonic_core::boxed::{BoxedNodeDecl, Wrap};
    use photonic_core::color;
    use photonic_dyn::builder::NodeBuilder;
    use photonic_dyn::config;
    use photonic_dyn::model::NodeModel;

    use anyhow::Result;
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct RotationConfig {
        pub source: config::Node,
        pub speed: config::Attr,
    }

    impl NodeModel for RotationConfig {
        fn assemble(
            self,
            builder: &mut impl NodeBuilder,
        ) -> Result<BoxedNodeDecl<color::RGBColor>> {
            return Ok(BoxedNodeDecl::wrap(super::RotationNodeDecl {
                source: builder.node("source", self.source)?,
                speed: builder.unbound_attr("speed", self.speed)?,
            }));
        }
    }
}

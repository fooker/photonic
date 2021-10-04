use std::time::Duration;

use anyhow::Result;

use photonic_core::attr::{Attr, UnboundAttrDecl};
use photonic_core::color::Black;
use photonic_core::math::Lerp;
use photonic_core::node::{Node, NodeDecl, Render, RenderType};
use photonic_core::scene::{NodeBuilder, NodeHandle};

pub struct BlackoutRenderer<Source> {
    source: Source,

    range: (usize, usize),
    active: bool,
}

impl<Source> Render for BlackoutRenderer<Source>
where
    Source: Render,
    Source::Element: Lerp + Black,
{
    type Element = Source::Element;

    fn get(&self, index: usize) -> Result<Self::Element> {
        return if self.range.0 <= index && index <= self.range.1 && self.active {
            Ok(Self::Element::black())
        } else {
            self.source.get(index)
        };
    }
}

pub struct BlackoutNodeDecl<Source, Active>
where
    Source: NodeDecl,
    Active: UnboundAttrDecl<Element=bool>,
{
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
where
    Source: NodeDecl<Element = E>,
    Active: UnboundAttrDecl<Element=bool>,
    E: Lerp + Black,
{
    type Element = E;
    type Target = BlackoutNode<Source::Target, Active::Target>;

    fn materialize(self, size: usize, builder: &mut NodeBuilder) -> Result<Self::Target> {
        return Ok(Self::Target {
            source: builder.node("source", self.source)?,
            active: builder.unbound_attr("active", self.active)?,
            range: self.range.unwrap_or((0, size - 1)),
        });
    }
}

impl<'a, Source, Active> RenderType<'a, Self> for BlackoutNode<Source, Active>
where
    Source: Node,
    Active: self::Attr<Element=bool>,
    Source::Element: Lerp + Black,
{
    type Render = BlackoutRenderer<<Source as RenderType<'a, Source>>::Render>;
}

impl<Source, Active> Node for BlackoutNode<Source, Active>
where
    Source: Node,
    Active: self::Attr<Element=bool>,
    Source::Element: Lerp + Black,
{
    const KIND: &'static str = "blackout";

    type Element = Source::Element;

    fn update(&mut self, duration: Duration) -> Result<()> {
        self.source.update(duration)?;

        self.active.update(duration);

        return Ok(());
    }

    fn render(&mut self) -> Result<<Self as RenderType<Self>>::Render> {
        return Ok(BlackoutRenderer {
            source: self.source.render()?,
            active: self.active.get(),
            range: self.range,
        });
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
    pub struct BlackoutConfig {
        pub source: config::Node,
        pub active: config::Attr,
        pub range: Option<(usize, usize)>,
    }

    impl NodeModel for BlackoutConfig {
        fn assemble(self, builder: &mut impl NodeBuilder) -> Result<BoxedNodeDecl<color::RGBColor>> {
            return Ok(BoxedNodeDecl::wrap(
                super::BlackoutNodeDecl {
                    source: builder.node("source", self.source)?,
                    active: builder.unbound_attr("active", self.active)?,
                    range: self.range,
                },
            ));
        }
    }
}

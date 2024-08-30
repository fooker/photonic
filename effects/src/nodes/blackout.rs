use std::ops::Range;

use anyhow::Result;

use photonic::{
    Attr, Buffer, BufferReader, FreeAttrDecl, Node, NodeBuilder, NodeDecl, NodeHandle, NodeRef, RenderContext,
};

pub struct Blackout<Source, Active>
where
    Source: NodeDecl + 'static,
    Active: FreeAttrDecl<Value = bool>,
{
    pub source: NodeHandle<Source>,
    pub active: Active,

    pub value: <<Source as NodeDecl>::Node as Node>::Element,
    pub range: Option<Range<usize>>,
}

pub struct BlackoutNode<Source, Active>
where
    Source: Node + 'static,
    Active: Attr<Value = bool>,
{
    source: NodeRef<Source>,
    active: Active,

    value: <Source as Node>::Element,
    range: Range<usize>,
}

impl<Source, Active> NodeDecl for Blackout<Source, Active>
where
    Source: NodeDecl + 'static,
    Active: FreeAttrDecl<Value = bool>,
{
    type Node = BlackoutNode<Source::Node, Active::Attr>;

    async fn materialize(self, builder: &mut NodeBuilder<'_>) -> Result<Self::Node> {
        return Ok(Self::Node {
            source: builder.node("source", self.source).await?,
            active: builder.unbound_attr("active", self.active)?,
            value: self.value,
            range: self.range.unwrap_or(0..builder.size),
        });
    }
}

impl<Source, Active> Node for BlackoutNode<Source, Active>
where
    Source: Node,
    Active: Attr<Value = bool>,
{
    const KIND: &'static str = "blackout";

    type Element = Source::Element;

    fn update(&mut self, ctx: &RenderContext, out: &mut Buffer<Self::Element>) -> Result<()> {
        let source = &ctx[self.source];

        let active = self.active.update(ctx);

        if active {
            out.blit_from(source.map_range(&self.range, |_| self.value.clone()));
        } else {
            out.blit_from(source);
        }

        return Ok(());
    }
}

#[cfg(feature = "dynamic")]
pub mod dynamic {
    use palette::rgb::Rgb;
    use serde::Deserialize;

    use photonic_dynamic::factory::Producible;
    use photonic_dynamic::{config, BoxedFreeAttrDecl, BoxedNodeDecl};

    use super::*;

    #[derive(Deserialize, Debug)]
    pub struct Config {
        pub source: config::Node,
        pub active: config::Attr<bool>,

        pub value: Rgb,
        pub range: Option<Range<usize>>,
    }

    impl Producible for Blackout<BoxedNodeDecl, BoxedFreeAttrDecl<bool>> {
        type Config = Config;
    }

    pub fn node<B>(config: Config, builder: &mut B) -> Result<Blackout<BoxedNodeDecl, BoxedFreeAttrDecl<bool>>>
    where B: photonic_dynamic::NodeBuilder {
        return Ok(Blackout {
            source: builder.node("source", config.source)?,
            active: builder.free_attr("active", config.active)?,
            value: config.value,
            range: config.range,
        });
    }
}

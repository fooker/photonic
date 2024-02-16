use anyhow::Result;
use std::ops::Range;

use photonic::{Attr, Buffer, BufferReader, Context, FreeAttrDecl, Node, NodeBuilder, NodeDecl, NodeHandle, NodeRef};
use photonic_dyn::DynamicNode;

#[derive(DynamicNode)]
pub struct Blackout<Source, Active>
where
    Source: NodeDecl + 'static,
    Active: FreeAttrDecl<Value = bool>,
{
    #[photonic(node)]
    pub source: NodeHandle<Source>,

    #[photonic(attr)]
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

    fn update(&mut self, ctx: &Context, out: &mut Buffer<Self::Element>) -> Result<()> {
        let source = &ctx[self.source];

        let active = self.active.update(ctx.duration);

        if active {
            out.blit_from(source.map_range(&self.range, |_| self.value.clone()));
        } else {
            out.blit_from(source);
        }

        return Ok(());
    }
}

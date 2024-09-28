use anyhow::Result;

use crate::{Buffer, BufferReader, Node, NodeBuilder, NodeDecl, NodeHandle, NodeRef, RenderContext};

pub struct Map<N, F, T, R>
where
    N: NodeDecl,
    N::Node: Node<Element = T> + 'static,
    F: Fn(T) -> R,
    T: Default,
    R: Default + Copy,
{
    pub source: NodeHandle<N>,
    pub mapper: F,
}

impl<N, F, T, R> NodeDecl for Map<N, F, T, R>
where
    N: NodeDecl,
    N::Node: Node<Element = T> + 'static,
    F: Fn(T) -> R,
    T: Default,
    R: Default + Copy,
{
    const KIND: &'static str = "map";

    type Node = MapNode<N::Node, F, T, R>;

    async fn materialize(self, builder: &mut NodeBuilder<'_>) -> Result<Self::Node> {
        return Ok(Self::Node {
            source: builder.node("source", self.source).await?,
            mapper: self.mapper,
        });
    }
}

pub struct MapNode<N, F, T, R>
where
    N: Node<Element = T> + 'static,
    F: Fn(T) -> R,
    R: Default + Copy,
{
    source: NodeRef<N>,
    mapper: F,
}

impl<N, F, T, R> Node for MapNode<N, F, T, R>
where
    N: Node<Element = T> + 'static,
    F: Fn(T) -> R,
    R: Default + Copy,
{
    type Element = R;

    fn update(&mut self, ctx: &RenderContext, out: &mut Buffer<Self::Element>) -> Result<()> {
        let source = &ctx[self.source];
        out.blit_from(source.map(&self.mapper));
        return Ok(());
    }
}

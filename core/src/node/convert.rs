use std::marker::PhantomData;

use anyhow::Result;

use crate::{Buffer, BufferReader, Node, NodeBuilder, NodeDecl, NodeHandle, NodeRef, RenderContext};

pub struct Convert<N, T, R>
where
    N: NodeDecl,
    N::Node: Node<Element = T> + 'static,
    T: Default,
    R: Default + Copy + From<T>,
{
    source: NodeHandle<N>,
    phantom: PhantomData<R>,
}

impl<N, T, R> NodeDecl for Convert<N, T, R>
where
    N: NodeDecl,
    N::Node: Node<Element = T> + 'static,
    T: Default,
    R: Default + Copy + From<T>,
{
    const KIND: &'static str = "convert";

    type Node = ConvertNode<N::Node, T, R>;

    async fn materialize(self, builder: &mut NodeBuilder<'_>) -> Result<Self::Node> {
        return Ok(Self::Node {
            source: builder.node("source", self.source).await?,
            phantom: self.phantom,
        });
    }
}

pub struct ConvertNode<N, T, R>
where
    N: Node<Element = T> + 'static,
    R: Default + Copy + From<T>,
{
    source: NodeRef<N>,
    phantom: PhantomData<R>,
}

impl<N, T, R> Node for ConvertNode<N, T, R>
where
    N: Node<Element = T> + 'static,
    R: Default + Copy + From<T>,
{
    type Element = R;

    fn update(&mut self, ctx: &RenderContext, out: &mut Buffer<Self::Element>) -> Result<()> {
        let source = &ctx[self.source];
        out.blit_from(source.map(|e| R::from(e)));
        return Ok(());
    }
}

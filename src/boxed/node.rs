use std::marker::PhantomData;

use anyhow::Result;
use async_trait::async_trait;
use palette::convert::{FromColorUnclamped, IntoColorUnclamped};

use crate::{Buffer, BufferReader, Node, NodeBuilder, NodeDecl, RenderContext};

use super::Boxed;

#[async_trait(? Send)]
pub trait DynNodeDecl<E> {
    async fn materialize(self: Box<Self>, builder: &mut NodeBuilder<'_>) -> Result<BoxedNode<E>>;
}

#[async_trait(? Send)]
impl<T, E> DynNodeDecl<E> for T
where
    T: NodeDecl + 'static,
    E: Default + Copy + FromColorUnclamped<<<T as NodeDecl>::Node as Node>::Element> + 'static,
{
    async fn materialize(self: Box<Self>, builder: &mut NodeBuilder<'_>) -> Result<BoxedNode<E>> {
        let node = <T as NodeDecl>::materialize(*self, builder).await?;
        let node = WrappedNode {
            node,
            buffer: Buffer::with_default(builder.size),
            target: PhantomData,
        };

        return Ok(Box::new(node));
    }
}

impl<T, E> Boxed<dyn DynNodeDecl<E>> for T
where
    T: NodeDecl + 'static,
    E: Default + Copy + FromColorUnclamped<<<T as NodeDecl>::Node as Node>::Element> + 'static,
{
    fn boxed(self) -> Box<dyn DynNodeDecl<E>> {
        return Box::new(self);
    }
}

pub type BoxedNodeDecl<E> = Box<dyn DynNodeDecl<E>>;

impl<E> NodeDecl for BoxedNodeDecl<E>
where E: Default + Copy
{
    const KIND: &'static str = "boxed";

    type Node = BoxedNode<E>;

    async fn materialize(self, builder: &mut NodeBuilder<'_>) -> Result<Self::Node> {
        return DynNodeDecl::materialize(self, builder).await;
    }
}

struct WrappedNode<N, E>
where
    N: Node,
    E: Default + Copy + FromColorUnclamped<<N as Node>::Element>,
{
    node: N,
    buffer: Buffer<<N as Node>::Element>,
    target: PhantomData<E>,
}

impl<N, E> Node for WrappedNode<N, E>
where
    N: Node,
    E: Default + Copy + FromColorUnclamped<<N as Node>::Element>,
{
    type Element = E;

    fn update(&mut self, ctx: &RenderContext, out: &mut Buffer<Self::Element>) -> Result<()> {
        self.node.update(ctx, &mut self.buffer)?;

        out.blit_from(self.buffer.map(|e| e.into_color_unclamped()));
        return Ok(());
    }
}

pub trait DynNode<E> {
    fn update(&mut self, ctx: &RenderContext, out: &mut Buffer<E>) -> Result<()>;
}

impl<N, E> DynNode<E> for WrappedNode<N, E>
where
    N: Node,
    E: Default + Copy + FromColorUnclamped<<N as Node>::Element>,
{
    fn update(&mut self, ctx: &RenderContext, out: &mut Buffer<E>) -> Result<()> {
        return Node::update(self, ctx, out);
    }
}

pub type BoxedNode<E> = Box<dyn DynNode<E>>;

impl<E> Node for BoxedNode<E>
where E: Default + Copy
{
    type Element = E;

    fn update(&mut self, ctx: &RenderContext, out: &mut Buffer<Self::Element>) -> Result<()> {
        return DynNode::update(self.as_mut(), ctx, out);
    }
}

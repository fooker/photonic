use anyhow::Result;
use async_trait::async_trait;
use palette::rgb::Rgb;
use palette::{FromColor, IntoColor};

use super::Boxed;
use crate::{Buffer, BufferReader, Node, NodeBuilder, NodeDecl, RenderContext};

#[async_trait(?Send)]
pub trait DynNodeDecl {
    async fn materialize(self: Box<Self>, builder: &mut NodeBuilder<'_>) -> Result<BoxedNode>;
}

#[async_trait(?Send)]
impl<T> DynNodeDecl for T
where
    T: NodeDecl + 'static,
    <T as NodeDecl>::Node: Sized + 'static,
    Rgb: FromColor<<<T as NodeDecl>::Node as Node>::Element>,
{
    async fn materialize(self: Box<Self>, builder: &mut NodeBuilder<'_>) -> Result<BoxedNode> {
        let node = <T as NodeDecl>::materialize(*self, builder).await?;
        let node = WrappedNode {
            node,
            buffer: Buffer::with_default(builder.size),
        };

        return Ok(Box::new(node));
    }
}

impl<T> Boxed<dyn DynNodeDecl> for T
where
    T: NodeDecl + 'static,
    <T as NodeDecl>::Node: Sized + 'static,
    Rgb: FromColor<<<T as NodeDecl>::Node as Node>::Element>,
{
    fn boxed(self) -> Box<dyn DynNodeDecl> {
        return Box::new(self);
    }
}

pub type BoxedNodeDecl = Box<dyn DynNodeDecl>;

impl NodeDecl for BoxedNodeDecl {
    const KIND: &'static str = "boxed";

    type Node = BoxedNode;

    async fn materialize(self, builder: &mut NodeBuilder<'_>) -> Result<Self::Node> {
        return DynNodeDecl::materialize(self, builder).await;
    }
}

struct WrappedNode<N>
where N: Node
{
    node: N,
    buffer: Buffer<N::Element>,
}

impl<N> Node for WrappedNode<N>
where
    N: Node,
    Rgb: FromColor<<N as Node>::Element>,
{
    type Element = Rgb;

    fn update(&mut self, ctx: &RenderContext, out: &mut Buffer<Self::Element>) -> Result<()> {
        self.node.update(ctx, &mut self.buffer)?;

        out.blit_from(self.buffer.map(|e| e.into_color()));
        return Ok(());
    }
}

pub trait DynNode {
    fn update(&mut self, ctx: &RenderContext, out: &mut Buffer<Rgb>) -> Result<()>;
}

impl<N> DynNode for WrappedNode<N>
where
    N: Node,
    Rgb: FromColor<<N as Node>::Element>,
{
    fn update(&mut self, ctx: &RenderContext, out: &mut Buffer<Rgb>) -> Result<()> {
        return Node::update(self, ctx, out);
    }
}

pub type BoxedNode = Box<dyn DynNode>;

impl Node for BoxedNode {
    type Element = Rgb;

    fn update(&mut self, ctx: &RenderContext, out: &mut Buffer<Self::Element>) -> Result<()> {
        return DynNode::update(self.as_mut(), ctx, out);
    }
}

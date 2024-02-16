use anyhow::Result;
use async_trait::async_trait;
use palette::rgb::Rgb;
use palette::{FromColor, IntoColor};

use photonic::{Buffer, BufferReader, Context, Node, NodeBuilder, NodeDecl};

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

        return Ok(Box::new(node) as Box<dyn DynNode>);
    }
}

pub type BoxedNodeDecl = Box<dyn DynNodeDecl>;

impl NodeDecl for BoxedNodeDecl {
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
    const KIND: &'static str = "boxed";
    type Element = Rgb;

    fn update(&mut self, ctx: &Context, out: &mut Buffer<Self::Element>) -> Result<()> {
        self.node.update(ctx, &mut self.buffer)?;

        out.blit_from(self.buffer.map(|e| e.into_color()));
        return Ok(());
    }
}

pub trait DynNode {
    fn update(&mut self, ctx: &Context, out: &mut Buffer<Rgb>) -> Result<()>;
}

impl<N> DynNode for WrappedNode<N>
where
    N: Node,
    Rgb: FromColor<<N as Node>::Element>,
{
    fn update(&mut self, ctx: &Context, out: &mut Buffer<Rgb>) -> Result<()> {
        return Node::update(self, ctx, out);
    }
}

pub type BoxedNode = Box<dyn DynNode>;

impl Node for BoxedNode {
    const KIND: &'static str = "todo!()";

    type Element = Rgb;

    fn update(&mut self, ctx: &Context, out: &mut Buffer<Self::Element>) -> Result<()> {
        return DynNode::update(self.as_mut(), ctx, out);
    }
}

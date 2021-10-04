use anyhow::Result;

use crate::node::{Node, NodeDecl, RenderType};

pub trait OutputDecl<Node>
    where
        Node: self::NodeDecl,
{
    type Target: Output<Node::Target>;

    fn materialize(self, size: usize) -> Result<Self::Target>
        where
            Self::Target: std::marker::Sized;
}

pub trait Output<Node>
    where
        Node: self::Node,
{
    const KIND: &'static str;

    fn render(&mut self, render: <Node as RenderType<'_, Node>>::Render) -> Result<()>;
}

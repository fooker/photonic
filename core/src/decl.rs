use anyhow::Result;

use crate::{Node, Output, NodeBuilder};

pub trait NodeDecl {
    type Node: Node;

    fn materialize(self, builder: &mut NodeBuilder) -> Result<Self::Node>;
}

pub trait OutputDecl
{
    type Output: Output;

    fn materialize(self, size: usize) -> Result<Self::Output>
        where Self::Output: Sized;
}
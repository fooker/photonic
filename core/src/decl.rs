use std::future::Future;
use anyhow::Result;

use crate::{Node, Output, NodeBuilder, AttrValue, Attr, AttrBuilder};
use crate::attr::{Bounded, Bounds};

pub trait NodeDecl {
    type Node: Node;

    fn materialize(self, builder: &mut NodeBuilder) -> impl Future<Output=Result<Self::Node>>;
}

pub trait OutputDecl
{
    type Output: Output;

    fn materialize(self, size: usize) -> impl Future<Output=Result<Self::Output>>;
}

pub trait FreeAttrDecl {
    type Value: AttrValue;
    type Attr: Attr<Value=Self::Value>;

    fn materialize(self, builder: &mut AttrBuilder) -> Result<Self::Attr>;
}

pub trait BoundAttrDecl {
    type Value: AttrValue + Bounded;
    type Attr: Attr<Value=Self::Value>;

    fn materialize(self, bounds: Bounds<Self::Value>, builder: &mut AttrBuilder) -> Result<Self::Attr>;
}

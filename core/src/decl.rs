use anyhow::Result;
use std::future::Future;

use crate::attr::{Bounded, Bounds};
use crate::{Attr, AttrBuilder, AttrValue, Node, NodeBuilder, Output};

pub trait NodeDecl {
    type Node: Node;

    fn materialize(self, builder: &mut NodeBuilder) -> impl Future<Output = Result<Self::Node>>;
}

pub trait OutputDecl {
    type Output: Output;

    fn materialize(self) -> impl Future<Output = Result<Self::Output>>;
}

pub trait FreeAttrDecl {
    type Value: AttrValue;
    type Attr: Attr<Value = Self::Value>;

    fn materialize(self, builder: &mut AttrBuilder) -> Result<Self::Attr>;
}

pub trait BoundAttrDecl {
    type Value: AttrValue + Bounded;
    type Attr: Attr<Value = Self::Value>;

    fn materialize(self, bounds: Bounds<Self::Value>, builder: &mut AttrBuilder) -> Result<Self::Attr>;
}

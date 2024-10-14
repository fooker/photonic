use anyhow::Result;
use std::future::Future;

use crate::attr::{Bounded, Bounds};
use crate::{Attr, AttrBuilder, AttrValue, Node, NodeBuilder, Output};

pub trait NodeDecl {
    const KIND: &'static str;

    type Node: Node;

    fn materialize(self, builder: &mut NodeBuilder) -> impl Future<Output = Result<Self::Node>>;
}

pub trait OutputDecl {
    const KIND: &'static str;

    type Output: Output;

    fn materialize(self) -> impl Future<Output = Result<Self::Output>>;
}

pub trait FreeAttrDecl<V: AttrValue> {
    const KIND: &'static str;

    type Attr: Attr<V>;

    fn materialize(self, builder: &mut AttrBuilder) -> Result<Self::Attr>;
}

pub trait BoundAttrDecl<V: AttrValue + Bounded> {
    const KIND: &'static str;

    type Attr: Attr<V>;

    fn materialize(self, bounds: Bounds<V>, builder: &mut AttrBuilder) -> Result<Self::Attr>;
}

#[allow(unreachable_code)]
impl<V> FreeAttrDecl<V> for !
where V: AttrValue
{
    const KIND: &'static str = "never";

    type Attr = !;

    fn materialize(self, _builder: &mut AttrBuilder) -> Result<Self::Attr> {
        return self;
    }
}

#[allow(unreachable_code)]
impl<V> BoundAttrDecl<V> for !
where V: AttrValue + Bounded
{
    const KIND: &'static str = "never";

    type Attr = !;

    fn materialize(self, _bounds: Bounds<V>, _builder: &mut AttrBuilder) -> Result<Self::Attr> {
        return self;
    }
}

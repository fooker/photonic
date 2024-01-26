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

    fn materialize(self, size: usize) -> impl Future<Output=Result<Self::Output>>
        where Self::Output: Sized;
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

// impl<V, T> BoundAttrDecl for Box<T>
//     where V: AttrValue + Bounded,
//           T: BoundAttrDecl<Value=V>,
// {
//     type Value = V;
//     type Target = T::Target;
//
//     fn materialize(self, bounds: Bounds<V>, builder: &mut AttrBuilder) -> Result<Self::Target> {
//         return T::materialize(*self, bounds, builder);
//     }
// }
//
// impl<V, T> FreeAttrDecl for Box<T>
//     where V: AttrValue,
//           T: FreeAttrDecl<Value=V>,
// {
//     type Value = V;
//     type Target = T::Target;
//
//     fn materialize(self, builder: &mut AttrBuilder) -> Result<Self::Target> {
//         return T::materialize(*self, builder);
//     }
// }
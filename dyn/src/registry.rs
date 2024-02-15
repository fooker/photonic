use anyhow::Result;
use serde::de::DeserializeOwned;

use photonic::attr::Bounded;
use photonic::AttrValue;

use crate::boxed::{BoxedBoundAttrDecl, BoxedFreeAttrDecl, BoxedNodeDecl, BoxedOutputDecl};
use crate::builder::{AttrBuilder, NodeBuilder, OutputBuilder};
use crate::config::Anything;

pub trait Factory<T, Builder>
    where T: Sized,
{
    fn produce(self: Box<Self>, config: Anything, builder: &mut Builder) -> Result<T>;
}

impl<F, T, Builder> Factory<T, Builder> for F
    where F: FnOnce(Anything, &mut Builder) -> Result<T>,
{
    fn produce(self: Box<Self>, config: Anything, builder: &mut Builder) -> Result<T> {
        return self(config, builder);
    }
}

impl<F, T, Builder> From<F> for Box<dyn Factory<T, Builder>>
    where F: FnOnce(Anything, &mut Builder) -> Result<T> + 'static,
{
    fn from(f: F) -> Self {
        return Box::new(f);
    }
}

pub type NodeFactory<Builder> = Box<dyn Factory<BoxedNodeDecl, Builder>>;
pub type FreeAttrFactory<V, Builder> = Box<dyn Factory<BoxedFreeAttrDecl<V>, Builder>>;
pub type BoundAttrFactory<V, Builder> = Box<dyn Factory<BoxedBoundAttrDecl<V>, Builder>>;
pub type OutputFactory<Builder> = Box<dyn Factory<BoxedOutputDecl, Builder>>;

pub struct AttrFactory<V, Builder> {
    pub free: Option<FreeAttrFactory<V, Builder>>,
    pub bound: Option<BoundAttrFactory<V, Builder>>,
}

pub trait Registry {
    fn node<Builder>(kind: &str) -> Option<NodeFactory<Builder>>
        where Builder: NodeBuilder;

    fn free_attr<V, Builder>(kind: &str) -> Option<FreeAttrFactory<V, Builder>>
        where Builder: AttrBuilder,
              V: AttrValue + DeserializeOwned;

    fn bound_attr<V, Builder>(kind: &str) -> Option<BoundAttrFactory<V, Builder>>
        where Builder: AttrBuilder,
              V: AttrValue + DeserializeOwned + Bounded;

    fn output<Builder>(kind: &str) -> Option<OutputFactory<Builder>>
        where Builder: OutputBuilder;
}

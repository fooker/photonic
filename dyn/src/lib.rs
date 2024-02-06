pub use serde;
use serde::de::DeserializeOwned;

use photonic::{AttrValue, NodeDecl, NodeHandle};
use photonic::attr::Bounded;
use photonic::input::InputValue;
pub use photonic_app_derive::{DynamicNode};

use crate::boxed::{BoxedBoundAttrDecl, BoxedFreeAttrDecl, BoxedNodeDecl};
use crate::builder::NodeBuilder;
use crate::registry::NodeFactory;

pub mod config;
pub mod registry;
pub mod boxed;
pub mod builder;

pub trait Dynamic: Sized {
    type Config: DeserializeOwned;

    fn from_config<Builder>(builder: &mut Builder, name: &'static str, config: Self::Config) -> anyhow::Result<Self>
        where Builder: NodeBuilder;
}

impl Dynamic for NodeHandle<BoxedNodeDecl>
{
    type Config = config::Node;

    fn from_config<Builder>(builder: &mut Builder, name: &'static str, config: Self::Config) -> anyhow::Result<Self>
        where Builder: NodeBuilder,
    {
        return builder.node(name, config);
    }
}

impl<V> Dynamic for BoxedFreeAttrDecl<V>
    where V: AttrValue + InputValue + DeserializeOwned,
{
    type Config = config::Attr;

    fn from_config<Builder>(builder: &mut Builder, name: &'static str, config: Self::Config) -> anyhow::Result<Self>
        where Builder: NodeBuilder,
    {
        return builder.free_attr(name, config);
    }
}

impl<V> Dynamic for BoxedBoundAttrDecl<V>
    where V: AttrValue + InputValue + DeserializeOwned + Bounded,
{
    type Config = config::Attr;

    fn from_config<Builder>(builder: &mut Builder, name: &'static str, config: Self::Config) -> anyhow::Result<Self>
        where Builder: NodeBuilder,
    {
        return builder.bound_attr(name, config);
    }
}

pub trait DynamicNode: NodeDecl {
    const KIND: &'static str;

    fn factory<Builder>() -> NodeFactory<Builder>
        where Builder: NodeBuilder;
}

pub trait DynamicAttr {
    const KIND: &'static str;

    fn factory<Builder>() -> NodeFactory<Builder>
        where Builder: NodeBuilder;
}


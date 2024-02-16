use crate::boxed::{BoxedBoundAttrDecl, BoxedFreeAttrDecl, BoxedNodeDecl};
use crate::builder::NodeBuilder;
use crate::config;
use photonic::attr::Bounded;
use photonic::input::InputValue;
use photonic::scene::InputHandle;
use photonic::{AttrValue, NodeHandle};
use serde::de::DeserializeOwned;

pub trait Dynamic: Sized {
    type Config: DeserializeOwned;

    fn from_config<Builder>(builder: &mut Builder, name: &'static str, config: Self::Config) -> anyhow::Result<Self>
    where Builder: NodeBuilder;
}

impl Dynamic for NodeHandle<BoxedNodeDecl> {
    type Config = config::Node;

    fn from_config<Builder>(builder: &mut Builder, name: &'static str, config: Self::Config) -> anyhow::Result<Self>
    where Builder: NodeBuilder {
        return builder.node(name, config);
    }
}

impl<V> Dynamic for BoxedFreeAttrDecl<V>
where V: AttrValue + InputValue + DeserializeOwned
{
    type Config = config::Attr;

    fn from_config<Builder>(builder: &mut Builder, name: &'static str, config: Self::Config) -> anyhow::Result<Self>
    where Builder: NodeBuilder {
        return builder.free_attr(name, config);
    }
}

impl<V> Dynamic for BoxedBoundAttrDecl<V>
where V: AttrValue + InputValue + DeserializeOwned + Bounded
{
    type Config = config::Attr;

    fn from_config<Builder>(builder: &mut Builder, name: &'static str, config: Self::Config) -> anyhow::Result<Self>
    where Builder: NodeBuilder {
        return builder.bound_attr(name, config);
    }
}

impl<V> Dynamic for InputHandle<V>
where V: InputValue + DeserializeOwned
{
    type Config = config::Input;

    fn from_config<Builder>(builder: &mut Builder, _name: &'static str, config: Self::Config) -> anyhow::Result<Self>
    where Builder: NodeBuilder {
        return builder.input(config);
    }
}

impl<T> Dynamic for Option<T>
where T: Dynamic
{
    type Config = Option<T::Config>;

    fn from_config<Builder>(builder: &mut Builder, name: &'static str, config: Self::Config) -> anyhow::Result<Self>
    where Builder: NodeBuilder {
        return config.map(|config| T::from_config(builder, name, config)).transpose();
    }
}

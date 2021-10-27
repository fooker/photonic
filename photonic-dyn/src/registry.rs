use std::marker::PhantomData;

use anyhow::Result;
use serde_json::Value;

use photonic_core::attr::Bounded;
use photonic_core::boxed::{
    BoxedBoundAttrDecl, BoxedNodeDecl, BoxedOutputDecl, BoxedUnboundAttrDecl,
};
use photonic_core::element;
use photonic_core::element::RGBColor;

use crate::builder::{AttrBuilder, NodeBuilder, OutputBuilder};
use crate::model::{AttrValueFactory, BoundAttrModel, NodeModel, OutputModel, UnboundAttrModel};

pub trait Factory<T, Builder> {
    fn produce(self: Box<Self>, config: Value, builder: &mut Builder) -> Result<T>;
}

pub fn output<Builder, X>() -> Box<dyn Factory<BoxedOutputDecl<BoxedNodeDecl<RGBColor>>, Builder>>
where
    X: OutputModel + 'static,
    Builder: OutputBuilder,
{
    return Box::new(OutputFactory::<X>(Default::default()));
}

pub fn node<Builder, X>() -> Box<dyn Factory<BoxedNodeDecl<element::RGBColor>, Builder>>
where
    X: NodeModel + 'static,
    Builder: NodeBuilder,
{
    return Box::new(NodeFactory::<X>(Default::default()));
}

pub fn bound_attr<Builder, V, X>() -> Box<dyn Factory<BoxedBoundAttrDecl<V>, Builder>>
where
    V: AttrValueFactory + Bounded,
    X: BoundAttrModel<V> + 'static,
    Builder: AttrBuilder,
{
    return Box::new(BoundAttrFactory::<X>(Default::default()));
}

pub fn unbound_attr<Builder, V, X>() -> Box<dyn Factory<BoxedUnboundAttrDecl<V>, Builder>>
where
    V: AttrValueFactory,
    X: UnboundAttrModel<V> + 'static,
    Builder: AttrBuilder,
{
    return Box::new(UnboundAttrFactory::<X>(Default::default()));
}

pub trait OutputRegistry {
    fn manufacture<Builder: OutputBuilder>(
        kind: &str,
    ) -> Option<Box<dyn Factory<BoxedOutputDecl<BoxedNodeDecl<RGBColor>>, Builder>>>;
}

pub struct OutputFactory<T>(PhantomData<T>);

impl<T, Builder> Factory<BoxedOutputDecl<BoxedNodeDecl<RGBColor>>, Builder> for OutputFactory<T>
where
    T: OutputModel,
    Builder: OutputBuilder,
{
    fn produce(
        self: Box<Self>,
        config: Value,
        builder: &mut Builder,
    ) -> Result<BoxedOutputDecl<BoxedNodeDecl<RGBColor>>> {
        let model: T = serde_json::from_value(config)?;
        let decl = T::assemble(model, builder)?;
        return Ok(decl);
    }
}

pub trait NodeRegistry {
    fn manufacture<Builder: NodeBuilder>(
        kind: &str,
    ) -> Option<Box<dyn Factory<BoxedNodeDecl<element::RGBColor>, Builder>>>;
}

pub struct NodeFactory<T>(PhantomData<T>);

impl<T, Builder> Factory<BoxedNodeDecl<element::RGBColor>, Builder> for NodeFactory<T>
where
    T: NodeModel,
    Builder: NodeBuilder,
{
    fn produce(
        self: Box<Self>,
        config: Value,
        builder: &mut Builder,
    ) -> Result<BoxedNodeDecl<element::RGBColor>> {
        let model: T = serde_json::from_value(config)?;
        let decl = T::assemble(model, builder)?;
        return Ok(decl);
    }
}

pub trait UnboundAttrRegistry {
    fn manufacture<V: AttrValueFactory, Builder: AttrBuilder>(
        kind: &str,
    ) -> Option<Box<dyn Factory<BoxedUnboundAttrDecl<V>, Builder>>>;
}

pub struct UnboundAttrFactory<T>(PhantomData<T>);

impl<V, T, Builder> Factory<BoxedUnboundAttrDecl<V>, Builder> for UnboundAttrFactory<T>
where
    V: AttrValueFactory,
    T: UnboundAttrModel<V>,
    Builder: AttrBuilder,
{
    fn produce(
        self: Box<Self>,
        config: Value,
        builder: &mut Builder,
    ) -> Result<BoxedUnboundAttrDecl<V>> {
        let model: T = serde_json::from_value(config)?;
        let decl = T::assemble(model, builder)?;
        return Ok(decl);
    }
}

pub trait BoundAttrRegistry {
    fn manufacture<V: AttrValueFactory + Bounded, Builder: AttrBuilder>(
        kind: &str,
    ) -> Option<Box<dyn Factory<BoxedBoundAttrDecl<V>, Builder>>>;
}

pub struct BoundAttrFactory<T>(PhantomData<T>);

impl<V, T, Builder> Factory<BoxedBoundAttrDecl<V>, Builder> for BoundAttrFactory<T>
where
    V: AttrValueFactory + Bounded,
    T: BoundAttrModel<V>,
    Builder: AttrBuilder,
{
    fn produce(
        self: Box<Self>,
        config: Value,
        builder: &mut Builder,
    ) -> Result<BoxedBoundAttrDecl<V>> {
        let model: T = serde_json::from_value(config)?;
        let decl = T::assemble(model, builder)?;
        return Ok(decl);
    }
}

pub trait Registry {
    type Output: self::OutputRegistry;
    type Node: self::NodeRegistry;
    type BoundAttr: self::BoundAttrRegistry;
    type UnboundAttr: self::UnboundAttrRegistry;
}

pub struct CombinedOutputRegistry<R1, R2>(PhantomData<(R1, R2)>);

impl<R1, R2> OutputRegistry for CombinedOutputRegistry<R1, R2>
where
    R1: OutputRegistry,
    R2: OutputRegistry,
{
    fn manufacture<Builder: OutputBuilder>(
        kind: &str,
    ) -> Option<Box<dyn Factory<BoxedOutputDecl<BoxedNodeDecl<RGBColor>>, Builder>>> {
        return R1::manufacture(kind).or_else(|| R2::manufacture(kind));
    }
}

pub struct CombinedNodeRegistry<R1, R2>(PhantomData<(R1, R2)>);

impl<R1, R2> NodeRegistry for CombinedNodeRegistry<R1, R2>
where
    R1: NodeRegistry,
    R2: NodeRegistry,
{
    fn manufacture<Builder: NodeBuilder>(
        kind: &str,
    ) -> Option<Box<dyn Factory<BoxedNodeDecl<RGBColor>, Builder>>> {
        return R1::manufacture(kind).or_else(|| R2::manufacture(kind));
    }
}

pub struct CombinedBoundAttrRegistry<R1, R2>(PhantomData<(R1, R2)>);

impl<R1, R2> BoundAttrRegistry for CombinedBoundAttrRegistry<R1, R2>
where
    R1: BoundAttrRegistry,
    R2: BoundAttrRegistry,
{
    fn manufacture<V: AttrValueFactory + Bounded, Builder: AttrBuilder>(
        kind: &str,
    ) -> Option<Box<dyn Factory<BoxedBoundAttrDecl<V>, Builder>>> {
        return R1::manufacture(kind).or_else(|| R2::manufacture(kind));
    }
}

pub struct CombinedUnboundAttrRegistry<R1, R2>(PhantomData<(R1, R2)>);

impl<R1, R2> UnboundAttrRegistry for CombinedUnboundAttrRegistry<R1, R2>
where
    R1: UnboundAttrRegistry,
    R2: UnboundAttrRegistry,
{
    fn manufacture<V: AttrValueFactory, Builder: AttrBuilder>(
        kind: &str,
    ) -> Option<Box<dyn Factory<BoxedUnboundAttrDecl<V>, Builder>>> {
        return R1::manufacture(kind).or_else(|| R2::manufacture(kind));
    }
}

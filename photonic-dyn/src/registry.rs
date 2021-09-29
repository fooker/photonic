use std::marker::PhantomData;

use anyhow::Result;
use serde_json::Value;

use photonic_core::{color, NodeDecl};
use photonic_core::attr::Bounded;
use photonic_core::boxed::{BoxedBoundAttrDecl, BoxedNodeDecl, BoxedOutputDecl, BoxedUnboundAttrDecl, Wrap};

use crate::builder::{AttrBuilder, NodeBuilder, OutputBuilder};
use crate::model::{AttrValueFactory, BoundAttrModel, NodeModel, OutputModel, UnboundAttrModel};

pub trait Factory<T, Builder> {
    fn produce(
        self: Box<Self>,
        config: Value,
        builder: &mut Builder,
    ) -> Result<T>;
}

impl<Builder> dyn Factory<BoxedOutputDecl<color::RGBColor>, Builder> {
    pub fn output<X>() -> Box<dyn Factory<BoxedOutputDecl<color::RGBColor>, Builder>>
        where
            X: OutputModel + 'static,
            Builder: OutputBuilder,
    {
        return Box::new(OutputFactory::<X>(Default::default()));
    }
}

impl<Builder> dyn Factory<BoxedNodeDecl<color::RGBColor>, Builder> {
    pub fn node<X>() -> Box<dyn Factory<BoxedNodeDecl<color::RGBColor>, Builder>>
        where
            X: NodeModel + 'static,
            Builder: NodeBuilder,
    {
        return Box::new(NodeFactory::<X>(Default::default()));
    }
}

impl<Builder> dyn Factory<BoxedBoundAttrDecl<color::RGBColor>, Builder> {
    pub fn bound_attr<V, X>() -> Box<dyn Factory<BoxedBoundAttrDecl<V>, Builder>>
        where
            V: AttrValueFactory + Bounded,
            X: BoundAttrModel<V> + 'static,
            Builder: AttrBuilder,
    {
        return Box::new(BoundAttrFactory::<X>(Default::default()));
    }
}

impl<Builder> dyn Factory<BoxedUnboundAttrDecl<color::RGBColor>, Builder> {
    pub fn unbound_attr<V, X>() -> Box<dyn Factory<BoxedUnboundAttrDecl<V>, Builder>>
        where
            V: AttrValueFactory,
            X: UnboundAttrModel<V> + 'static,
            Builder: AttrBuilder,
    {
        return Box::new(UnboundAttrFactory::<X>(Default::default()));
    }
}

pub trait OutputRegistry {
    fn manufacture<Builder: OutputBuilder>(
        kind: &str,
    ) -> Option<Box<dyn Factory<BoxedOutputDecl<color::RGBColor>, Builder>>>;
}

pub struct OutputFactory<T>(PhantomData<T>);

impl<T, Builder> Factory<BoxedOutputDecl<color::RGBColor>, Builder> for OutputFactory<T>
    where T: OutputModel,
          Builder: OutputBuilder,
{
    fn produce(self: Box<Self>, config: Value, builder: &mut Builder) -> Result<BoxedOutputDecl<color::RGBColor>> {
        let model: T = serde_json::from_value(config)?;
        let decl = T::assemble(model, builder)?;
        return Ok(decl);
    }
}

pub trait NodeRegistry {
    fn manufacture<Builder: NodeBuilder>(
        kind: &str,
    ) -> Option<Box<dyn Factory<BoxedNodeDecl<color::RGBColor>, Builder>>>;
}

pub struct NodeFactory<T>(PhantomData<T>);

impl<T, Builder> Factory<BoxedNodeDecl<color::RGBColor>, Builder> for NodeFactory<T>
    where T: NodeModel,
          Builder: NodeBuilder,
{
    fn produce(self: Box<Self>, config: Value, builder: &mut Builder) -> Result<BoxedNodeDecl<color::RGBColor>> {
        let model: T = serde_json::from_value(config)?;
        let decl = T::assemble(model, builder)?;
        return Ok(BoxedNodeDecl::wrap(decl.map(Into::into)));
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
    fn produce(self: Box<Self>, config: Value, builder: &mut Builder) -> Result<BoxedUnboundAttrDecl<V>> {
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
    fn produce(self: Box<Self>, config: Value, builder: &mut Builder) -> Result<BoxedBoundAttrDecl<V>> {
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

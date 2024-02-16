use std::marker::PhantomData;

use anyhow::{anyhow, Context, Result};
use serde::de::DeserializeOwned;

use photonic::attr::{AsFixedAttr, Bounded};
use photonic::input::InputValue;
use photonic::scene::InputHandle;
use photonic::{NodeHandle, Scene};

use crate::boxed::{BoxedBoundAttrDecl, BoxedFreeAttrDecl, BoxedNodeDecl, BoxedOutputDecl};
use crate::builder::values::DynAttrValue;
use crate::config;
use crate::registry::Registry;

mod values;

pub trait InputBuilder {
    fn input<V>(&mut self, config: config::Input) -> Result<InputHandle<V>>
    where V: InputValue;
}

pub trait AttrBuilder: InputBuilder {
    fn free_attr<V>(&mut self, name: &str, config: config::Attr) -> Result<BoxedFreeAttrDecl<V>>
    where V: DynAttrValue + DeserializeOwned;

    fn bound_attr<V>(&mut self, name: &str, config: config::Attr) -> Result<BoxedBoundAttrDecl<V>>
    where V: DynAttrValue + DeserializeOwned + Bounded;
}

pub trait NodeBuilder: AttrBuilder {
    fn node(&mut self, name: &str, config: config::Node) -> Result<NodeHandle<BoxedNodeDecl>>;
}

pub trait OutputBuilder {}

pub struct Builder<Registry> {
    scene: Scene,
    registry: PhantomData<Registry>,
}

impl<Registry> Builder<Registry> {
    pub fn new(size: usize) -> Self {
        let scene = Scene::new(size);

        return Self {
            scene,
            registry: PhantomData,
        };
    }

    pub fn build(self) -> Scene {
        return self.scene;
    }
}

impl<Registry> Builder<Registry>
where Registry: self::Registry
{
    pub fn node(&mut self, name: &str, config: config::Node) -> Result<NodeHandle<BoxedNodeDecl>> {
        return NodeBuilder::node(self, name, config);
    }

    pub fn output(&mut self, config: config::Output) -> Result<BoxedOutputDecl> {
        let factory = Registry::output(&config.kind).ok_or_else(|| anyhow!("Unknown output type: {}", config.kind))?;

        let decl = factory
            .produce(config.config, self)
            .with_context(|| format!("Failed to build output: (type={})", config.kind))?;

        return Ok(decl);
    }
}

impl<Registry> InputBuilder for Builder<Registry>
where Registry: self::Registry
{
    fn input<V>(&mut self, config: config::Input) -> Result<InputHandle<V>>
    where V: InputValue {
        let decl =
            self.scene.input(&config.input).with_context(|| format!("Failed to build input: {}", config.input))?;

        return Ok(decl);
    }
}

impl<Registry> AttrBuilder for Builder<Registry>
where Registry: self::Registry
{
    fn free_attr<V>(&mut self, name: &str, config: config::Attr) -> Result<BoxedFreeAttrDecl<V>>
    where V: DynAttrValue + DeserializeOwned {
        match config {
            config::Attr::Attr {
                kind,
                config,
            } => {
                let factory = Registry::free_attr(&kind).ok_or_else(|| anyhow!("Unknown attribute type: {}", kind))?;

                let decl = factory
                    .produce(config, self)
                    .with_context(|| format!("Failed to build attribute: (type={}) @{}", kind, name))?;

                return Ok(decl);
            }

            config::Attr::Input {
                input,
                initial,
            } => {
                let initial = V::parse(initial)?;

                let input = self.input::<V>(input)?;
                let attr = input.attr(initial);

                return Ok(Box::new(attr));
            }

            config::Attr::Fixed(value) => {
                let value = V::parse(value)?;

                let attr = V::fixed(value);

                return Ok(Box::new(attr));
            }
        }
    }

    fn bound_attr<V>(&mut self, name: &str, config: config::Attr) -> Result<BoxedBoundAttrDecl<V>>
    where V: DynAttrValue + DeserializeOwned + Bounded {
        match config {
            config::Attr::Attr {
                kind,
                config,
            } => {
                let factory = Registry::bound_attr(&kind).ok_or_else(|| anyhow!("Unknown attribute type: {}", kind))?;

                let decl = factory
                    .produce(config, self)
                    .with_context(|| format!("Failed to build attribute: (type={}) @{}", kind, name))?;

                return Ok(decl);
            }

            config::Attr::Input {
                input,
                initial,
            } => {
                let initial = V::parse(initial)?;

                let input = self.input::<V>(input)?;
                let attr = input.attr(initial);

                return Ok(Box::new(attr));
            }

            config::Attr::Fixed(value) => {
                let value = V::parse(value)?;

                let attr = V::fixed(value);

                return Ok(Box::new(attr));
            }
        }
    }
}

impl<Registry> NodeBuilder for Builder<Registry>
where Registry: self::Registry
{
    fn node(&mut self, name: &str, config: config::Node) -> Result<NodeHandle<BoxedNodeDecl>> {
        let factory = Registry::node(&config.kind).ok_or_else(|| anyhow!("Unknown node type: {}", config.kind))?;

        let decl = factory
            .produce(config.config, self)
            .with_context(|| format!("Failed to build node: {} (type={}) @{}", config.name, config.kind, name))?;

        return self.scene.node(&config.name, decl);
    }
}

impl<Registry> OutputBuilder for Builder<Registry> where Registry: self::Registry {}

use std::any::type_name;
use std::marker::PhantomData;

use anyhow::{anyhow, bail, Context, Result};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Value;

use photonic::attr::{AsFixedAttr, Bounded};
use photonic::input::InputValue;
use photonic::scene::InputHandle;
use photonic::{AttrValue, NodeHandle, Scene};
use photonic_dynamic_registry::Registry;

use crate::boxed::{BoxedBoundAttrDecl, BoxedFreeAttrDecl, BoxedNodeDecl, BoxedOutputDecl};
use crate::config;

pub trait InputBuilder {
    fn input<V>(&mut self, config: config::Input) -> Result<InputHandle<V>>
    where V: InputValue;
}

pub trait AttrBuilder: InputBuilder {
    fn free_attr<V>(&mut self, name: &str, config: config::Attr) -> Result<BoxedFreeAttrDecl<V>>
    where V: AttrValue + DeserializeOwned + MaybeInputValue;

    fn bound_attr<V>(&mut self, name: &str, config: config::Attr) -> Result<BoxedBoundAttrDecl<V>>
    where V: AttrValue + DeserializeOwned + MaybeInputValue + Bounded;
}

pub trait NodeBuilder: AttrBuilder {
    fn node(&mut self, name: &str, config: config::Node) -> Result<NodeHandle<BoxedNodeDecl>>;
}

pub trait OutputBuilder {}

pub trait Registries<B>
where B: ?Sized
{
    type FreeAttr<V>: Registry<BoxedFreeAttrDecl<V>, B>;
    type BoundAttr<V>: Registry<BoxedBoundAttrDecl<V>, B>;
    type Node: Registry<BoxedNodeDecl, B>;
    type Output: Registry<BoxedOutputDecl, B>;
}

pub struct Builder<Registries>
where Registries: ?Sized
{
    scene: Scene,
    registry: PhantomData<Registries>,
}

impl<Registries> Builder<Registries> {
    pub fn new() -> Self {
        let scene = Scene::new();

        return Self {
            scene,
            registry: PhantomData,
        };
    }

    pub fn build(self) -> Scene {
        return self.scene;
    }
}

impl<Registries> Builder<Registries>
where Registries: self::Registries<Self> + ?Sized
{
    pub fn node(&mut self, name: &str, config: config::Node) -> Result<NodeHandle<BoxedNodeDecl>> {
        return NodeBuilder::node(self, name, config);
    }

    pub fn output(&mut self, config: config::Output) -> Result<BoxedOutputDecl> {
        let factory =
            Registries::Output::lookup(&config.kind).ok_or_else(|| anyhow!("Unknown output type: {}", config.kind))?;

        let decl = factory
            .produce(config.config, self)
            .with_context(|| format!("Failed to build output: (type={})", config.kind))?;

        return Ok(decl);
    }
}

impl<Registries> InputBuilder for Builder<Registries>
where Registries: self::Registries<Self> + ?Sized
{
    fn input<V>(&mut self, config: config::Input) -> Result<InputHandle<V>>
    where V: InputValue {
        let decl =
            self.scene.input(&config.input).with_context(|| format!("Failed to build input: {}", config.input))?;

        return Ok(decl);
    }
}

impl<Registries> AttrBuilder for Builder<Registries>
where Registries: self::Registries<Self> + ?Sized
{
    fn free_attr<V>(&mut self, name: &str, config: config::Attr) -> Result<BoxedFreeAttrDecl<V>>
    where V: AttrValue + DeserializeOwned + MaybeInputValue {
        match config {
            config::Attr::Attr {
                kind,
                config,
            } => {
                let factory =
                    Registries::FreeAttr::lookup(&kind).ok_or_else(|| anyhow!("Unknown attribute type: {}", kind))?;

                let decl = factory
                    .produce(config, self)
                    .with_context(|| format!("Failed to build attribute: (type={}) @{}", kind, name))?;

                return Ok(decl);
            }

            config::Attr::Input {
                input,
                initial,
            } => {
                return V::free_attr_input(self, input, initial);
            }

            config::Attr::Fixed(value) => {
                let value = V::deserialize(value)?;
                let attr = V::fixed(value);
                return Ok(Box::new(attr));
            }
        }
    }

    fn bound_attr<V>(&mut self, name: &str, config: config::Attr) -> Result<BoxedBoundAttrDecl<V>>
    where V: AttrValue + DeserializeOwned + MaybeInputValue + Bounded {
        match config {
            config::Attr::Attr {
                kind,
                config,
            } => {
                let factory =
                    Registries::BoundAttr::lookup(&kind).ok_or_else(|| anyhow!("Unknown attribute type: {}", kind))?;

                let decl = factory
                    .produce(config, self)
                    .with_context(|| format!("Failed to build attribute: (type={}) @{}", kind, name))?;

                return Ok(decl);
            }

            config::Attr::Input {
                input,
                initial,
            } => {
                return V::bound_attr_input(self, input, initial);
            }

            config::Attr::Fixed(value) => {
                let value = V::deserialize(value)?;
                let attr = V::fixed(value);
                return Ok(Box::new(attr));
            }
        }
    }
}

impl<Registries> NodeBuilder for Builder<Registries>
where Registries: self::Registries<Self> + ?Sized
{
    fn node(&mut self, name: &str, config: config::Node) -> Result<NodeHandle<BoxedNodeDecl>> {
        let factory =
            Registries::Node::lookup(&config.kind).ok_or_else(|| anyhow!("Unknown node type: {}", config.kind))?;

        let decl = factory
            .produce(config.config, self)
            .with_context(|| format!("Failed to build node: {} (type={}) @{}", config.name, config.kind, name))?;

        return self.scene.node(&config.name, decl);
    }
}

impl<Registries> OutputBuilder for Builder<Registries> where Registries: self::Registries<Self> + ?Sized {}

pub trait MaybeInputValue {
    fn free_attr_input<Registries>(
        builder: &mut Builder<Registries>,
        config: config::Input,
        initial: config::Anything,
    ) -> Result<BoxedFreeAttrDecl<Self>>
    where
        Registries: self::Registries<Builder<Registries>> + ?Sized;

    fn bound_attr_input<Registries>(
        builder: &mut Builder<Registries>,
        config: config::Input,
        initial: config::Anything,
    ) -> Result<BoxedBoundAttrDecl<Self>>
    where
        Registries: self::Registries<Builder<Registries>> + ?Sized;
}

impl<T: AttrValue> MaybeInputValue for T {
    default fn free_attr_input<Registries>(
        _builder: &mut Builder<Registries>,
        _config: config::Input,
        _initial: Value,
    ) -> Result<BoxedFreeAttrDecl<Self>>
    where
        Registries: self::Registries<Builder<Registries>> + ?Sized,
    {
        bail!("Can not create input for type: {}", type_name::<T>());
    }

    default fn bound_attr_input<Registries>(
        _builder: &mut Builder<Registries>,
        _config: config::Input,
        _initial: Value,
    ) -> Result<BoxedBoundAttrDecl<Self>>
    where
        Registries: self::Registries<Builder<Registries>> + ?Sized,
    {
        bail!("Can not create input for type: {}", type_name::<T>());
    }
}

impl MaybeInputValue for bool {
    fn free_attr_input<Registries>(
        builder: &mut Builder<Registries>,
        config: config::Input,
        initial: Value,
    ) -> Result<BoxedFreeAttrDecl<Self>>
    where
        Registries: self::Registries<Builder<Registries>> + ?Sized,
    {
        let initial = Self::deserialize(initial)?;

        let input = builder.input::<Self>(config)?;
        let attr = input.attr(initial);

        return Ok(Box::new(attr));
    }

    fn bound_attr_input<Registries>(
        builder: &mut Builder<Registries>,
        config: config::Input,
        initial: Value,
    ) -> Result<BoxedBoundAttrDecl<Self>>
    where
        Registries: self::Registries<Builder<Registries>> + ?Sized,
    {
        let initial = Self::deserialize(initial)?;

        let input = builder.input::<Self>(config)?;
        let attr = input.attr(initial);

        return Ok(Box::new(attr));
    }
}

use anyhow::{anyhow, Context, Result};
use palette::rgb::Rgb;
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

use photonic::attr::{AsFixedAttr, Bounded};
use photonic::boxed::{Boxed, BoxedBoundAttrDecl, BoxedFreeAttrDecl, BoxedNodeDecl, BoxedOutputDecl};
use photonic::input::InputValue;
use photonic::scene::InputHandle;
use photonic::{input, AttrValue, NodeHandle, Scene};

use crate::config;
use crate::registry::Registry;

pub struct InputBuilder<'b, Reg: Registry>(&'b mut Builder<Reg>);

pub struct AttrBuilder<'b, Reg: Registry>(&'b mut Builder<Reg>);

pub struct NodeBuilder<'b, Reg: Registry>(&'b mut Builder<Reg>);

pub struct OutputBuilder<'b, Reg: Registry>(&'b mut Builder<Reg>);

impl<Reg: Registry> InputBuilder<'_, Reg> {
    pub fn input<I>(&mut self, config: config::Input) -> Result<InputHandle<I>>
    where I: InputValue {
        return self.0.input(config);
    }
}

impl<Reg: Registry> AttrBuilder<'_, Reg> {
    pub fn input<I>(&mut self, config: config::Input) -> Result<InputHandle<I>>
    where I: InputValue {
        return self.0.input(config);
    }

    pub fn free_attr<V>(&mut self, name: &str, config: config::Attr<V>) -> Result<BoxedFreeAttrDecl<V>>
    where V: AttrValue + input::Coerced + DeserializeOwned {
        return self.0.free_attr(name, config);
    }

    pub fn bound_attr<V>(&mut self, name: &str, config: config::Attr<V>) -> Result<BoxedBoundAttrDecl<V>>
    where V: AttrValue + input::Coerced + DeserializeOwned + Bounded {
        return self.0.bound_attr(name, config);
    }
}

impl<Reg: Registry> NodeBuilder<'_, Reg> {
    pub fn node(&mut self, name: &str, config: config::Node) -> Result<NodeHandle<BoxedNodeDecl<Rgb>>> {
        return self.0.node(name, config);
    }

    pub fn input<I>(&mut self, config: config::Input) -> Result<InputHandle<I>>
    where I: InputValue {
        return self.0.input(config);
    }

    pub fn free_attr<V>(&mut self, name: &str, config: config::Attr<V>) -> Result<BoxedFreeAttrDecl<V>>
    where V: AttrValue + input::Coerced + DeserializeOwned {
        return self.0.free_attr(name, config);
    }

    pub fn bound_attr<V>(&mut self, name: &str, config: config::Attr<V>) -> Result<BoxedBoundAttrDecl<V>>
    where V: AttrValue + input::Coerced + DeserializeOwned + Bounded {
        return self.0.bound_attr(name, config);
    }
}

impl<Reg: Registry> OutputBuilder<'_, Reg> {
    pub fn output(&mut self, config: config::Output) -> Result<BoxedOutputDecl> {
        return self.0.output(config);
    }
}

pub struct Builder<Reg> {
    scene: Scene,
    registry: PhantomData<Reg>,
}

impl<Reg: Registry> Default for Builder<Reg> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Reg: Registry> Builder<Reg> {
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

    fn input<I>(&mut self, config: config::Input) -> Result<InputHandle<I>>
    where I: InputValue {
        return self.scene.input(&config.input).with_context(|| format!("Failed to build input: {}", config.input));
    }

    fn free_attr<V>(&mut self, name: &str, config: config::Attr<V>) -> Result<BoxedFreeAttrDecl<V>>
    where V: AttrValue + input::Coerced + DeserializeOwned {
        match config {
            config::Attr::Attr {
                kind,
                config,
            } => {
                let factory =
                    Reg::free_attr::<Reg, V>(&kind).ok_or_else(|| anyhow!("Unknown attribute type: {}", kind))?;

                let decl = factory
                    .produce(config, AttrBuilder(self))
                    .with_context(|| format!("Failed to build attribute: (type={}) @{}", kind, name))?;

                return Ok(decl);
            }

            config::Attr::Input {
                input,
                initial,
            } => {
                let input: InputHandle<V::Input> =
                    self.input(input).with_context(|| format!("Failed to build input: @{}", name))?;
                return Ok(input.attr(initial).boxed());
            }

            config::Attr::Fixed(value) => {
                let attr = V::fixed(value);
                return Ok(Box::new(attr));
            }
        }
    }

    fn bound_attr<V>(&mut self, name: &str, config: config::Attr<V>) -> Result<BoxedBoundAttrDecl<V>>
    where V: AttrValue + input::Coerced + DeserializeOwned + Bounded {
        match config {
            config::Attr::Attr {
                kind,
                config,
            } => {
                let factory =
                    Reg::bound_attr::<Reg, V>(&kind).ok_or_else(|| anyhow!("Unknown attribute type: {}", kind))?;

                let decl = factory
                    .produce(config, AttrBuilder(self))
                    .with_context(|| format!("Failed to build attribute: (type={}) @{}", kind, name))?;

                return Ok(decl);
            }

            config::Attr::Input {
                input,
                initial,
            } => {
                let input: InputHandle<V::Input> =
                    self.input(input).with_context(|| format!("Failed to build input: @{}", name))?;
                return Ok(input.attr(initial).boxed());
            }

            config::Attr::Fixed(value) => {
                let attr = V::fixed(value);
                return Ok(Box::new(attr));
            }
        }
    }

    pub fn node(&mut self, name: &str, config: config::Node) -> Result<NodeHandle<BoxedNodeDecl<Rgb>>> {
        let factory = Reg::node::<Reg>(&config.kind).ok_or_else(|| anyhow!("Unknown node type: {}", config.kind))?;

        let decl = factory
            .produce(config.config, NodeBuilder(self))
            .with_context(|| format!("Failed to build node: {} (type={}) @{}", config.name, config.kind, name))?;

        return self.scene.node(&config.name, decl);
    }

    pub fn output(&mut self, config: config::Output) -> Result<BoxedOutputDecl> {
        let factory =
            Reg::output::<Reg>(&config.kind).ok_or_else(|| anyhow!("Unknown output type: {}", config.kind))?;

        let decl = factory
            .produce(config.config, OutputBuilder(self))
            .with_context(|| format!("Failed to build output: (type={})", config.kind))?;

        return Ok(decl);
    }
}

use std::marker::PhantomData;

use anyhow::{Context, format_err, Result};

use photonic_core::{color, InputHandle, NodeHandle, Scene};
use photonic_core::boxed::{
    BoxedBoundAttrDecl, BoxedNodeDecl, BoxedOutputDecl,
    BoxedUnboundAttrDecl,
};
use photonic_core::input::InputValue;

use crate::config;
use crate::model::{BoundAttrFactory, UnboundAttrFactory};
use crate::registry::{BoundAttrRegistry, NodeRegistry, OutputRegistry, Registry, UnboundAttrRegistry};

pub trait InputBuilder {
    fn input<V>(&mut self, config: config::Input) -> Result<InputHandle<V>>
        where
            V: InputValue;
}

pub trait AttrBuilder: InputBuilder {
    fn unbound_attr<V>(
        &mut self,
        name: &str,
        config: config::Attr,
    ) -> Result<BoxedUnboundAttrDecl<V>>
        where
            V: UnboundAttrFactory;

    fn bound_attr<V>(
        &mut self,
        name: &str,
        config: config::Attr,
    ) -> Result<BoxedBoundAttrDecl<V>>
        where
            V: BoundAttrFactory;
}

pub trait NodeBuilder: AttrBuilder {
    fn node(
        &mut self,
        name: &str,
        config: config::Node,
    ) -> Result<NodeHandle<BoxedNodeDecl<color::RGBColor>>>;
}

pub trait OutputBuilder {}

pub struct Builder<Registry: self::Registry> {
    scene: Scene,
    registry: PhantomData<Registry>,
}

// TODO: Restrict access to build function via traits
impl<Registry: self::Registry> Builder<Registry> {
    pub fn new(size: usize) -> Self {
        let scene = Scene::new(size);
        return Self {
            scene,
            registry: Default::default(),
        };
    }

    pub fn finish(self) -> Scene {
        return self.scene;
    }

    pub fn output(&mut self, config: config::Output) -> Result<BoxedOutputDecl<BoxedNodeDecl<color::RGBColor>>> {
        return Ok(
            Registry::Output::manufacture(&config.kind)
                .ok_or(format_err!("Unknown output type: {}", config.kind))?
                .produce(config.config, self)
                .context(format!("Failed to build output: (type={})", config.kind))?
        );
    }
}

impl<Registry: self::Registry> NodeBuilder for Builder<Registry> {
    fn node(
        &mut self,
        name: &str,
        config: config::Node,
    ) -> Result<NodeHandle<BoxedNodeDecl<color::RGBColor>>> {
        let decl =
            Registry::Node::manufacture(&config.kind)
                .ok_or(format_err!("Unknown node type: {}", config.kind))?
                .produce(config.config, self)
                .context(format!("Failed to build node: {} (type={}) @{}",
                                 config.name, config.kind, name))?;
        return self.scene.node(&config.name, decl);
    }
}

impl<Registry: self::Registry> AttrBuilder for Builder<Registry> {
    fn unbound_attr<V>(
        &mut self,
        name: &str,
        config: config::Attr,
    ) -> Result<BoxedUnboundAttrDecl<V>>
        where
            V: UnboundAttrFactory,
    {
        match config {
            config::Attr::Attr { kind, config } => {
                return Ok(
                    Registry::UnboundAttr::manufacture(&kind)
                        .ok_or(format_err!("Unknown unbound attribute type: {}", kind))?
                        .produce(config, self)
                        .context(format!("Failed to build attr: (type={}) @{}", kind, name))?
                );
            }
            config::Attr::Input { input, initial } => V::make_input(self, input, initial),
            config::Attr::Fixed(value) => V::make_fixed(self, value),
        }
    }

    fn bound_attr<V>(
        &mut self,
        name: &str,
        config: config::Attr,
    ) -> Result<BoxedBoundAttrDecl<V>>
        where
            V: BoundAttrFactory,
    {
        match config {
            config::Attr::Attr { kind, config } => {
                return Ok(
                    Registry::BoundAttr::manufacture(&kind)
                        .ok_or(format_err!("Unknown bound attribute type: {}", kind))?
                        .produce(config, self)
                        .context(format!("Failed to build attr: (type={}) @{}", kind, name))?
                );
            }
            config::Attr::Input { input, initial } => V::make_input(self, input, initial),
            config::Attr::Fixed(value) => V::make_fixed(self, value),
        }
    }
}

impl<Registry: self::Registry> InputBuilder for Builder<Registry> {
    fn input<V>(&mut self, config: config::Input) -> Result<InputHandle<V>>
        where
            V: InputValue,
    {
        return self.scene.input(&config.input);
    }
}

impl<Registry: self::Registry> OutputBuilder for Builder<Registry> {}

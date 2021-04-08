use std::sync::Arc;

use anyhow::{Context, Result};

use photonic_core::{color, InputHandle, Introspection, Loop, NodeHandle, Scene};
use photonic_core::boxed::{BoxedNode, BoxedOutput, BoxedNodeDecl, BoxedUnboundAttrDecl, BoxedBoundAttrDecl, BoxedOutputDecl};
use photonic_core::input::InputValue;

use crate::config;
use crate::model::{BoundAttrFactory, UnboundAttrFactory};
use crate::registry::{BoundAttrRegistry, NodeRegistry, OutputRegistry, UnboundAttrRegistry};

pub struct Builder {
    scene: Scene,
}

// TODO: Restrict access to build function via traits
impl Builder {
    pub fn new(size: usize) -> Self {
        let scene = Scene::new(size);
        return Self { scene };
    }

    #[allow(clippy::type_complexity)]
    pub fn build(
        config: config::Scene,
    ) -> Result<(Loop<BoxedNode<color::RGBColor>, BoxedOutput<color::RGBColor>>, Arc<Introspection>)> {
        let mut builder = Self::new(config.size);

        let root = builder.node("root", config.root)?;
        let output = builder.output(config.output)?;

        return builder.scene.run(root, output);
    }

    pub fn node(
        &mut self,
        name: &str,
        config: config::Node,
    ) -> Result<NodeHandle<BoxedNodeDecl<color::RGBColor>>> {
        let decl =
            NodeRegistry::manufacture(&config.kind, config.config, self).context(format!(
                "Failed to build node: {} (type={}) @{}",
                config.name, config.kind, name
            ))?;
        return self.scene.node(&config.name, decl);
    }

    pub fn unbound_attr<V>(
        &mut self,
        name: &str,
        config: config::Attr,
    ) -> Result<BoxedUnboundAttrDecl<V>>
    where
        V: UnboundAttrFactory,
    {
        match config {
            config::Attr::Attr { kind, config } => {
                return UnboundAttrRegistry::manufacture(&kind, config, self)
                    .context(format!("Failed to build attr: (type={}) @{}", kind, name))
            }
            config::Attr::Input { input, initial } => V::make_input(self, input, initial),
            config::Attr::Fixed(value) => V::make_fixed(self, value),
        }
    }

    pub fn bound_attr<V>(
        &mut self,
        name: &str,
        config: config::Attr,
    ) -> Result<BoxedBoundAttrDecl<V>>
    where
        V: BoundAttrFactory,
    {
        match config {
            config::Attr::Attr { kind, config } => {
                return BoundAttrRegistry::manufacture(&kind, config, self)
                    .context(format!("Failed to build attr: (type={}) @{}", kind, name))
            }
            config::Attr::Input { input, initial } => V::make_input(self, input, initial),
            config::Attr::Fixed(value) => V::make_fixed(self, value),
        }
    }

    pub fn input<V>(&mut self, config: config::Input) -> Result<InputHandle<V>>
    where
        V: InputValue,
    {
        return self.scene.input(&config.input);
    }

    pub fn output(
        &mut self,
        config: config::Output,
    ) -> Result<BoxedOutputDecl<color::RGBColor>> {
        return OutputRegistry::manufacture(&config.kind, config.config, self)
            .context(format!("Failed to build output: (type={})", config.kind));
    }
}

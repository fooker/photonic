use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use std::{future, ops};

use anyhow::{bail, Context, Result};
use futures::future::SelectAll;
use futures::FutureExt;
use palette::FromColor;

use crate::arena::{Arena, Ref, Slice};
use crate::attr::{AttrValue, Bounded, Bounds};
use crate::decl::{BoundAttrDecl, FreeAttrDecl, NodeDecl, OutputDecl};
use crate::input::{Input, InputSink, InputValue};
use crate::interface::{AttrInfoBuilder, InputInfoBuilder, Interface, Introspection, NodeInfoBuilder};
use crate::utils::{FrameStats, FrameTimer};
use crate::{Buffer, BufferReader, Node, Output};

pub struct RenderContext<'ctx> {
    /// Duration since last update
    pub duration: Duration,

    nodes: Slice<'ctx, dyn NodeHolder>,
}

impl<Node> ops::Index<NodeRef<Node>> for RenderContext<'_>
where Node: self::Node
{
    type Output = NodeContainer<Node>;

    fn index(&self, index: NodeRef<Node>) -> &Self::Output {
        return &self.nodes[index.node];
    }
}

#[derive(Debug)]
pub struct NodeHandle<Decl>
where Decl: NodeDecl
{
    /// The scene-wide unique name of the node
    pub name: String,

    /// The declaration of the node
    pub decl: Decl,
}

#[cfg(feature = "boxed")]
impl<Decl> NodeHandle<Decl>
where Decl: NodeDecl + 'static
{
    pub fn boxed<E>(self) -> NodeHandle<crate::boxed::BoxedNodeDecl<E>>
    where E: Default + Copy + palette::convert::FromColorUnclamped<<<Decl as NodeDecl>::Node as Node>::Element> + 'static
    {
        return NodeHandle {
            name: self.name,
            decl: crate::boxed::Boxed::boxed(self.decl),
        };
    }
}

#[derive(Debug)]
pub struct NodeContainer<Node>
where Node: self::Node
{
    node: Node,
    buffer: Buffer<Node::Element>,
}

impl<Node> NodeContainer<Node>
where Node: self::Node
{
    pub fn build(builder: &NodeBuilder, node: Node) -> Result<Self> {
        let buffer = Buffer::with_default(builder.size);

        return Ok(Self {
            node,
            buffer,
        });
    }
}

impl<Node> BufferReader for NodeContainer<Node>
where Node: self::Node
{
    type Element = Node::Element;

    fn get(&self, index: usize) -> Self::Element {
        return *self.buffer.get(index);
    }

    fn size(&self) -> usize {
        return self.buffer.size();
    }
}

impl<Node> BufferReader for &NodeContainer<Node>
where Node: self::Node
{
    type Element = Node::Element;

    fn get(&self, index: usize) -> Self::Element {
        return *self.buffer.get(index);
    }

    fn size(&self) -> usize {
        return self.buffer.size();
    }
}

trait NodeHolder {
    fn update(&mut self, ctx: &RenderContext) -> Result<()>;
}

impl<Node> NodeHolder for NodeContainer<Node>
where Node: self::Node
{
    fn update(&mut self, ctx: &RenderContext) -> Result<()> {
        return self.node.update(ctx, &mut self.buffer);
    }
}

#[derive(Debug)]
pub struct NodeRef<Node>
where Node: self::Node + 'static // TODO: Is static required?
{
    node: Ref<NodeContainer<Node>, dyn NodeHolder>,
}

impl<Node> Clone for NodeRef<Node>
where Node: self::Node
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<Node> Copy for NodeRef<Node> where Node: self::Node {}

/// Handle to a [`Input`].
///
/// The handle represents an input registered in a [`Scene`] and can be used to get the real thing
/// while manifesting the scene.
#[derive(Debug)]
pub struct InputHandle<V>
where V: InputValue
{
    /// The scene-wide unique name of the input
    pub name: String,

    input: Input<V>,
}

impl<V> InputHandle<V>
where V: InputValue
{
    /// Returns a sink into the input represented by this handle.
    pub fn sink(&self) -> InputSink {
        return self.input.sink().into();
    }
}

/// Declaration of a scene.
///
/// This is used to declare nodes, attributes and inputs.
pub struct Scene {}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

impl Scene {
    /// Create a new scene with a given size.
    pub fn new() -> Self {
        return Self {};
    }

    /// Declares a new node in the scene.
    ///
    /// The given name must be unique over all nodes in the scene.
    ///
    /// The returned handle represents the node in the scene and can be used to reference the node
    /// in another node.
    pub fn node<Decl>(&mut self, name: &str, decl: Decl) -> Result<NodeHandle<Decl>>
    where Decl: NodeDecl {
        return Ok(NodeHandle {
            name: name.to_owned(),
            decl,
        });
    }

    /// Declares a input in the scene.
    ///
    /// The given name must be unique over all inputs in the scene.
    ///
    /// The returned handle represents the input in the scene and can be used to reference the input
    /// in other nodes and attributes.
    pub fn input<V>(&mut self, name: &str) -> Result<InputHandle<V>>
    where V: InputValue {
        return Ok(InputHandle {
            name: name.to_owned(),
            input: Input::new(),
        });
    }

    /// Create a driver for the scene.
    ///
    /// This is a termination method for the scene object. It consumes the scene and combines the
    /// root node of the node tree with an [`Output`] to render the animation to a target.
    ///
    /// The returned driver is used to run the scene in a loop.
    /// In addition, an [`Introspection`] is returned for the declared scene.
    // TODO: Find better naming
    #[allow(clippy::type_complexity)]
    pub async fn run<Node, Output>(
        self,
        root: NodeHandle<Node>,
        decl: Output,
    ) -> Result<Loop<Node::Node, Output::Output>>
    where
        Node: NodeDecl,
        Output: OutputDecl,
        <Output::Output as self::Output>::Element: FromColor<<Node::Node as self::Node>::Element>,
        <Node as NodeDecl>::Node: 'static,
        <<Node as NodeDecl>::Node as self::Node>::Element: Default, // TODO: Remove this constraint
    {
        let output = decl.materialize().await.context("Failed to materialize output")?;

        // Materialize the node tree using a builder tracking the info object creation
        let (scene, root) = SceneBuilder::build(output.size(), root).await.context("Failed to build scene")?;

        let introspection = Introspection::with(scene.root);
        introspection.log();

        return Ok(Loop {
            nodes: scene.nodes,
            root,
            output,
            stats: FrameStats::default(),
            introspection,
            servers: Vec::new(),
        });
    }
}

/// The rendering loop.
///
/// The rendering loop updates a scene and all its elements and then renders the root node to the
/// output.
pub struct Loop<Node, Output>
where
    Node: self::Node + 'static, // TODO: Is static required?
    Output: self::Output,
    Output::Element: FromColor<Node::Element>,
{
    nodes: Arena<dyn NodeHolder>,

    root: NodeRef<Node>,
    output: Output,

    stats: FrameStats,

    pub introspection: Arc<Introspection>,

    servers: Vec<Pin<Box<dyn Future<Output = Result<()>>>>>,
}

impl<Node, Output> Loop<Node, Output>
where
    Node: self::Node + 'static,
    Output: self::Output,
    Output::Element: FromColor<Node::Element> + Copy,
{
    /// Constantly run the render loop.
    ///
    /// The loop is driven by this function at the given rate.
    pub async fn run(mut self, fps: usize) -> Result<()> {
        // Wait for any server to finish
        let mut servers: Pin<Box<dyn Future<Output = Result<()>>>> = if !self.servers.is_empty() {
            Box::pin(self.servers.into_iter().collect::<SelectAll<_>>().map(|(result, _, _)| result))
        } else {
            Box::pin(future::pending())
        };

        let mut timer = FrameTimer::new(fps);
        loop {
            let duration = tokio::select! {
                duration = timer.tick() => duration,

                result = &mut servers => {
                    return result.context("Failed to run servers")
                }
            };

            self.nodes
                .try_walk(|curr, tail| {
                    let ctx = RenderContext {
                        duration,
                        nodes: tail,
                    };

                    return curr.update(&ctx);
                })
                .context("Failed to update nodes")?;

            let root = &self.nodes.as_slice()[self.root.node];

            // Render node tree to output
            self.output
                .render(root.buffer.map(Output::Element::from_color))
                .await
                .context("Output failed to render")?;

            self.stats.update(duration);

            if let Some(stats) = self.stats.reset(fps) {
                eprintln!(
                    "Stats: min={:3.2}, max={:3.2}, avg={:3.2}",
                    stats.min_fps(),
                    stats.max_fps(),
                    stats.avg_fps(),
                );
            }
        }
    }

    pub fn serve(&mut self, name: &'static str, interface: impl Interface) {
        let interface = interface.listen(self.introspection.clone());
        let interface = interface.inspect(move |result| {
            if let Ok(()) = result {
                eprintln!("Server terminated: {name}");
            }
        });

        self.servers.push(Box::pin(interface));
    }
}

/// Builder used while building a scene from its definition.
///
/// **Note:** This is not used to build a [`Scene`] but to build the elements within such a scene.
// TODO: This is more like the build result, whereas the other builders are for context
pub struct SceneBuilder {
    /// Size of the scene
    pub size: usize,

    nodes: Arena<dyn NodeHolder>,

    root: NodeInfoBuilder,
}

pub struct NodeBuilder<'b> {
    nodes: &'b mut Arena<dyn NodeHolder>,

    /// Size of the scene
    pub size: usize,

    info: NodeInfoBuilder,
}

impl NodeBuilder<'_> {
    pub fn key(&self) -> &str {
        return &self.info.key;
    }

    pub fn kind(&self) -> &'static str {
        return self.info.kind;
    }

    pub fn name(&self) -> &str {
        return &self.info.name;
    }
}

pub struct AttrBuilder<'b> {
    nodes: &'b mut Arena<dyn NodeHolder>,

    /// Size of the scene
    pub size: usize,

    info: AttrInfoBuilder,
}

impl AttrBuilder<'_> {
    pub fn key(&self) -> &str {
        return &self.info.key;
    }

    pub fn kind(&self) -> &'static str {
        return self.info.kind;
    }
}

impl SceneBuilder {
    /// Create a node from its handle.
    pub async fn build<Node>(size: usize, root: NodeHandle<Node>) -> Result<(Self, NodeRef<Node::Node>)>
    where
        Node: NodeDecl,
        <Node as NodeDecl>::Node: 'static,
        <<Node as NodeDecl>::Node as self::Node>::Element: Default, // TODO: Remove this constraint
    {
        let mut nodes = Arena::new();

        let mut builder = NodeBuilder {
            size,

            nodes: &mut nodes,

            info: NodeInfoBuilder {
                key: "".to_string(),
                kind: Node::KIND,
                name: root.name,
                nodes: HashMap::new(),
                attrs: HashMap::new(),
            },
        };

        // TODO: Dedup with code from node-builder
        let node = Node::materialize(root.decl, &mut builder).await?;
        let info = builder.info;

        let buffer = Buffer::with_default(size);

        let node = nodes.append(NodeContainer {
            node,
            buffer,
        });

        eprintln!("✨ Materialized node {} ({:?})", info.name, node);

        let scene = SceneBuilder {
            size,
            nodes,
            root: info,
        };

        return Ok((scene, NodeRef {
            node,
        }));
    }
}

impl NodeBuilder<'_> {
    pub async fn node<Node>(&mut self, key: impl Into<String>, decl: NodeHandle<Node>) -> Result<NodeRef<Node::Node>>
    where
        Node: NodeDecl,
        <Node as NodeDecl>::Node: 'static,
        <<Node as NodeDecl>::Node as self::Node>::Element: Default, // TODO: Remove this constraint
    {
        return self.node_with_size(key, decl, self.size).await;
    }

    pub async fn node_with_size<Node>(
        &mut self,
        key: impl Into<String>,
        decl: NodeHandle<Node>,
        size: usize,
    ) -> Result<NodeRef<Node::Node>>
    where
        Node: NodeDecl,
        <Node as NodeDecl>::Node: 'static,
        <<Node as NodeDecl>::Node as self::Node>::Element: Default, // TODO: Remove this constraint
    {
        let key = key.into();

        let mut builder = NodeBuilder {
            size,

            nodes: self.nodes,

            info: NodeInfoBuilder {
                key: key.clone(),
                kind: Node::KIND,
                name: decl.name,
                nodes: HashMap::new(),
                attrs: HashMap::new(),
            },
        };

        let node = Node::materialize(decl.decl, &mut builder).await?;
        let info = builder.info;

        let buffer = Buffer::with_default(size);

        let node = self.nodes.append(NodeContainer {
            node,
            buffer,
        });

        eprintln!("✨ Materialized node {} ({:?})", info.name, node);

        if let Err(err) = self.info.nodes.try_insert(key, info) {
            bail!("Duplicated node: {}", err.entry.key())
        }

        return Ok(NodeRef {
            node,
        });
    }

    /// Create a bound attribute.
    ///
    /// The created attribute is registered as an attribute to the currently built node.
    pub fn bound_attr<V, Attr>(
        &mut self,
        key: impl Into<String>,
        decl: Attr,
        bounds: impl Into<Bounds<V>>,
    ) -> Result<Attr::Attr>
    where
        V: AttrValue + Bounded,
        Attr: BoundAttrDecl<V>,
    {
        let key = key.into();

        let bounds = bounds.into();

        let mut builder = AttrBuilder {
            nodes: self.nodes,
            size: self.size,
            info: AttrInfoBuilder {
                key: key.clone(),
                kind: Attr::KIND,
                value_type: V::TYPE,
                attrs: HashMap::new(),
                inputs: HashMap::new(),
            },
        };

        let attr = decl.materialize(bounds, &mut builder)?;

        if let Err(err) = self.info.attrs.try_insert(key, builder.info) {
            bail!("Duplicated attribute: {}", err.entry.key())
        }

        return Ok(attr);
    }

    /// Create a unbound attribute.
    ///
    /// The created attribute is registered as an attribute to the currently built node.
    // TODO: Rename to `free_attr`
    pub fn unbound_attr<V, Attr>(&mut self, key: impl Into<String>, decl: Attr) -> Result<Attr::Attr>
    where
        V: AttrValue,
        Attr: FreeAttrDecl<V>,
    {
        let key = key.into();

        let mut builder = AttrBuilder {
            nodes: self.nodes,
            size: self.size,
            info: AttrInfoBuilder {
                key: key.clone(),
                kind: Attr::KIND,
                value_type: V::TYPE,
                attrs: HashMap::new(),
                inputs: HashMap::new(),
            },
        };

        let attr = decl.materialize(&mut builder)?;

        if let Err(err) = self.info.attrs.try_insert(key, builder.info) {
            bail!("Duplicated attribute: {}", err.entry.key())
        }

        return Ok(attr);
    }
}

impl AttrBuilder<'_> {
    /// Create a bound child-attribute from its handle.
    ///
    /// The created attribute is registered as an attribute to the currently built node.
    pub fn bound_attr<V, Attr>(
        &mut self,
        key: impl Into<String>,
        decl: Attr,
        bounds: impl Into<Bounds<V>>,
    ) -> Result<Attr::Attr>
    where
        V: AttrValue + Bounded,
        Attr: BoundAttrDecl<V>,
    {
        let key = key.into();

        let bounds = bounds.into();

        let mut builder = AttrBuilder {
            nodes: self.nodes,
            size: self.size,
            info: AttrInfoBuilder {
                key: key.clone(),
                kind: Attr::KIND,
                value_type: V::TYPE,
                attrs: HashMap::new(),
                inputs: HashMap::new(),
            },
        };

        let attr = decl.materialize(bounds, &mut builder)?;

        if let Err(err) = self.info.attrs.try_insert(key, builder.info) {
            bail!("Duplicated attribute: {}", err.entry.key())
        }

        return Ok(attr);
    }

    /// Create a unbound child-attribute from its handle.
    ///
    /// The created attribute is registered as an attribute to the currently built node.
    pub fn unbound_attr<V, Attr>(&mut self, key: impl Into<String>, decl: Attr) -> Result<Attr::Attr>
    where
        V: AttrValue,
        Attr: FreeAttrDecl<V>,
    {
        let key = key.into();

        let mut builder = AttrBuilder {
            nodes: self.nodes,
            size: self.size,
            info: AttrInfoBuilder {
                key: key.clone(),
                kind: Attr::KIND,
                value_type: V::TYPE,
                attrs: HashMap::new(),
                inputs: HashMap::new(),
            },
        };

        let attr = decl.materialize(&mut builder)?;

        if let Err(err) = self.info.attrs.try_insert(key, builder.info) {
            bail!("Duplicated attribute: {}", err.entry.key())
        }

        return Ok(attr);
    }

    /// Create an input from its handle.
    ///
    /// The created input is registered as an input to the currently built node.
    pub fn input<V>(&mut self, key: impl Into<String>, input: InputHandle<V>) -> Result<Input<V>>
    where V: InputValue {
        let key = key.into();

        let sink = input.sink();

        let info = InputInfoBuilder {
            key: key.clone(),
            name: input.name,
            value_type: V::TYPE,
            sink,
        };

        if let Err(err) = self.info.inputs.try_insert(key, info) {
            bail!("Duplicated attribute: {}", err.entry.key())
        }

        return Ok(input.input);
    }
}

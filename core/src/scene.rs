extern crate anyhow;

use std::collections::HashMap;
use std::future::Future;
use std::ops;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use palette::FromColor;

use crate::{AttrInfo, Buffer, BufferReader, InputInfo, NodeInfo};
use crate::{Node, Output};
use crate::arena::{Arena, Ref, Slice};
use crate::attr::{Attr, AttrValue};
use crate::attr::Bounds;
use crate::decl::{BoundAttrDecl, FreeAttrDecl, NodeDecl, OutputDecl};
use crate::input::{Input, InputSink, InputValue};
use crate::interface::{Interface, Introspection};
use crate::utils::{FrameStats, FrameTimer};

pub struct Context<'ctx>
{
    /// Duration since last update
    pub duration: Duration,

    nodes: Slice<'ctx, dyn NodeHolder>,
}

impl<Node> ops::Index<&NodeRef<Node>> for Context<'_>
    where Node: self::Node,
{
    type Output = Buffer<Node::Element>;

    fn index(&self, index: &NodeRef<Node>) -> &Self::Output {
        return &self.nodes[&index.node].buffer;
    }
}

pub struct NodeHandle<Decl>
    where Decl: NodeDecl,
{
    /// The scene-wide unique name of the node
    pub name: String,

    /// The declaration of the node
    pub decl: Decl,
}

struct NodeContainer<Node>
    where Node: self::Node,
{
    node: Node,
    buffer: Buffer<Node::Element>,
}

trait NodeHolder {
    fn update(&mut self, ctx: &Context) -> Result<()>;
}

impl<Node> NodeHolder for NodeContainer<Node>
    where Node: self::Node,
{
    fn update(&mut self, ctx: &Context) -> Result<()> {
        return self.node.update(ctx, &mut self.buffer);
    }
}

pub struct NodeRef<Node>
    where Node: self::Node + 'static, // TODO: Is static required?
{
    node: Ref<NodeContainer<Node>, dyn NodeHolder>,
}

/// Handle to a [`Input`].
///
/// The handle represents a input registered in a [`Scene'] and can be used to get the real thing
/// while manifesting the scene.
pub struct InputHandle<V>
    where V: InputValue,
{
    /// The scene-wide unique name of the input
    pub name: String,

    input: Input<V>,
}

impl<V> InputHandle<V>
    where V: InputValue,
{
    /// Returns a sink into the input represented by this handle.
    pub fn sink(&self) -> InputSink {
        return self.input.sink().into();
    }
}

/// Declaration of a scene.
///
/// This is used to declare nodes, attributes and inputs.
pub struct Scene {
    size: usize,
}

impl Scene {
    /// Create a new scene with a given size.
    ///
    /// The size represents the number of lights in the scene.
    pub fn new(size: usize) -> Self {
        return Self {
            size,
        };
    }

    /// Returns the size of the scene.
    pub fn size(&self) -> usize {
        return self.size;
    }

    /// Declares a new node in the scene.
    ///
    /// The given name must be unique over all nodes in the scene.
    ///
    /// The returned handle represents the node in the scene and can be used to reference the node
    /// in another node.
    pub fn node<Decl>(&mut self, name: &str, decl: Decl) -> Result<NodeHandle<Decl>>
        where
            Decl: NodeDecl,
    {
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
        where V: InputValue,
    {
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
            <<Node as NodeDecl>::Node as self::Node>::Element: Default, // TODO: Remove this constraint
    {
        // Materialize the node tree using a builder tracking the info object creation
        let (scene, root) = SceneBuilder::build(self.size, root).await?;

        let introspection = Introspection::with(scene.root);
        introspection.log();

        return Ok(Loop {
            nodes: scene.nodes,
            root,
            output: decl.materialize(self.size()).await?,
            stats: FrameStats::default(),
            introspection,
        });
    }
}

/// The rendering loop.
///
/// The rendering loop updates a scene and all its elements and then renders the root node to the
/// output.
pub struct Loop<Node, Output>
    where Node: self::Node + 'static, // TODO: Is static required?
          Output: self::Output,
          Output::Element: FromColor<Node::Element>,
{
    nodes: Arena<dyn NodeHolder>,

    root: NodeRef<Node>,
    output: Output,

    stats: FrameStats,

    pub introspection: Arc<Introspection>,
}

impl<'a, Node, Output> Loop<Node, Output>
    where
        Node: self::Node + 'static,
        Output: self::Output,
        Output::Element: FromColor<Node::Element> + Copy,
{
    /// Update and render a single frame.
    ///
    /// The passed `duration` is the time passed since the last call to this function.
    pub async fn frame(&mut self, duration: Duration) -> Result<()> {
        self.nodes.try_walk(|curr, tail| {
            let ctx = Context {
                duration,
                nodes: tail,
            };

            return curr.update(&ctx);
        })?;

        let root = &self.nodes.as_slice()[&self.root.node];

        // Render node tree to output
        self.output.render(root.buffer.map(Output::Element::from_color)).await?;

        self.stats.update(duration);

        return Ok(());
    }

    /// Constantly run the render loop.
    ///
    /// The loop is driven by this function at the given rate.
    pub async fn run(mut self, fps: usize) -> Result<()> {
        let mut timer = FrameTimer::new(fps);
        loop {
            let duration = timer.tick().await;

            self.frame(duration).await?;

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

    pub fn serve(&self, interface: impl Interface) -> impl Future<Output=Result<()>> {
        return interface.listen(self.introspection.clone());
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

    root: Arc<NodeInfo>,
}

pub struct NodeBuilder<'b> {
    nodes: &'b mut Arena<dyn NodeHolder>,

    /// Size of the scene
    pub size: usize,

    info: NodeInfo,
}

impl<'b> NodeBuilder<'b> {
    pub fn kind(&self) -> &'static str {
        return &self.info.kind;
    }

    pub fn name(&self) -> &str {
        return &self.info.name;
    }
}

pub struct AttrBuilder<'b> {
    nodes: &'b mut Arena<dyn NodeHolder>,

    /// Size of the scene
    pub size: usize,

    info: AttrInfo,
}

impl SceneBuilder {
    /// Create a node from its handle.
    pub async fn build<Node>(size: usize, root: NodeHandle<Node>) -> Result<(Self, NodeRef<Node::Node>)>
        where Node: NodeDecl,
              <<Node as NodeDecl>::Node as self::Node>::Element: Default, // TODO: Remove this constraint
    {
        let mut nodes = Arena::new();

        let mut builder = NodeBuilder {
            size,

            nodes: &mut nodes,

            info: NodeInfo {
                kind: Node::Node::KIND,
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
            root: Arc::new(info),
        };

        return Ok((scene, NodeRef {
            node
        }));
    }
}

impl NodeBuilder<'_> {
    pub async fn node<Node>(&mut self, key: &'static str, decl: NodeHandle<Node>) -> Result<NodeRef<Node::Node>>
        where Node: NodeDecl,
              <<Node as NodeDecl>::Node as self::Node>::Element: Default, // TODO: Remove this constraint
    {
        let mut builder = NodeBuilder {
            size: self.size,

            nodes: &mut self.nodes,

            info: NodeInfo {
                kind: Node::Node::KIND,
                name: decl.name,
                nodes: HashMap::new(),
                attrs: HashMap::new(),
            },
        };

        let node = Node::materialize(decl.decl, &mut builder).await?;
        let info = builder.info;

        let buffer = Buffer::with_default(self.size);

        let node = self.nodes.append(NodeContainer {
            node,
            buffer,
        });

        eprintln!("✨ Materialized node {} ({:?})", info.name, node);

        self.info.nodes.insert(key, Arc::new(info));

        return Ok(NodeRef {
            node
        });
    }

    /// Create a bound attribute.
    ///
    /// The created attribute is registered as an attribute to the currently built node.
    pub fn bound_attr<Attr>(&mut self, name: &'static str, decl: Attr, bounds: impl Into<Bounds<Attr::Value>>) -> Result<Attr::Attr>
        where Attr: BoundAttrDecl,
    {
        let bounds = bounds.into();

        let mut builder = AttrBuilder {
            nodes: self.nodes,
            size: self.size,
            info: AttrInfo {
                kind: Attr::Attr::KIND,
                value_type: Attr::Value::TYPE,
                attrs: HashMap::new(),
                inputs: HashMap::new(),
            },
        };

        let attr = decl.materialize(bounds, &mut builder)?;

        let info = Arc::new(builder.info);
        self.info.attrs.insert(name, info);

        return Ok(attr);
    }

    /// Create a unbound attribute.
    ///
    /// The created attribute is registered as an attribute to the currently built node.
    // TODO: Rename to `free_attr`
    pub fn unbound_attr<Attr>(&mut self, name: &'static str, decl: Attr) -> Result<Attr::Attr>
        where Attr: FreeAttrDecl,
    {
        let mut builder = AttrBuilder {
            nodes: self.nodes,
            size: self.size,
            info: AttrInfo {
                kind: Attr::Attr::KIND,
                value_type: Attr::Value::TYPE,
                attrs: HashMap::new(),
                inputs: HashMap::new(),
            },
        };

        let attr = decl.materialize(&mut builder)?;

        let info = Arc::new(builder.info);
        self.info.attrs.insert(name, info);

        return Ok(attr);
    }
}

impl<'b> AttrBuilder<'b> {
    /// Create a bound child-attribute from its handle.
    ///
    /// The created attribute is registered as an attribute to the currently built node.
    pub fn bound_attr<Attr>(&mut self, name: &'static str, decl: Attr, bounds: impl Into<Bounds<Attr::Value>>) -> Result<Attr::Attr>
        where Attr: BoundAttrDecl,
    {
        let bounds = bounds.into();

        let mut builder = AttrBuilder {
            nodes: self.nodes,
            size: self.size,
            info: AttrInfo {
                kind: Attr::Attr::KIND,
                value_type: Attr::Value::TYPE,
                attrs: HashMap::new(),
                inputs: HashMap::new(),
            },
        };

        let attr = decl.materialize(bounds, &mut builder)?;

        let info = Arc::new(builder.info);
        self.info.attrs.insert(name, info);

        return Ok(attr);
    }

    /// Create a unbound child-attribute from its handle.
    ///
    /// The created attribute is registered as an attribute to the currently built node.
    pub fn unbound_attr<Attr>(&mut self, name: &'static str, decl: Attr) -> Result<Attr::Attr>
        where Attr: FreeAttrDecl,
    {
        let mut builder = AttrBuilder {
            nodes: self.nodes,
            size: self.size,
            info: AttrInfo {
                kind: Attr::Attr::KIND,
                value_type: Attr::Value::TYPE,
                attrs: HashMap::new(),
                inputs: HashMap::new(),
            },
        };

        let attr = decl.materialize(&mut builder)?;

        let info = Arc::new(builder.info);
        self.info.attrs.insert(name, info);

        return Ok(attr);
    }

    /// Create a input from its handle.
    ///
    /// The created input is registered as an input to the currently built node.
    pub fn input<V>(&mut self, name: &'static str, input: InputHandle<V>) -> Result<Input<V>>
        where V: InputValue,
    {
        let sink = input.sink();

        let info = InputInfo {
            name: input.name,
            value_type: V::TYPE,
            sink,
        };

        let info = Arc::new(info);
        self.info.inputs.insert(name, info);

        return Ok(input.input);
    }
}

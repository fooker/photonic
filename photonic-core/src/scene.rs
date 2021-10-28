use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;

use crate::attr::{Attr, AttrValue, BoundAttrDecl, Bounds, UnboundAttrDecl};
use crate::input::{Input, InputValue, Sink};
use crate::interface::{AttrInfo, InputInfo, Introspection, NodeInfo};
use crate::node::{MapNodeDecl, Node, NodeDecl};
use crate::output::{Output, OutputDecl};
use crate::utils::{FrameStats, FrameTimer};

pub struct SceneBuilder {
    /// The size of the scene
    pub size: usize,
}

impl SceneBuilder {
    pub fn node<Node>(&mut self, decl: NodeHandle<Node>) -> Result<(NodeInfo, Node::Target)>
    where
        Node: NodeDecl,
    {
        let mut builder = NodeBuilder {
            scene: self,
            name: decl.name.clone(),
            info: NodeInfo {
                kind: Node::Target::KIND,
                name: decl.name.clone(),
                nodes: HashMap::new(),
                attrs: HashMap::new(),
            },
        };

        let node = Node::materialize(decl.decl, builder.scene.size, &mut builder)?;

        return Ok((builder.info, node));
    }
}

pub struct NodeBuilder<'b> {
    /// The parent builder used to materialize the scene
    pub scene: &'b mut SceneBuilder,

    /// The name of the node to materialize
    pub name: String,

    info: NodeInfo,
}

impl<'b> NodeBuilder<'b> {
    pub fn node<Node>(&mut self, name: &str, decl: NodeHandle<Node>) -> Result<Node::Target>
    where
        Node: NodeDecl,
    {
        let (info, node) = self.scene.node(decl)?;

        // TODO: Assert for duplicated keys
        self.info.nodes.insert(name.to_owned(), Arc::new(info));

        return Ok(node);
    }

    pub fn bound_attr<Attr>(
        &mut self,
        name: &str,
        decl: Attr,
        bounds: impl Into<Bounds<Attr::Value>>,
    ) -> Result<Attr::Target>
    where
        Attr: BoundAttrDecl,
    {
        let bounds = bounds.into();

        let mut builder = AttrBuilder {
            node: self,
            info: AttrInfo {
                kind: Attr::Target::KIND,
                value_type: Attr::Value::TYPE,
                attrs: HashMap::new(),
                inputs: HashMap::new(),
            },
        };

        let attr = decl.materialize(bounds, &mut builder)?;

        let info = Arc::new(builder.info);
        self.info.attrs.insert(name.to_owned(), info);

        return Ok(attr);
    }

    pub fn unbound_attr<Attr>(&mut self, name: &str, decl: Attr) -> Result<Attr::Target>
    where
        Attr: UnboundAttrDecl,
    {
        let mut builder = AttrBuilder {
            node: self,
            info: AttrInfo {
                kind: Attr::Target::KIND,
                value_type: Attr::Value::TYPE,
                attrs: HashMap::new(),
                inputs: HashMap::new(),
            },
        };

        let attr = decl.materialize(&mut builder)?;

        let info = Arc::new(builder.info);
        self.info.attrs.insert(name.to_owned(), info);

        return Ok(attr);
    }
}

pub struct AttrBuilder<'b, 'p> {
    /// The parent builder used to materialize the node
    pub node: &'b mut NodeBuilder<'p>,

    info: AttrInfo,
}

impl<'b, 'p> AttrBuilder<'b, 'p> {
    pub fn bound_attr<Attr>(
        &mut self,
        name: &str,
        decl: Attr,
        bounds: impl Into<Bounds<Attr::Value>>,
    ) -> Result<Attr::Target>
    where
        Attr: BoundAttrDecl,
    {
        let bounds = bounds.into();

        let mut builder = AttrBuilder {
            node: self.node,
            info: AttrInfo {
                kind: Attr::Target::KIND,
                value_type: Attr::Value::TYPE,
                attrs: HashMap::new(),
                inputs: HashMap::new(),
            },
        };

        let attr = decl.materialize(bounds, &mut builder)?;

        let info = Arc::new(builder.info);
        self.info.attrs.insert(name.to_owned(), info);

        return Ok(attr);
    }

    pub fn unbound_attr<Attr>(&mut self, name: &str, decl: Attr) -> Result<Attr::Target>
    where
        Attr: UnboundAttrDecl,
    {
        let mut builder = AttrBuilder {
            node: self.node,
            info: AttrInfo {
                kind: Attr::Target::KIND,
                value_type: Attr::Value::TYPE,
                attrs: HashMap::new(),
                inputs: HashMap::new(),
            },
        };

        let attr = decl.materialize(&mut builder)?;

        let info = Arc::new(builder.info);
        self.info.attrs.insert(name.to_owned(), info);

        return Ok(attr);
    }

    pub fn input<V>(&mut self, name: &str, input: InputHandle<V>) -> Result<Input<V>>
    where
        V: InputValue,
    {
        let info = InputInfo {
            name: input.name,
            // kind: "", // TODO
            value_type: V::TYPE,
            sender: V::sender(input.input.sink()),
        };

        let info = Arc::new(info);
        self.info.inputs.insert(name.to_owned(), info);

        return Ok(input.input);
    }
}

pub struct NodeHandle<Decl>
where
    Decl: NodeDecl,
{
    /// The scene-wide unique name of the node
    pub name: String,

    /// The declaration of the node
    pub decl: Decl,
}

impl<Decl> NodeHandle<Decl>
where
    Decl: NodeDecl,
    Decl::Element: 'static,
{
    pub fn map<R, F>(self, f: F) -> NodeHandle<MapNodeDecl<Decl, F>>
    where
        F: Fn(Decl::Element) -> R + 'static,
        R: 'static,
    {
        return NodeHandle {
            name: self.name,
            decl: self.decl.map(f),
        };
    }
}

pub struct InputHandle<V>
where
    V: InputValue,
{
    /// The scene-wide unique name of the input
    pub name: String,

    input: Input<V>,
}

impl<V> InputHandle<V>
where
    V: InputValue,
{
    pub fn new(name: String) -> Self {
        return Self {
            name,
            input: Input::default(),
        };
    }

    /// Returns a sink into the input represented by this handle.
    pub fn sink(&self) -> Sink<V> {
        return self.input.sink();
    }
}

impl<V> From<InputHandle<V>> for Input<V>
where
    V: InputValue,
{
    fn from(handle: InputHandle<V>) -> Self {
        return handle.input;
    }
}

pub struct Scene {
    size: usize,
}

impl Scene {
    pub fn new(size: usize) -> Self {
        return Self {
            size,
        };
    }

    pub fn size(&self) -> usize {
        return self.size;
    }

    pub fn node<Node>(&mut self, name: &str, decl: Node) -> Result<NodeHandle<Node>>
    where
        Node: NodeDecl,
    {
        return Ok(NodeHandle {
            name: name.to_owned(),
            decl,
        });
    }

    pub fn input<V>(&mut self, name: &str) -> Result<InputHandle<V>>
    where
        V: InputValue,
    {
        return Ok(InputHandle::new(name.to_owned()));
    }

    #[allow(clippy::type_complexity)]
    pub fn run<Root, Output>(
        self,
        root: NodeHandle<Root>,
        decl: Output,
    ) -> Result<(Loop<Root::Target, Output::Target>, Arc<Introspection>)>
    where
        Root: NodeDecl,
        Output: OutputDecl<Root>,
    {
        let mut builder = SceneBuilder {
            size: self.size,
        };

        // Materialize the node tree using a builder tracking the info object creation
        let (root_info, root_node) = builder.node(root)?;

        let introspection = Introspection::with(Arc::new(root_info));

        return Ok((
            Loop {
                root: root_node,
                output: decl.materialize(self.size())?,
                stats: FrameStats::default(),
            },
            introspection,
        ));
    }
}

pub struct Loop<Root, Output> {
    root: Root,
    output: Output,

    stats: FrameStats,
}

impl<Root, Output> Loop<Root, Output>
where
    Root: self::Node,
    Output: self::Output<Root>,
{
    pub fn frame(&mut self, duration: Duration) -> Result<()> {
        self.root.update(duration)?;

        // Render node tree to output
        let render = self.root.render()?;
        self.output.render(render)?;

        self.stats.update(duration);

        return Ok(());
    }

    pub async fn run(mut self, fps: usize) -> Result<()> {
        let mut timer = FrameTimer::new(fps);

        loop {
            let duration = timer.next().await;

            self.frame(duration)?;

            if let Some(stats) = self.stats.reset(fps) {
                eprintln!(
                    "Stats: min={:3.2}, max={:3.2}, avg={:3.2}",
                    stats.min_fps(),
                    stats.max_fps(),
                    stats.avg_fps()
                );
            }
        }
    }
}

use std::borrow::Cow;
use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Duration;

use failure::Error;

use crate::attr::{Attr, AttrValue, BoundAttrDecl, Bounded, Bounds, UnboundAttrDecl};
use crate::input::{Input, InputValue};
use crate::interface::{AttrInfo, NodeInfo, Interface, Registry};
use crate::utils::{FrameStats, FrameTimer};
use std::collections::HashMap;

struct NodeArena {
    elements: Vec<Box<dyn NodeBase>>,
}

impl NodeArena {
    pub fn new() -> Self {
        return Self {
            elements: Vec::new(),
        };
    }

    pub fn insert<Node>(&mut self, name: String, node: Node) -> NodeRef<Node>
        where Node: self::Node + 'static {
        self.elements.push(Box::new(node));
        return NodeRef {
            name,
            index: self.elements.len() - 1,
            phantom: PhantomData::default(),
        };
    }

    pub fn resolve<'n, Node>(&self, r: &NodeRef<Node>) -> &'n Node
        where Node: self::Node {
        let element = self.elements[r.index].as_ref();
        return unsafe { &*(element as *const dyn NodeBase as *const Node) };
    }
}

pub struct NodeRef<Node> {
    /// The scene-wide unique name assigned to the node during declaration
    pub name: String,

    // Index of the element in the node arena
    index: usize,

    phantom: PhantomData<Node>,
}

pub struct Renderer<'a> {
    nodes: &'a NodeArena,
}

impl<'a> Renderer<'a> {
    pub fn render<'b, N, E>(&'b self, node: &'a NodeRef<N>) -> <N as RenderType<'a>>::Render
        where 'b: 'a,
              N: Node<Element=E> + 'b {
        self.nodes.resolve(node).render(self)
    }
}

pub trait Render {
    type Element;

    fn get(&self, index: usize) -> Self::Element;
}

impl<E, F> Render for F
    where F: Fn(usize) -> E {
    type Element = E;

    fn get(&self, index: usize) -> Self::Element {
        return self(index);
    }
}

pub trait NodeDecl {
    type Element;
    type Target: Node<Element=Self::Element> + 'static;

    fn materialize(self, size: usize, builder: &mut NodeBuilder) -> Result<Self::Target, Error>
        where Self::Target: std::marker::Sized;
}

pub trait RenderType<'a> {
    type Element;
    type Render: Render<Element=Self::Element> + 'a;
}

pub trait Node: for<'a> RenderType<'a> {
    const KIND: &'static str;

    fn update(&mut self, duration: &Duration);
    fn render<'a>(&'a self, renderer: &'a Renderer) -> <Self as RenderType<'a>>::Render;
}

pub trait OutputDecl {
    type Element;
    type Target: Output<Element=Self::Element>;

    fn materialize(self, size: usize) -> Result<Self::Target, Error>
        where Self::Target: std::marker::Sized;
}

pub trait Output {
    type Element;

    fn render<E: Into<Self::Element>>(&mut self, render: &dyn Render<Element=E>);
}

pub enum AttrPath<'p> {
    Root,
    Nested {
        parent: &'p AttrPath<'p>,
        name: String,
    },
}

impl<'p> AttrPath<'p> {
    pub fn to_vec(&self) -> Vec<String> {
        return match self {
            AttrPath::Root => Vec::new(),
            AttrPath::Nested { parent, name } => {
                let mut r = parent.to_vec();
                r.push(name.clone());
                r
            }
        };
    }
}

pub struct SceneBuilder {
    /// The size of the scene
    pub size: usize,

    // The arena used to build nodes and hand out node-refs
    nodes: NodeArena,

    // infos: Vec<Arc<NodeInfo>>,
}

impl SceneBuilder {
    pub fn node<Node>(&mut self, decl: NodeHandle<Node>) -> Result<(NodeInfo, NodeRef<Node::Target>), Error>
        where Node: NodeDecl {
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

        return Ok((builder.info, self.nodes.insert(decl.name, node)));
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
    pub fn node<Node>(&mut self, name: &str, decl: NodeHandle<Node>) -> Result<NodeRef<Node::Target>, Error>
        where Node: NodeDecl {
        let (info, node) = self.scene.node(decl)?;

        // TODO: Assert for duplicated keys
        self.info.nodes.insert(name.to_owned(), Arc::new(info));

        return Ok(node);
    }

    pub fn bound_attr<V, Attr>(&mut self, name: &str, decl: Attr, bounds: impl Into<Bounds<V>>) -> Result<Attr::Target, Error>
        where V: AttrValue + Bounded,
              Attr: BoundAttrDecl<V> {
        let bounds = bounds.into();

        let mut builder = AttrBuilder {
            node: self,
            info: AttrInfo {
                kind: Attr::Target::KIND,
                value_type: V::TYPE,
                attrs: HashMap::new(),
                inputs: HashMap::new(),
            },
        };

        let attr = decl.materialize(bounds, &mut builder)?;

        let info = Arc::new(builder.info);
        self.info.attrs.insert(name.to_owned(), info);

        return Ok(attr);
    }

    pub fn unbound_attr<V, Attr>(&mut self, name: &str, decl: Attr) -> Result<Attr::Target, Error>
        where V: AttrValue,
              Attr: UnboundAttrDecl<V> {
        let mut builder = AttrBuilder {
            node: self,
            info: AttrInfo {
                kind: Attr::Target::KIND,
                value_type: V::TYPE,
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
    pub fn bound_attr<V, Attr>(&mut self, name: &str, decl: Attr, bounds: impl Into<Bounds<V>>) -> Result<Attr::Target, Error>
        where V: AttrValue + Bounded,
              Attr: BoundAttrDecl<V> {
        return self.node.bound_attr(name, decl, bounds);
    }

    pub fn unbound_attr<V, Attr>(&mut self, name: &str, decl: Attr) -> Result<Attr::Target, Error>
        where V: AttrValue,
              Attr: UnboundAttrDecl<V> {
        return self.node.unbound_attr(name, decl);
    }

    pub fn input<V>(&mut self, name: &str, input: Input<V>) -> Result<Input<V>, Error>
        where V: InputValue {
        return Ok(input);
    }
}

pub struct NodeHandle<Node>
    where Node: NodeDecl {
    /// The scene-wide unique name of the node
    pub name: String,

    /// The declaration of the node
    pub decl: Node,
}

trait NodeBase {
    fn update(&mut self, duration: &Duration);
}

impl<Node> NodeBase for Node
    where Node: self::Node {
    fn update(&mut self, duration: &Duration) {
        Node::update(self, duration);
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

    pub fn node<'a, Node, E>(&mut self, name: impl Into<Cow<'a, str>>, decl: Node) -> Result<NodeHandle<Node>, Error>
        where Node: NodeDecl<Element=E> {
        return Ok(NodeHandle {
            name: name.into().into_owned(),
            decl,
        });
    }

    pub fn run<Node, Output, EN, EO>(self, root: NodeHandle<Node>, decl: Output) -> Result<(Loop<Node::Target, Output::Target>, Arc<Registry>), Error>
        where Node: NodeDecl<Element=EN>,
              Output: OutputDecl<Element=EO>,
              EN: Into<EO> {
        let mut builder = SceneBuilder {
            size: self.size,
            nodes: NodeArena::new(),
        };

        // Materialize the node tree using a builder tracking the info object creation
        let (root_info, root_node) = builder.node(root)?;

        // Materialize the output
        let output = decl.materialize(self.size())?;

        let registry = Registry::from(Arc::new(root_info));

        return Ok((Loop {
            nodes: builder.nodes,
            root: root_node,
            output,
        }, registry));
    }
}

pub struct Loop<Root, Output> {
    nodes: NodeArena,

    root: NodeRef<Root>,
    output: Output,
}

impl<Root, Output, EN, EO> Loop<Root, Output>
    where Root: self::Node<Element=EN>,
          Output: self::Output<Element=EO>,
          EN: Into<EO> {
    pub async fn run(mut self, fps: usize) -> Result<!, Error> {
        let mut timer = FrameTimer::new(fps);

        let mut stats = FrameStats::new();

        loop {
            let duration = timer.next().await;

            // Update the nodes
            for node in self.nodes.elements.iter_mut() {
                node.update(&duration);
            }

            // Render node tree to output
            let renderer = Renderer {
                nodes: &self.nodes,
            };
            let render = renderer.render(&self.root);
            self.output.render(&render);

            if let Some(stats) = stats.update(duration, fps) {
                eprintln!("Stats: min={:3.2}, max={:3.2}, avg={:3.2}", stats.min_fps(), stats.max_fps(), stats.avg_fps())
            }
        }
    }
}

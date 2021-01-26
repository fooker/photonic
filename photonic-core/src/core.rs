use std::borrow::Cow;
use std::marker::PhantomData;
use std::time::Duration;

use failure::Error;

use crate::attr::{AttrValue, BoundAttrDecl, Bounded, Bounds, UnboundAttrDecl};
use crate::input::{Input, InputValue};
use crate::interface::{NodeInfo, Registry};
use crate::utils::{FrameStats, FrameTimer};

struct Arena<E> {
    elements: Vec<E>,
}

impl<E> Arena<E> {
    fn new() -> Self {
        return Self {
            elements: Vec::new(),
        };
    }

    fn insert(&mut self, e: E) -> usize {
        self.elements.push(e);
        return self.elements.len() - 1;
    }
}

pub struct NodeHandle<Node> {
    // The name assigned to the node during declaration
    pub name: String,

    // Index of the element in the node arena
    index: usize,

    phantom: PhantomData<Node>,
}

impl<Node> NodeHandle<Node>
    where Node: self::Node {
    fn resolve<'l>(&self, nodes: &'l Arena<Box<dyn NodeBase>>) -> &'l Node {
        let element = nodes.elements[self.index].as_ref();
        return unsafe { &*(element as *const dyn NodeBase as *const Node) };
    }
}

pub struct Renderer<'a> {
    nodes: &'a Arena<Box<dyn NodeBase>>,
}

impl<'a> Renderer<'a> {
    pub fn render<'b, N, E>(&'b self, node: &'a NodeHandle<N>) -> <N as RenderType<'a>>::Render
        where 'b: 'a,
              N: Node<Element=E> + 'b {
        node.resolve(&self.nodes).render(self)
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

    fn materialize(self, size: usize, builder: &mut SceneBuilder) -> Result<Self::Target, Error>
        where Self::Target: std::marker::Sized;
}

pub trait RenderType<'a> {
    type Element;
    type Render: Render<Element=Self::Element> + 'a;
}

pub trait Node: for<'a> RenderType<'a> {
    const TYPE: &'static str;

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
    Root {
        node: String,
    },

    Nested {
        parent: &'p AttrPath<'p>,
        name: String,
    },
}

pub struct SceneBuilder<'r, 'l, 'p> {
    size: usize,

    registry: &'r mut Registry,

    nodes: &'l mut Arena<Box<dyn NodeBase>>,

    // TODO: Make name ownership nice
    path: AttrPath<'p>,
}

impl<'r, 'l, 'p> SceneBuilder<'r, 'l, 'p> {
    pub fn node<Node>(&mut self, name: &str, decl: NodeRef<Node>) -> Result<NodeHandle<Node::Target>, Error>
        where Node: NodeDecl {
        let node = Node::materialize(decl.decl, self.size, &mut SceneBuilder {
            size: self.size,
            registry: self.registry,
            nodes: self.nodes,
            path: AttrPath::Root { node: decl.name.clone() },
        })?;

        let handle = NodeHandle {
            name: decl.name,
            index: self.nodes.insert(Box::new(node)),
            phantom: Default::default(),
        };

        self.registry.register_node(&handle);

        return Ok(handle);
    }

    pub fn bound_attr<V, Decl>(&mut self, name: &str, decl: Decl, bounds: impl Into<Bounds<V>>) -> Result<Decl::Target, Error>
        where V: AttrValue + Bounded,
              Decl: BoundAttrDecl<V> {
        let bounds = bounds.into();

        let attr = Decl::materialize(decl, bounds, &mut SceneBuilder {
            size: self.size,
            registry: self.registry,
            nodes: self.nodes,
            path: AttrPath::Nested { parent: &self.path, name: name.to_owned() },
        })?;

        self.registry.register_attr(&attr, Some(bounds));

        return Ok(attr);
    }

    pub fn unbound_attr<V, Decl>(&mut self, name: &str, decl: Decl) -> Result<Decl::Attr, Error>
        where V: AttrValue,
              Decl: UnboundAttrDecl<V> {
        let attr = Decl::materialize(decl, &mut SceneBuilder {
            size: self.size,
            registry: self.registry,
            nodes: self.nodes,
            path: AttrPath::Nested { parent: &self.path, name: name.to_owned() },
        })?;

        return Ok(attr);
    }

    pub fn input<V>(&mut self, name: &str, input: Input<V>) -> Result<Input<V>, Error>
        where V: InputValue {
        return Ok(input);
    }
}

pub struct NodeRef<Node>
    where Node: NodeDecl {
    name: String,
    decl: Node,
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

    pub fn node<'a, Node, E>(&mut self, name: impl Into<Cow<'a, str>>, decl: Node) -> Result<NodeRef<Node>, Error>
        where Node: NodeDecl<Element=E> {
        return Ok(NodeRef {
            name: name.into().into_owned(),
            decl,
        });
    }

    pub fn output<Node, Output, EN, EO>(self, node: NodeRef<Node>, decl: Output) -> Result<Loop<Node::Target, Output::Target>, Error>
        where Node: NodeDecl<Element=EN>,
              Output: OutputDecl<Element=EO>,
              EN: Into<EO> {
        // The nodes created while materializing the scene
        let mut nodes: Arena<Box<dyn NodeBase>> = Arena::new();

        // Registry of info elements for external interface
        let mut registry = Registry::new();

        let mut builder = SceneBuilder {
            size: self.size,
            registry: &mut registry,
            nodes: &mut nodes,
            path: AttrPath::Root { node: node.name.to_string() },
        };

        let root = builder.node("root", node)?;

        let output = decl.materialize(self.size())?;

        return Ok(Loop {
            nodes,
            root,
            output,
        });
    }
}

pub struct Loop<Root, Output> {
    nodes: Arena<Box<dyn NodeBase>>,

    root: NodeHandle<Root>,
    output: Output,
}

impl<Root, Output, EN, EO> Loop<Root, Output>
    where Root: self::Node<Element=EN>,
          Output: self::Output<Element=EO>,
          EN: Into<EO> {
    pub fn run(mut self, fps: usize) -> Result<!, Error> {
        let mut timer = FrameTimer::new(fps);

        let mut stats = FrameStats::new();

        loop {
            let duration = timer.next();

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

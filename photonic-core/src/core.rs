use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::atomic::AtomicUsize;
use std::time::Duration;

use failure::Error;

use crate::input::Input;
use crate::utils::{FrameStats, FrameTimer};
use crate::value::{Bounds, BoundValueDecl, UnboundValueDecl, Value};
use std::borrow::Cow;

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
    index: usize,
    phantom: PhantomData<Node>,
}

impl<Node> NodeHandle<Node>
    where Node: NodeBase {
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
    type Target: Node<Element=Self::Element>;

    fn materialize(self, size: usize, builder: SceneBuilder) -> Result<Self::Target, Error>
        where Self::Target: std::marker::Sized;
}

pub trait RenderType<'a> {
    type Element;
    type Render: Render<Element=Self::Element> + 'a;
}

pub trait Node: for<'a> RenderType<'a> {
    fn update(&mut self, duration: &Duration);
    fn render<'a>(&'a self, renderer: &'a Renderer) -> <Self as RenderType<'a>>::Render;
}

pub trait OutputDecl {
    type Element;
    type Output: Output<Element=Self::Element>;

    fn materialize(self, size: usize) -> Result<Self::Output, Error>
        where Self::Output: std::marker::Sized;
}

pub trait Output {
    type Element;

    fn render<E: Into<Self::Element>>(&mut self, render: &dyn Render<Element=E>);
}

pub enum ValuePath<'p> {
    Root {
        node: String,
    },

    Nested {
        parent: &'p ValuePath<'p>,
        name: String,
    },
}

pub struct SceneBuilder<'s, 'l, 'p> {
    scene: &'s Scene,

    nodes: &'l mut Arena<Box<dyn NodeBase>>,

    path: ValuePath<'p>,
}

impl<'s, 'l, 'p> SceneBuilder<'s, 'l, 'p> {
    pub fn node<Node>(&mut self, name: &str, node: NodeRef<Node>) -> Result<NodeHandle<Node>, Error>
        where Node: self::Node {
        // TODO: Make name owning nice
        let node = (node.materialize)(self.scene.size, SceneBuilder {
            scene: self.scene,
            nodes: self.nodes,
            path: ValuePath::Root { node: node.name.to_string() },
        })?;

        let index = self.nodes.insert(node);

        return Ok(NodeHandle { index, phantom: Default::default() });
    }

    pub fn bound_value<Value, T>(&mut self, name: &str, value: Value, bounds: impl Into<Bounds<T>>) -> Result<Value::Value, Error>
        where Value: BoundValueDecl<T> {
        let mut builder = SceneBuilder {
            scene: self.scene,
            nodes: self.nodes,
            path: ValuePath::Nested { parent: &self.path, name: name.to_owned() },
        };

        let value = Value::meterialize(value, bounds.into(), &mut builder)?;

        return Ok(value);
    }

    pub fn unbound_value<Value, T>(&mut self, name: &str, value: Value) -> Result<Value::Value, Error>
        where Value: UnboundValueDecl<T> {
        let mut builder = SceneBuilder {
            scene: self.scene,
            nodes: self.nodes,
            path: ValuePath::Nested { parent: &self.path, name: name.to_owned() },
        };

        let value = Value::meterialize(value, &mut builder)?;

        return Ok(value);
    }

    pub fn input<T>(&mut self, name: &str, input: Input<T>) -> Result<Input<T>, Error> {
        return Ok(input);
    }
}

pub struct NodeRef<Node> {
    name: String,
    materialize: Box<dyn FnOnce(usize, SceneBuilder) -> Result<Box<dyn NodeBase>, Error>>,

    phantom: PhantomData<Node>,
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

    pub fn node<'a, Node: NodeDecl<Element=E> + 'static, E>(&mut self, name: impl Into<Cow<'a, str>>, decl: Node) -> Result<NodeRef<Node::Target>, Error>
        where Node::Target: 'static {

        return Ok(NodeRef {
            name: name.into().into_owned(),
            materialize: Box::new(|size, builder| {
                return Ok(Box::new(decl.materialize(size, builder)?));
            }),
            phantom: Default::default(),
        });
    }

    pub fn output<Node, Output, EN, EO>(self, node: NodeRef<Node>, decl: Output) -> Result<Loop<Node, Output::Output>, Error>
        where Node: self::Node<Element=EN>,
              Output: OutputDecl<Element=EO>,
              EN: Into<EO> {
        let mut nodes: Arena<Box<dyn NodeBase>> = Arena::new();

        let mut builder = SceneBuilder {
            scene: &self,
            nodes: &mut nodes,
            path: ValuePath::Root { node: node.name.to_string() }
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

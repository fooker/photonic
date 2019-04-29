use std::marker::PhantomData;
use std::time::Duration;

use failure::Error;

use crate::color::RGBColor;
use crate::utils::{FrameStats, FrameTimer};

pub type MainColor = RGBColor;

pub struct Scene {
    size: usize,

    nodes: Vec<Box<Node>>,
}

impl Scene {
    pub fn new(size: usize) -> Self {
        return Self {
            size,
            nodes: Vec::new(),
        };
    }

    pub fn size(&self) -> usize {
        return self.size;
    }

    pub fn node<D: NodeDecl>(&mut self, _name: &str, decl: D) -> Result<Handle<D::Target>, Error>
        where D::Target: 'static {
        let node = Box::new(decl.new(self.size())?);

        self.nodes.push(node);

        return Ok(Handle {
            index: self.nodes.len() - 1,
            _data: PhantomData,
        });
    }

    pub fn output<D: OutputDecl, N: Node>(self, node: Handle<N>, decl: D) -> Result<Loop<N, D::Output>, Error> {
        let output = decl.new(self.size())?;

        return Ok(Loop {
            scene: self,
            callbacks: Vec::new(),
            node,
            output,
        });
    }
}

pub struct Handle<N: Node> {
    index: usize,
    _data: PhantomData<N>,
}

//pub trait Dynamic {
//    fn update(&mut self, duration: &Duration);
//}

pub struct Renderer<'a> {
    scene: &'a Scene,
}

impl<'a> Renderer<'a> {
    pub fn render<'b: 'a, N: Node + 'b>(&'b self, handle: &'a Handle<N>) -> Box<Render + 'b> {
        self.scene.nodes[handle.index].render(self)
    }
}

pub trait Render {
    fn get(&self, index: usize) -> MainColor;
}

pub trait NodeDecl {
    type Target: Node;

    fn new(self, size: usize) -> Result<Self::Target, Error>
        where Self::Target: std::marker::Sized;
}

pub trait Node {
    fn update(&mut self, duration: &Duration);

    fn render<'a>(&'a self, renderer: &'a Renderer) -> Box<Render + 'a>;
}

//impl<T> Dynamic for T where T: Node {
//    fn update(&mut self, duration: &Duration) {
//        Node::update(self, duration)
//    }
//}

pub trait OutputDecl {
    type Output: Output;

    fn new(self, size: usize) -> Result<Self::Output, Error>
        where Self::Output: std::marker::Sized;
}

pub trait Output {
    fn render(&mut self, renderer: &Render);
}

pub struct Loop<Node: self::Node, Output: self::Output> {
    scene: Scene,

    callbacks: Vec<Box<FnMut(&Duration)>>,

    node: Handle<Node>,
    output: Output,
}

impl<Node: self::Node, Output: self::Output> Loop<Node, Output> {
    pub fn register<F>(&mut self, callback: F)
        where F: FnMut(&Duration) + 'static {
        self.callbacks.push(Box::new(callback));
    }

    pub fn run(mut self, fps: usize) -> Result<!, Error> {
        let mut timer = FrameTimer::new(fps);

        let mut stats = FrameStats::new();

        loop {
            let duration = timer.next();

            // Trigger callbacks
            for callback in self.callbacks.iter_mut() {
                callback(&duration);
            }

            // Update nodes
            for node in self.scene.nodes.iter_mut() {
                node.update(&duration);
            }

            // Render node tree to output
            let renderer = Renderer {
                scene: &self.scene,
            };
            let render = renderer.render(&self.node);
            self.output.render(render.as_ref());

            if let Some(stats) = stats.update(duration, fps) {
                eprintln!("Stats: min={:3.2}, max={:3.2}, avg={:3.2}", stats.min_fps(), stats.max_fps(), stats.avg_fps())
            }
        }
    }
}

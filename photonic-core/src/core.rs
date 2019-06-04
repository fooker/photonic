use std::marker::PhantomData;
use std::time::Duration;

use failure::Error;

use crate::utils::{FrameStats, FrameTimer};

pub struct Scene {
    size: usize,

    dynamics: Vec<Box<Dynamic>>,
}

impl Scene {
    pub fn new(size: usize) -> Self {
        return Self {
            size,
            dynamics: Vec::new(),
        };
    }

    pub fn size(&self) -> usize {
        return self.size;
    }

    pub fn register<D>(&mut self, _name: &str, dynamic: D) -> Handle<D>
        where D: Dynamic + 'static {
        let dynamic = Box::new(dynamic);

        self.dynamics.push(dynamic);

        return Handle {
            index: self.dynamics.len() - 1,
            phantom: PhantomData,
        };
    }

    pub fn node<Node: NodeDecl<Element=E>, E>(&mut self, name: &str, decl: Node) -> Result<Handle<Node::Target>, Error>
        where Node::Target: 'static {
        let node = decl.new(self.size())?;

        return Ok(self.register(name, node));
    }

    pub fn output<Node, Output, EN, EO>(self, node: Handle<Node>, decl: Output) -> Result<Loop<Node, Output::Output>, Error>
        where Node: self::Node<Element=EN>,
              Output: OutputDecl<Element=EO>,
              EN: Into<EO> {
        let output = decl.new(self.size())?;

        return Ok(Loop {
            scene: self,
            node,
            output,
        });
    }
}

pub struct Handle<T> {
    index: usize,
    phantom: PhantomData<T>,
}

impl<D> Handle<D>
    where D: Dynamic {
    fn resolve(&self, scene: &Scene) -> &D {
        let d = scene.dynamics[self.index].as_ref();
        return unsafe { &*(d as *const Dynamic as *const D) };
    }
}

pub struct Renderer<'a> {
    scene: &'a Scene,
}

impl<'a> Renderer<'a> {
    pub fn render<'b, N, E>(&'b self, handle: &'a Handle<N>) -> <N as RenderType<'a>>::Render
        where 'b: 'a,
              N: Node<Element=E> + 'b {
        handle.resolve(&self.scene).render(self)
    }
}

pub trait Render {
    type Element;

    fn get(&self, index: usize) -> Self::Element;
}

pub trait NodeDecl {
    type Element;
    type Target: Node<Element=Self::Element>;

    fn new(self, size: usize) -> Result<Self::Target, Error>
        where Self::Target: std::marker::Sized;
}

pub trait RenderType<'a> {
    type Element;
    type Render: Render<Element=Self::Element> + 'a;
}

pub trait Node: Dynamic + for<'a> RenderType<'a> {
    fn render<'a>(&'a self, renderer: &'a Renderer) -> <Self as RenderType<'a>>::Render;
}

pub trait Dynamic {
    fn update(&mut self, duration: &Duration);
}

pub trait OutputDecl {
    type Element;
    type Output: Output<Element=Self::Element>;

    fn new(self, size: usize) -> Result<Self::Output, Error>
        where Self::Output: std::marker::Sized;
}

pub trait Output {
    type Element;

    fn render<E: Into<Self::Element>>(&mut self, render: &Render<Element=E>);
}

pub struct Loop<Node, Output> {
    scene: Scene,

    node: Handle<Node>,
    output: Output,
}

impl<Node, Output, EN, EO> Loop<Node, Output>
    where Node: self::Node<Element=EN>,
          Output: self::Output<Element=EO>,
          EN: Into<EO> {
    pub fn run(mut self, fps: usize) -> Result<!, Error> {
        let mut timer = FrameTimer::new(fps);

        let mut stats = FrameStats::new();

        loop {
            let duration = timer.next();

            // Update all dynamics
            for dynamic in self.scene.dynamics.iter_mut() {
                dynamic.update(&duration);
            }

            // Render node tree to output
            let renderer = Renderer {
                scene: &self.scene,
            };
            let render = renderer.render(&self.node);
            self.output.render(&render);

            if let Some(stats) = stats.update(duration, fps) {
                eprintln!("Stats: min={:3.2}, max={:3.2}, avg={:3.2}", stats.min_fps(), stats.max_fps(), stats.avg_fps())
            }
        }
    }
}

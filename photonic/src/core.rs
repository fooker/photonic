use crate::color::RGBColor;
use crate::math::{self, Lerp};
use std::ops::{Deref, DerefMut};
use std::time::Duration;
use failure::Error;
use crate::utils::{FrameTimer, FrameStats};

pub type MainColor = RGBColor;

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

    pub fn node<Decl: NodeDecl>(&mut self, name: &str, decl: Decl) -> Result<Decl::Node, Error> {
        let node = decl.new(self.size)?;
        return Ok(node);
    }

    pub fn output<Decl: OutputDecl>(&mut self, decl: Decl) -> Result<Decl::Output, Error> {
        let output = decl.new(self.size)?;
        return Ok(output);
    }
}

pub trait Renderer {
    fn get(&self, index: usize) -> MainColor;
}

pub trait NodeDecl {
    type Node: Node;

    fn new(self, size: usize) -> Result<Self::Node, Error>
        where Self::Node: std::marker::Sized;
}

pub trait Node {
    const TYPE: &'static str;

    fn update(&mut self, duration: &Duration);

    fn render<'a>(&'a self) -> Box<Renderer + 'a>;
}

pub trait OutputDecl {
    type Output: Output;

    fn new(self, size: usize) -> Result<Self::Output, Error>
        where Self::Output: std::marker::Sized;
}

pub trait Output {
    fn render(&mut self, renderer: &Renderer);
}

pub struct Loop<Node: self::Node, Output: self::Output> {
    node: Node,
    output: Output,
}

impl<Node: self::Node, Output: self::Output> Loop<Node, Output> {
    pub fn new(node: Node, output: Output) -> Self {
        return Self {
            node,
            output,
        };
    }

    pub fn run(mut self, fps: usize) -> Result<!, Error> {
        let mut timer = FrameTimer::new(fps);

        let mut stats = FrameStats::new();

        loop {
            let duration = timer.next();

            // Update node tree
            self.node.update(&duration);

            // Render node tree to output
            self.output.render(self.node.render().as_ref());

            if let Some(stats) = stats.update(duration, fps) {
                eprintln!("Stats: min={:3.2}, max={:3.2}, avg={:3.2}", stats.min_fps(), stats.max_fps(), stats.avg_fps())
            }
        }
    }
}

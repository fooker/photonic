use core::*;
use std::time::Duration;
use utils::FractionalDuration;

struct RotationRenderer<'a> {
    source: Box<Renderer + 'a>,
    node: &'a RotationNode,
}

impl <'a> Renderer for RotationRenderer<'a> {
    fn size(&self) -> usize {
        self.source.size()
    }

    fn get(&self, index: usize) -> MainColor {
        self.source.get_interpolated(index as f64 - self.node.offset)
    }
}

pub struct RotationNode {
    source: Box<Node>,

    speed: Box<Value>,
    offset: f64,
}

impl RotationNode {
    pub fn new(source: Box<Node>,
               speed: Box<Value>) -> Self {
        Self {
            source,
            offset: 0.0,
            speed,
        }
    }
}

impl Node for RotationNode {
    fn render<'a>(&'a self) -> Box<Renderer + 'a> {
        Box::new(RotationRenderer {
            source: self.source.render(),
            node: self,
        })
    }
}

impl Dynamic for RotationNode {
    fn update(&mut self, duration: Duration) {
        self.source.update(duration);
        self.speed.update(duration);
        self.offset += self.speed.get() * duration.as_fractional_secs();
    }
}

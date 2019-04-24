use crate::core::*;
use crate::math;
use crate::math::Lerp;
use crate::values::*;
use std::time::Duration;
use failure::Error;

struct RotationRenderer<'a> {
    source: Box<Renderer + 'a>,
    size: usize,
    offset: f64,
}

impl<'a> Renderer for RotationRenderer<'a> {
    fn get(&self, index: usize) -> MainColor {
        let index = math::wrap((index as f64) - self.offset, self.size);
        let index = (index.trunc() as usize, index.fract());

        let c1 = self.source.get((index.0 + 0) % self.size);
        let c2 = self.source.get((index.0 + 1) % self.size);

        return MainColor::lerp(c1, c2, index.1);
    }
}

pub struct RotationNodeDecl<Source: Node> {
    source: Source,
    speed: Box<UnboundValueDecl<f64>>,
}

pub struct RotationNode<Source: Node> {
    size: usize,

    source: Source,

    speed: Box<Value<f64>>,

    offset: f64,
}

impl <Source: Node> NodeDecl for RotationNodeDecl<Source> {
    type Node = RotationNode<Source>;

    fn new(self, size: usize) -> Result<Self::Node, Error> {
        let speed = self.speed;
        let speed = speed.new();
        let speed = speed?;
        return Ok(Self::Node {
            size,
            source: self.source,
            speed: speed,
            offset: 0.0,
        });
    }
}

impl <Source: Node> Node for RotationNode<Source> {
    const TYPE: &'static str = "rotation";

    fn update(&mut self, duration: &Duration) {
        self.source.update(duration);
        self.speed.update(duration);
        self.offset += self.speed.get() * duration.as_secs_f64();
    }

    fn render<'a>(&'a self) -> Box<Renderer + 'a> {
        Box::new(RotationRenderer {
            source: self.source.render(),
            size: self.size,
            offset: self.offset,
        })
    }
}

//impl Inspection for RotationNode {
//    fn children(&self) -> Vec<NodeRef> {
//        vec![
//            NodeRef { name: "source", ptr: self.source.as_ref() },
//        ]
//    }
//
//    fn values(&self) -> Vec<ValueRef> {
//        vec![
////            ValueRef { name: "speed", ptr: self.speed },
//        ]
//    }
//}

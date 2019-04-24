use crate::animation::*;
use crate::core::*;
use crate::math::Lerp;
use crate::values::*;
use std::time::Duration;
use failure::Error;

struct SwitchRenderer<'a> {
    source: Box<Renderer + 'a>,
    target: Box<Renderer + 'a>,

    blend: f64,
}

impl<'a> Renderer for SwitchRenderer<'a> {
    fn get(&self, index: usize) -> MainColor {
        let source = self.source.get(index);
        let target = self.target.get(index);

        // TODO: Blending modes
        return MainColor::lerp(source,
                               target,
                               self.blend);
    }
}

pub struct SwitchNodeDecl<Source: Node> {
    // TODO: Make sources an iterator?

    pub sources: Vec<Source>,
    pub position: Box<BoundValueDecl<usize>>,
    pub easing: Option<Easing>,
}

pub struct SwitchNode<Source: Node> {
    sources: Vec<Source>,

    position: Box<Value<usize>>,

    source: usize,
    target: usize,
    blend: f64,

    easing: Option<Easing>,
    transition: Animation,
}

impl <Source: Node> NodeDecl for SwitchNodeDecl<Source> {
    type Node = SwitchNode<Source>;

    fn new(self, size: usize) -> Result<Self::Node, Error> {
        let position = self.position.new((0, self.sources.len()).into())?;

        return Ok(Self::Node {
            sources: self.sources,
            position,
            source: 0,
            target: 0,
            blend: 0.0,
            easing: self.easing,
            transition: Animation::Idle,
        });
    }
}

impl <Source: Node> Node for SwitchNode<Source> {
    const TYPE: &'static str = "switch";

    fn update(&mut self, duration: &Duration) {
        for source in self.sources.iter_mut() {
            source.update(duration);
        }

        if let Update::Changed(position) = self.position.update(duration) {
            let position = position as usize; // FIXME: Not sure
            if let Some(easing) = self.easing {
                self.source = self.target;
                self.target = position;
                self.blend = 0.0;
                self.transition = Animation::start(easing, 0.0, 1.0);
            } else {
                self.source = position;
                self.target = position;
            }
        }

        if let Some(value) = self.transition.update(duration) {
            self.blend = value;
        } else {
            self.source = self.target;
            self.blend = 0.0;
        }
    }

    fn render<'a>(&'a self) -> Box<Renderer + 'a> {
        if self.source == self.target {
            return self.sources[self.source].render();
        } else {
            return Box::new(SwitchRenderer {
                source: self.sources[self.source].render(),
                target: self.sources[self.target].render(),
                blend: self.blend,
            });
        }
    }
}

//impl Inspection for SwitchNode {
//    fn children(&self) -> Vec<NodeRef> {
//        return self.sources.iter().map(|source| {
//            NodeRef { name: "source", ptr: source.as_ref() }
//        }).collect();
//    }
//
//    fn values(&self) -> Vec<ValueRef> {
//        vec![
//            ValueRef { name: "position", ptr: ValuePtr::Int(&self.position) }
//        ]
//    }
//}

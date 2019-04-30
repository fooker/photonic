use std::time::Duration;

use failure::Error;

use crate::animation::*;
use crate::core::*;
use crate::math::Lerp;
use crate::values::*;

struct SwitchRenderer<'a> {
    source: Box<Render + 'a>,
    target: Box<Render + 'a>,

    blend: f64,
}

impl<'a> Render for SwitchRenderer<'a> {
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

    pub sources: Vec<Handle<Source>>,
    pub position: Box<BoundValueDecl<usize>>,
    pub easing: Option<Easing<f64>>,
}

pub struct SwitchNode<Source: Node> {
    sources: Vec<Handle<Source>>,

    position: Box<Value<usize>>,

    source: usize,
    target: usize,
    blend: f64,

    easing: Option<Easing<f64>>,
    transition: Animation<f64>,
}

impl<Source: Node> NodeDecl for SwitchNodeDecl<Source> {
    type Target = SwitchNode<Source>;

    fn new(self, _size: usize) -> Result<Self::Target, Error> {
        let position = self.position.new((0, self.sources.len() - 1).into())?;

        return Ok(Self::Target {
            sources: self.sources,
            position,
            source: 0,
            target: 0,
            blend: 0.0,
            easing: self.easing,
            transition: Animation::idle(),
        });
    }
}

impl<Source: Node> Node for SwitchNode<Source> {
    fn update(&mut self, duration: &Duration) {
        if let Update::Changed(position) = self.position.update(duration) {
            if let Some(easing) = self.easing {
                self.source = self.target;
                self.target = position;
                self.blend = 0.0;
                self.transition.start(easing, 0.0, 1.0);
            } else {
                self.source = position;
                self.target = position;
            }
        }

        if let Transition::Running(value) = self.transition.update(duration) {
            self.blend = value;
        } else {
            self.source = self.target;
            self.blend = 0.0;
        }
    }

    fn render<'a>(&'a self, renderer: &'a Renderer) -> Box<Render + 'a> {
        if self.source == self.target {
            return renderer.render(&self.sources[self.source]);
        } else {
            return Box::new(SwitchRenderer {
                source: renderer.render(&self.sources[self.source]),
                target: renderer.render(&self.sources[self.target]),
                blend: self.blend,
            });
        }
    }
}

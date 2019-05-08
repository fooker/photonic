use std::time::Duration;

use failure::Error;

use photonic_core::animation::*;
use photonic_core::core::*;
use photonic_core::math::Lerp;
use photonic_core::value::*;

struct SwitchRenderer<'a, E> {
    source: Box<Render<Element=E> + 'a>,
    target: Box<Render<Element=E> + 'a>,

    blend: f64,
}

impl<'a, E> Render for SwitchRenderer<'a, E>
    where E: Lerp {
    type Element = E;

    fn get(&self, index: usize) -> Self::Element {
        let source = self.source.get(index);
        let target = self.target.get(index);

        // TODO: Blending modes
        return E::lerp(source,
                       target,
                       self.blend);
    }
}

pub struct SwitchNodeDecl<Source> {
    // TODO: Make sources an iterator?

    pub sources: Vec<Handle<Source>>,
    pub position: Box<BoundValueDecl<usize>>,
    pub easing: Option<Easing<f64>>,
}

pub struct SwitchNode<Source> {
    sources: Vec<Handle<Source>>,

    position: Box<Value<usize>>,

    source: usize,
    target: usize,
    blend: f64,

    easing: Option<Easing<f64>>,
    transition: Animation<f64>,
}

impl<Source, E> NodeDecl for SwitchNodeDecl<Source>
    where Source: Node<Element=E>,
          E: Lerp + 'static {
    type Element = E;
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

impl<Source> Dynamic for SwitchNode<Source> {
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
}

impl<Source, E> Node for SwitchNode<Source>
    where Source: Node<Element=E>,
          E: Lerp + 'static {
    type Element = E;

    fn render<'a>(&'a self, renderer: &'a Renderer) -> Box<Render<Element=Self::Element> + 'a> {
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

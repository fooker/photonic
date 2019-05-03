use std::time::Duration;

use failure::Error;

use photonic_core::color::*;
use photonic_core::core::*;
use photonic_core::math;
use photonic_core::value::*;

struct Distortion<'a, F>
    where F: Fn(MainColor, f64) -> MainColor {
    source: Box<Render + 'a>,
    distortion: &'a F,
    value: f64,
    time: f64,
}

impl<'a, F> Render for Distortion<'a, F>
    where F: Fn(MainColor, f64) -> MainColor {
    fn get(&self, index: usize) -> MainColor {
        let c1 = self.source.get(index).convert();
        let c2 = (self.distortion)(c1, self.time);
        return math::Lerp::lerp(c1, c2, self.value);
    }
}

pub struct DistortionNodeDecl<Source: Node, F>
    where F: Fn(MainColor, f64) -> MainColor {
    pub source: Handle<Source>,
    pub value: Box<BoundValueDecl<f64>>,
    pub distortion: F,
}

pub struct DistortionNode<Source: Node, F>
    where F: Fn(MainColor, f64) -> MainColor {
    source: Handle<Source>,
    value: Box<Value<f64>>,
    distortion: F,

    time: f64,
}

impl<Source: Node, F> NodeDecl for DistortionNodeDecl<Source, F>
    where F: Fn(MainColor, f64) -> MainColor {
    type Target = DistortionNode<Source, F>;

    fn new(self, _size: usize) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            source: self.source,
            value: self.value.new(Bounds::norm())?,
            distortion: self.distortion,
            time: 0.0,
        });
    }
}

impl<Source: Node, F> Node for DistortionNode<Source, F>
    where F: Fn(MainColor, f64) -> MainColor {
    fn update(&mut self, duration: &Duration) {
        self.value.update(duration);

        self.time += duration.as_secs_f64();
    }

    fn render<'a>(&'a self, renderer: &'a Renderer) -> Box<Render + 'a> {
        return Box::new(Distortion {
            source: renderer.render(&self.source),
            distortion: &self.distortion,
            value: self.value.get(),
            time: self.time,
        });
    }
}

use std::time::Duration;

use failure::Error;
use noise::{NoiseFn, Perlin, Seedable};

use photonic_core::color::*;
use photonic_core::core::*;
use photonic_core::math;
use photonic_core::value::*;

struct Plasma<'a> {
    noise: &'a Perlin,

    h: ((f64, f64), f64),
    s: ((f64, f64), f64),
    v: ((f64, f64), f64),

    time: f64,
}

impl<'a> Render for Plasma<'a> {
    fn get(&self, index: usize) -> MainColor {
        let h = self.noise.get([index as f64 / self.h.1, self.time / self.h.1]);
        let s = self.noise.get([index as f64 / self.s.1, self.time / self.s.1]);
        let v = self.noise.get([index as f64 / self.v.1, self.time / self.v.1]);

        return HSVColor {
            h: math::remap(h, (-1.0, 1.0), self.h.0),
            s: math::remap(s, (-1.0, 1.0), self.s.0),
            v: math::remap(v, (-1.0, 1.0), self.v.0),
        }.convert();
    }
}

pub struct PlasmaNodeDecl {
    pub h: ((Box<BoundValueDecl<f64>>, Box<BoundValueDecl<f64>>), Box<UnboundValueDecl<f64>>),
    pub s: ((Box<BoundValueDecl<f64>>, Box<BoundValueDecl<f64>>), Box<UnboundValueDecl<f64>>),
    pub v: ((Box<BoundValueDecl<f64>>, Box<BoundValueDecl<f64>>), Box<UnboundValueDecl<f64>>),

    pub speed: Box<UnboundValueDecl<f64>>,
}

pub struct PlasmaNode {
    perlin: Perlin,

    h: ((Box<Value<f64>>, Box<Value<f64>>), Box<Value<f64>>),
    s: ((Box<Value<f64>>, Box<Value<f64>>), Box<Value<f64>>),
    v: ((Box<Value<f64>>, Box<Value<f64>>), Box<Value<f64>>),

    speed: Box<Value<f64>>,

    time: f64,
}

impl NodeDecl for PlasmaNodeDecl {
    type Target = PlasmaNode;

    fn new(self, _size: usize) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            perlin: Perlin::new()
                .set_seed(1),

            h: (((self.h.0).0.new((0.0, 360.0).into())?,
                 (self.h.0).1.new((0.0, 360.0).into())?),
                self.h.1.new()?),
            s: (((self.s.0).0.new(Bounds::norm())?,
                 (self.s.0).1.new(Bounds::norm())?),
                self.s.1.new()?),
            v: (((self.v.0).0.new(Bounds::norm())?,
                 (self.v.0).1.new(Bounds::norm())?),
                self.v.1.new()?),

            speed: self.speed.new()?,

            time: 0.0,
        });
    }
}

impl Node for PlasmaNode {
    fn update(&mut self, duration: &Duration) {
        (self.h.0).0.update(duration);
        (self.h.0).1.update(duration);
        (self.h.1).update(duration);
        (self.s.0).0.update(duration);
        (self.s.0).1.update(duration);
        (self.s.1).update(duration);
        (self.v.0).0.update(duration);
        (self.v.0).1.update(duration);
        (self.v.1).update(duration);

        self.speed.update(duration);

        self.time += duration.as_secs_f64() * self.speed.get();
    }

    fn render<'a>(&'a self, _renderer: &'a Renderer) -> Box<Render + 'a> {
        return Box::new(Plasma {
            noise: &self.perlin,
            h: (((self.h.0).0.get(), (self.h.0).1.get()), self.h.1.get()),
            s: (((self.s.0).0.get(), (self.s.0).1.get()), self.s.1.get()),
            v: (((self.v.0).0.get(), (self.v.0).1.get()), self.v.1.get()),
            time: self.time,
        });
    }
}

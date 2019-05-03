use std::time::Duration;

use failure::Error;
use noise::{NoiseFn, Perlin};

use photonic_core::color::*;
use photonic_core::core::*;
use photonic_core::math;
use photonic_core::value::*;

struct Plasma<'a> {
    noise: &'a Perlin,

    h: (f64, f64),
    s: (f64, f64),
    v: (f64, f64),

    t: f64,
}

impl<'a> Render for Plasma<'a> {
    fn get(&self, index: usize) -> MainColor {
        let point = [index as f64, self.t];

        let x = self.noise.get(point);

        return HSVColor {
            h: math::remap(x, (-1.0, 1.0), self.h),
            s: math::remap(x, (-1.0, 1.0), self.s),
            v: math::remap(x, (-1.0, 1.0), self.v),
        }.convert();
    }
}

pub struct PlasmaNodeDecl {
    pub hue: (Box<BoundValueDecl<f64>>, Box<BoundValueDecl<f64>>),
    pub saturation: (Box<BoundValueDecl<f64>>, Box<BoundValueDecl<f64>>),
    pub value: (Box<BoundValueDecl<f64>>, Box<BoundValueDecl<f64>>),

    pub speed: Box<UnboundValueDecl<f64>>,
}

pub struct PlasmaNode {
    perlin: Perlin,

    h: (Box<Value<f64>>, Box<Value<f64>>),
    s: (Box<Value<f64>>, Box<Value<f64>>),
    v: (Box<Value<f64>>, Box<Value<f64>>),

    speed: Box<Value<f64>>,

    time: f64,
}

impl NodeDecl for PlasmaNodeDecl {
    type Target = PlasmaNode;

    fn new(self, _size: usize) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            perlin: Perlin::new(),

            h: (self.hue.0.new((0.0, 360.0).into())?, self.hue.1.new((0.0, 360.0).into())?),
            s: (self.saturation.0.new(Bounds::norm())?, self.saturation.1.new(Bounds::norm())?),
            v: (self.value.0.new(Bounds::norm())?, self.value.1.new(Bounds::norm())?),

            speed: self.speed.new()?,

            time: 0.0,
        });
    }
}

impl Node for PlasmaNode {
    fn update(&mut self, duration: &Duration) {
        self.h.0.update(duration);
        self.h.1.update(duration);
        self.s.0.update(duration);
        self.s.1.update(duration);
        self.v.0.update(duration);
        self.v.1.update(duration);

        self.speed.update(duration);

        self.time += duration.as_secs_f64() * self.speed.get();
    }

    fn render<'a>(&'a self, _renderer: &'a Renderer) -> Box<Render + 'a> {
        return Box::new(Plasma {
            noise: &self.perlin,
            h: (self.h.0.get(), self.h.1.get()),
            s: (self.h.0.get(), self.h.1.get()),
            v: (self.h.0.get(), self.h.1.get()),
            t: self.time,
        });
    }
}

use std::time::Duration;

use failure::Error;
use rand::prelude::{FromEntropy, Rng, SmallRng};

use photonic_core::color::*;
use photonic_core::core::*;
use photonic_core::math;
use photonic_core::value::*;

#[derive(Clone)]
struct Raindrop {
    color: HSLColor,
    decay: f64,
}

impl Default for Raindrop {
    fn default() -> Self {
        Self {
            color: HSLColor::black(),
            decay: 0.0,
        }
    }
}

pub struct RaindropsRenderer<'a>(&'a Vec<Raindrop>);

impl<'a> Render for RaindropsRenderer<'a> {
    type Element = HSLColor;

    fn get(&self, index: usize) -> Self::Element {
        self.0[index].color
    }
}

struct Random(SmallRng);

impl Random {
    pub fn new() -> Self {
        Self(SmallRng::from_entropy())
    }

    pub fn rate(&mut self,
                value: f64,
                duration: &Duration) -> bool {
        return self.0.gen_bool(math::clamp(duration.as_secs_f64() * value, (0.0, 1.0)));
    }

    #[allow(clippy::float_cmp)]
    pub fn range(&mut self,
                 min: f64,
                 max: f64) -> f64 {
        let values = math::minmax(min, max);
        if values.0 == values.1 {
            return values.0;
        }

        return self.0.gen_range(values.0, values.1);
    }
}

pub struct RaindropsNodeDecl {
    pub rate: Box<BoundValueDecl<f64>>,

    pub hue: (Box<BoundValueDecl<f64>>, Box<BoundValueDecl<f64>>),
    pub saturation: (Box<BoundValueDecl<f64>>, Box<BoundValueDecl<f64>>),
    pub lightness: (Box<BoundValueDecl<f64>>, Box<BoundValueDecl<f64>>),

    pub decay: (Box<BoundValueDecl<f64>>, Box<BoundValueDecl<f64>>),
}

pub struct RaindropsNode {
    rate: Box<Value<f64>>,

    hue_min: Box<Value<f64>>,
    hue_max: Box<Value<f64>>,

    saturation_min: Box<Value<f64>>,
    saturation_max: Box<Value<f64>>,

    lightness_min: Box<Value<f64>>,
    lightness_max: Box<Value<f64>>,

    decay_min: Box<Value<f64>>,
    decay_max: Box<Value<f64>>,

    raindrops: Vec<Raindrop>,

    random: Random,
}

impl NodeDecl for RaindropsNodeDecl {
    type Element = HSLColor;
    type Target = RaindropsNode;

    fn new(self, size: usize) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            rate: self.rate.new(Bounds::norm())?,
            hue_min: self.hue.0.new((0.0, 360.0).into())?,
            hue_max: self.hue.1.new((0.0, 360.0).into())?,
            saturation_min: self.saturation.0.new(Bounds::norm())?,
            saturation_max: self.saturation.1.new(Bounds::norm())?,
            lightness_min: self.lightness.0.new(Bounds::norm())?,
            lightness_max: self.lightness.1.new(Bounds::norm())?,
            decay_min: self.decay.0.new(Bounds::norm())?,
            decay_max: self.decay.1.new(Bounds::norm())?,
            raindrops: vec![Raindrop::default(); size],
            random: Random::new(),
        });
    }
}

impl Dynamic for RaindropsNode {
    fn update(&mut self, duration: &Duration) {
        self.rate.update(duration);
        self.hue_min.update(duration);
        self.hue_max.update(duration);
        self.saturation_min.update(duration);
        self.saturation_max.update(duration);
        self.lightness_min.update(duration);
        self.lightness_max.update(duration);
        self.decay_min.update(duration);
        self.decay_max.update(duration);

        for raindrop in self.raindrops.iter_mut() {
            if self.random.rate(self.rate.get(), duration) {
                raindrop.color = HSLColor::new(self.random.range(self.hue_min.get(), self.hue_max.get()),
                                               self.random.range(self.saturation_min.get(), self.saturation_max.get()),
                                               self.random.range(self.lightness_min.get(), self.lightness_max.get()));
                raindrop.decay = self.random.range(self.decay_min.get(), self.decay_max.get());
            } else {
                raindrop.color.lightness = f64::max(0.0, raindrop.color.lightness * 1.0 - raindrop.decay * duration.as_secs_f64());
            }
        }
    }
}

impl <'a> RenderType<'a> for RaindropsNode {
    type Element = HSLColor;
    type Render = RaindropsRenderer<'a>;
}

impl Node for RaindropsNode {
    fn render<'a>(&'a self, _renderer: &Renderer) -> <Self as RenderType<'a>>::Render {
        return RaindropsRenderer(&self.raindrops);
    }
}

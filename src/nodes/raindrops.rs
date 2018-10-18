use color::*;
use core::*;
use attributes::*;
use math;
use rand::prelude::{FromEntropy, Rng, SmallRng};
use std::time::Duration;
use utils::FractionalDuration;

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

impl<'a> Renderer for &'a Vec<Raindrop> {
    fn size(&self) -> usize {
        self.len()
    }

    fn get(&self, index: usize) -> RGBColor {
        self[index].color.convert()
    }
}

struct Random(SmallRng);

impl Random {
    pub fn new() -> Self {
        Self(SmallRng::from_entropy())
    }

    pub fn rate(&mut self,
                value: &Box<Attribute>,
                duration: Duration) -> bool {
        return self.0.gen_bool(math::clamp(duration.as_fractional_secs() * value.get(), (0.0, 1.0)));
    }

    pub fn range(&mut self,
                 values: &(Box<Attribute>, Box<Attribute>)) -> f64 {
        let values = math::minmax(values.0.get(), values.1.get());
        if values.0 == values.1 {
            return values.0;
        }

        return self.0.gen_range(values.0, values.1);
    }
}

pub struct RaindropsNode {
    rate: Box<Attribute>,
    hue: (Box<Attribute>, Box<Attribute>),
    saturation: (Box<Attribute>, Box<Attribute>),
    lightness: (Box<Attribute>, Box<Attribute>),
    decay: (Box<Attribute>, Box<Attribute>),

    raindrops: Vec<Raindrop>,

    random: Random,
}

impl RaindropsNode {
    pub fn new(size: usize,
               rate: Box<Attribute>,
               hue: (Box<Attribute>, Box<Attribute>),
               saturation: (Box<Attribute>, Box<Attribute>),
               lightness: (Box<Attribute>, Box<Attribute>),
               decay: (Box<Attribute>, Box<Attribute>),
    ) -> Self {
        Self {
            rate,
            hue,
            saturation,
            lightness,
            decay,
            raindrops: vec![Raindrop::default(); size],
            random: Random::new(),
        }
    }
}

impl Node for RaindropsNode {
    fn render<'a>(&'a self) -> Box<Renderer + 'a> {
        Box::new(&self.raindrops)
    }
}

impl Dynamic for RaindropsNode {
    fn update(&mut self, duration: Duration) {
        self.rate.update(duration);
        self.hue.0.update(duration);
        self.hue.1.update(duration);
        self.saturation.0.update(duration);
        self.saturation.1.update(duration);
        self.lightness.0.update(duration);
        self.lightness.1.update(duration);
        self.decay.0.update(duration);
        self.decay.1.update(duration);

        for raindrop in self.raindrops.iter_mut() {
            if self.random.rate(&self.rate, duration) {
                raindrop.color.h = self.random.range(&self.hue);
                raindrop.color.s = self.random.range(&self.saturation);
                raindrop.color.l = self.random.range(&self.lightness);
                raindrop.decay = self.random.range(&self.decay);
            } else {
                raindrop.color.l = f64::max(0.0, raindrop.color.l * 1.0 - raindrop.decay * duration.as_fractional_secs());
            }
        }
    }
}

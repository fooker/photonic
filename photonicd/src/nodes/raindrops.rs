use photonic::values::*;
use photonic::color::*;
use photonic::core::*;
use photonic::math;
use rand::prelude::{FromEntropy, Rng, SmallRng};
use std::time::Duration;

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

struct Raindrops<'a>(&'a Vec<Raindrop>);

impl<'a> Renderer for Raindrops<'a> {
    fn get(&self, index: usize) -> RGBColor {
        self.0[index].color.convert()
    }
}

struct Random(SmallRng);

impl Random {
    pub fn new() -> Self {
        Self(SmallRng::from_entropy())
    }

    pub fn rate(&mut self,
                value: &FloatValue,
                duration: &Duration) -> bool {
        return self.0.gen_bool(math::clamp(duration.as_float_secs() * value.get(), (0.0, 1.0)));
    }

    #[allow(clippy::float_cmp)]
    pub fn range(&mut self,
                 min: &FloatValue,
                 max: &FloatValue) -> f64 {
        let values = math::minmax(min.get(), max.get());
        if values.0 == values.1 {
            return values.0;
        }

        return self.0.gen_range(values.0, values.1);
    }
}

#[derive(Inspection)]
pub struct RaindropsNode {
    #[value()] rate: FloatValue,

    #[value()] hue_min: FloatValue,
    #[value()] hue_max: FloatValue,

    #[value()] saturation_min: FloatValue,
    #[value()] saturation_max: FloatValue,

    #[value()] lightness_min: FloatValue,
    #[value()] lightness_max: FloatValue,

    #[value()] decay_min: FloatValue,
    #[value()] decay_max: FloatValue,

    raindrops: Vec<Raindrop>,

    random: Random,
}

impl RaindropsNode {
    const CLASS: &'static str = "raindrops";

    pub fn new(size: usize,
               rate: FloatValueFactory,
               hue_min: FloatValueFactory,
               hue_max: FloatValueFactory,
               saturation_min: FloatValueFactory,
               saturation_max: FloatValueFactory,
               lightness_min: FloatValueFactory,
               lightness_max: FloatValueFactory,
               decay_min: FloatValueFactory,
               decay_max: FloatValueFactory,
    ) -> Result<Self, String> {
        Ok(Self {
            rate: rate(FloatValueDecl{name: "rate", min: Some(0.0), max: None})?,
            hue_min: hue_min(FloatValueDecl{name: "hue_min", min: Some(0.0), max: Some(360.0)})?,
            hue_max: hue_max(FloatValueDecl{name: "hue_max", min: Some(0.0), max: Some(360.0)})?,
            saturation_min: saturation_min(FloatValueDecl{name: "saturation_min", min: Some(0.0), max: Some(1.0)})?,
            saturation_max: saturation_max(FloatValueDecl{name: "saturation_max", min: Some(0.0), max: Some(1.0)})?,
            lightness_min: lightness_min(FloatValueDecl{name: "lightness_min", min: Some(0.0), max: Some(1.0)})?,
            lightness_max: lightness_max(FloatValueDecl{name: "lightness_max", min: Some(0.0), max: Some(1.0)})?,
            decay_min: decay_min(FloatValueDecl{name: "decay_min", min: Some(0.0), max: None})?,
            decay_max: decay_max(FloatValueDecl{name: "decay_max", min: Some(0.0), max: None})?,
            raindrops: vec![Raindrop::default(); size],
            random: Random::new(),
        })
    }
}

impl Node for RaindropsNode {
    fn class(&self) -> &'static str {
        Self::CLASS
    }
}

impl Source for RaindropsNode {
    fn render<'a>(&'a self) -> Box<Renderer + 'a> {
        Box::new(Raindrops(&self.raindrops))
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
            if self.random.rate(&self.rate, duration) {
                raindrop.color.h = self.random.range(&self.hue_min, &self.hue_max);
                raindrop.color.s = self.random.range(&self.saturation_min, &self.saturation_max);
                raindrop.color.l = self.random.range(&self.lightness_min, &self.lightness_max);
                raindrop.decay = self.random.range(&self.decay_min, &self.decay_max);
            } else {
                raindrop.color.l = f64::max(0.0, raindrop.color.l * 1.0 - raindrop.decay * duration.as_float_secs());
            }
        }
    }
}

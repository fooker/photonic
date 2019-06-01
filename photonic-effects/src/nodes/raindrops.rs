use std::time::Duration;

use failure::Error;
use rand::prelude::{FromEntropy, Rng, SmallRng};

use photonic_core::color::*;
use photonic_core::core::*;
use photonic_core::math;
use photonic_core::math::Lerp;
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

    pub fn color(&mut self,
                 c1: HSLColor,
                 c2: HSLColor) -> HSLColor {
        let v = self.0.gen();
        return Lerp::lerp(c1, c2, v);
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

pub struct RaindropsNodeDecl<Rate, Color, Decay> {
    pub rate: Rate,
    pub color: Color,
    pub decay: Decay,
}

pub struct RaindropsNode<Rate, Color, Decay> {
    rate: Rate,
    color: Color,
    decay: Decay,

    raindrops: Vec<Raindrop>,

    random: Random,
}

impl<Rate, Color, Decay> NodeDecl for RaindropsNodeDecl<Rate, Color, Decay>
    where Rate: BoundValueDecl<f64>,
          Color: UnboundValueDecl<Range<HSLColor>>,
          Decay: BoundValueDecl<Range<f64>> {
    type Element = HSLColor;
    type Target = RaindropsNode<Rate::Value, Color::Value, Decay::Value>;

    fn new(self, size: usize) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            rate: self.rate.new(Bounds::norm())?,
            color: self.color.new()?,
            decay: self.decay.new(Bounds { min: (0.0, 0.0).into(), max: (1.0, 1.0).into() })?,
            raindrops: vec![Raindrop::default(); size],
            random: Random::new(),
        });
    }
}

impl<Rate, Color, Decay> Dynamic for RaindropsNode<Rate, Color, Decay>
    where Rate: Value<f64>,
          Color: Value<Range<HSLColor>>,
          Decay: Value<Range<f64>> {
    fn update(&mut self, duration: &Duration) {
        self.rate.update(duration);
        self.color.update(duration);
        self.decay.update(duration);

        for raindrop in self.raindrops.iter_mut() {
            if self.random.rate(self.rate.get(), duration) {
                raindrop.color = self.random.color(self.color.get().0, self.color.get().1);
                raindrop.decay = self.random.range(self.decay.get().0, self.decay.get().1);
            } else {
                raindrop.color.lightness = f64::max(0.0, raindrop.color.lightness * 1.0 - raindrop.decay * duration.as_secs_f64());
            }
        }
    }
}

impl<'a, Rate, Color, Decay> RenderType<'a> for RaindropsNode<Rate, Color, Decay> {
    type Element = HSLColor;
    type Render = RaindropsRenderer<'a>;
}

impl<Rate, Color, Decay> Node for RaindropsNode<Rate, Color, Decay>
    where Rate: Value<f64>,
          Color: Value<Range<HSLColor>>,
          Decay: Value<Range<f64>> {
    fn render<'a>(&'a self, _renderer: &Renderer) -> <Self as RenderType<'a>>::Render {
        return RaindropsRenderer(&self.raindrops);
    }
}

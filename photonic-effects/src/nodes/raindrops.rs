use std::time::Duration;

use anyhow::Error;
use rand::prelude::{FromEntropy, Rng, SmallRng};

use photonic_core::color::{HSLColor, Black};
use photonic_core::scene::NodeBuilder;
use photonic_core::math;
use photonic_core::math::Lerp;
use photonic_core::attr::{BoundAttrDecl, UnboundAttrDecl, Attr, Range, Bounds};
use photonic_core::node::{RenderType, Node, NodeDecl, Render};

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
                duration: Duration) -> bool {
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
    where Rate: BoundAttrDecl<f64>,
          Color: UnboundAttrDecl<Range<HSLColor>>,
          Decay: BoundAttrDecl<Range<f64>> {
    type Element = HSLColor;
    type Target = RaindropsNode<Rate::Target, Color::Target, Decay::Target>;

    fn materialize(self, size: usize, builder: &mut NodeBuilder) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            rate: builder.bound_attr("rate", self.rate, Bounds::normal())?,
            color: builder.unbound_attr("color", self.color)?,
            decay: builder.bound_attr("decay", self.decay, Bounds { min: Range(0.0, 0.0), max: Range(1.0, 1.0) })?,
            raindrops: vec![Raindrop::default(); size],
            random: Random::new(),
        });
    }
}

impl<'a, Rate, Color, Decay> RenderType<'a, Self> for RaindropsNode<Rate, Color, Decay>
    where Rate: Attr<f64>,
          Color: Attr<Range<HSLColor>>,
          Decay: Attr<Range<f64>> {
    type Render = RaindropsRenderer<'a>;
}

impl<Rate, Color, Decay> Node for RaindropsNode<Rate, Color, Decay>
    where Rate: Attr<f64>,
          Color: Attr<Range<HSLColor>>,
          Decay: Attr<Range<f64>> {
    const KIND: &'static str = "raindrops";

    type Element = HSLColor;

    fn update(&mut self, duration: Duration) {
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

    fn render(&mut self) -> <Self as RenderType<Self>>::Render {
        return RaindropsRenderer(&self.raindrops);
    }
}

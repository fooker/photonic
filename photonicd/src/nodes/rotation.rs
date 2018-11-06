use photonic::core::*;
use photonic::math;
use photonic::math::Lerp;
use photonic::values::*;
use std::time::Duration;

struct RotationRenderer<'a> {
    source: Box<Renderer + 'a>,
    size: usize,
    offset: f64,
}

impl<'a> Renderer for RotationRenderer<'a> {
    fn get(&self, index: usize) -> MainColor {
        let index = math::wrap((index as f64) - self.offset, self.size);
        let index = (index.trunc() as usize, index.fract());

        let c1 = self.source.get((index.0 + 0) % self.size);
        let c2 = self.source.get((index.0 + 1) % self.size);

        return MainColor::lerp(c1, c2, index.1);
    }
}

#[derive(Inspection)]
pub struct RotationNode {
    size: usize,

    #[node()]
    source: Box<Node>,

    #[value()]
    speed: FloatValue,

    offset: f64,
}

impl RotationNode {
    const CLASS: &'static str = "rotation";

    pub fn new(size: usize,
               source: Box<Node>,
               speed: FloatValueFactory) -> Result<Self, String> {
        Ok(Self {
            size,
            source,
            speed: speed(FloatValueDecl { name: "speed", min: None, max: None })?,
            offset: 0.0,
        })
    }
}

impl Node for RotationNode {
    fn class(&self) -> &'static str {
        Self::CLASS
    }
}

impl Source for RotationNode {
    fn render<'a>(&'a self) -> Box<Renderer + 'a> {
        Box::new(RotationRenderer {
            source: self.source.render(),
            size: self.size,
            offset: self.offset,
        })
    }
}

impl Dynamic for RotationNode {
    fn update(&mut self, duration: &Duration) {
        self.source.update(duration);
        self.speed.update(duration);
        self.offset += self.speed.get() * duration.as_float_secs();
    }
}

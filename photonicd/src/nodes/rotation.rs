use photonic::attributes::*;
use photonic::core::*;
use std::time::Duration;

struct RotationRenderer<'a> {
    source: Box<Renderer + 'a>,
    offset: f64,
}

impl<'a> Renderer for RotationRenderer<'a> {
    fn size(&self) -> usize {
        self.source.size()
    }

    fn get(&self, index: usize) -> MainColor {
        self.source.get_interpolated(index as f64 - self.offset)
    }
}

#[derive(Inspection)]
pub struct RotationNode {
    #[node()]
    source: Box<Node>,

    #[attr()]
    speed: Attribute,
    offset: f64,
}

impl RotationNode {
    const CLASS: &'static str = "rotation";

    pub fn new(source: Box<Node>,
               speed: Attribute) -> Self {
        Self {
            source,
            offset: 0.0,
            speed,
        }
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

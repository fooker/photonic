use photonic::attributes::*;
use photonic::core::*;
use photonic::utils::FractionalDuration;
use std::time::Duration;

struct RotationRenderer<'a> {
    source: Box<Renderer + 'a>,
    node: &'a RotationNode,
}

impl<'a> Renderer for RotationRenderer<'a> {
    fn size(&self) -> usize {
        self.source.size()
    }

    fn get(&self, index: usize) -> MainColor {
        self.source.get_interpolated(index as f64 - self.node.offset)
    }
}

#[derive(Node)]
pub struct RotationNode {
    #[node()]
    source: Box<Node>,

    #[attr()]
    speed: Box<Attribute>,
    offset: f64,
}

impl RotationNode {
    pub fn new(source: Box<Node>,
               speed: Box<Attribute>) -> Self {
        Self {
            source,
            offset: 0.0,
            speed,
        }
    }
}

impl Source for RotationNode {
    fn render<'a>(&'a self) -> Box<Renderer + 'a> {
        Box::new(RotationRenderer {
            source: self.source.render(),
            node: self,
        })
    }
}

impl Dynamic for RotationNode {
    fn update(&mut self, duration: Duration) {
        self.source.update(duration);
        self.speed.update(duration);
        self.offset += self.speed.get() * duration.as_fractional_secs();
    }
}

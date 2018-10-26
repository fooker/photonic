use photonic::attributes::*;
use photonic::color::Black;
use photonic::core::*;
use photonic::math::Lerp;
use std::time::Duration;

struct OverlayRenderer<'a> {
    base: Box<Renderer + 'a>,
    overlay: Box<Renderer + 'a>,

    blend: f64,
}

impl<'a> Renderer for OverlayRenderer<'a> {
    fn get(&self, index: usize) -> MainColor {
        let base = self.base.get(index);
        let overlay = self.overlay.get(index);

        // TODO: Blending modes
        return MainColor::lerp(base,
                               overlay,
                               self.blend);
    }
}

#[derive(Inspection)]
pub struct OverlayNode {
    #[node()] base: Box<Node>,
    #[node()] overlay: Box<Node>,

    #[attr()] blend: Attribute,
}

impl OverlayNode {
    const CLASS: &'static str = "overlay";

    pub fn new(base: Box<Node>,
               overlay: Box<Node>,
               blend: Attribute) -> Self {
        Self {
            base,
            overlay,
            blend,
        }
    }
}

impl Node for OverlayNode {
    fn class(&self) -> &'static str {
        Self::CLASS
    }
}

impl Source for OverlayNode {
    fn render<'a>(&'a self) -> Box<Renderer + 'a> {
        let blend = self.blend.get();
        if blend > 0f64 {
            return Box::new(OverlayRenderer {
                base: self.base.render(),
                overlay: self.overlay.render(),
                blend,
            });
        } else {
            return self.base.render();
        }
    }
}

impl Dynamic for OverlayNode {
    fn update(&mut self, duration: &Duration) {
        self.base.update(duration);
        self.overlay.update(duration);
        self.blend.update(duration);
    }
}


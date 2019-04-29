use std::time::Duration;

use crate::core::*;
use crate::math::Lerp;
use crate::values::*;
use failure::Error;

struct OverlayRenderer<'a> {
    base: Box<Render + 'a>,
    overlay: Box<Render + 'a>,

    blend: f64,
}

impl<'a> Render for OverlayRenderer<'a> {
    fn get(&self, index: usize) -> MainColor {
        let base = self.base.get(index);
        let overlay = self.overlay.get(index);

        // TODO: Blending modes
        return MainColor::lerp(base,
                               overlay,
                               self.blend);
    }
}

pub struct OverlayNodeDecl<Base: Node, Overlay: Node> {
    pub base: Handle<Base>,
    pub overlay: Handle<Overlay>,

    pub blend: Box<BoundValueDecl<f64>>,
}

pub struct OverlayNode<Base: Node, Overlay: Node> {
    base: Handle<Base>,
    overlay: Handle<Overlay>,

    blend: Box<Value<f64>>,
}

impl<Base: Node, Overlay: Node> NodeDecl for OverlayNodeDecl<Base, Overlay> {
    type Target = OverlayNode<Base, Overlay>;

    fn new(self, _size: usize) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            base: self.base,
            overlay: self.overlay,
            blend: self.blend.new(Bounds::norm())?,
        });
    }
}

impl<Base: Node, Overlay: Node> Node for OverlayNode<Base, Overlay> {
    fn update(&mut self, duration: &Duration) {
        self.blend.update(duration);
    }

    fn render<'a>(&'a self, renderer: &'a Renderer) -> Box<Render + 'a> {
        let blend = self.blend.get();
        if blend > 0f64 {
            return Box::new(OverlayRenderer {
                base: renderer.render(&self.base),
                overlay: renderer.render(&self.overlay),
                blend,
            });
        } else {
            return renderer.render(&self.base);
        }
    }
}

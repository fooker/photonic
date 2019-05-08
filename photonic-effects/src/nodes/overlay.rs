use std::time::Duration;

use failure::Error;

use photonic_core::core::*;
use photonic_core::math::Lerp;
use photonic_core::value::*;

struct OverlayRenderer<'a, EB, EO> {
    base: Box<Render<Element=EB> + 'a>,
    overlay: Box<Render<Element=EO> + 'a>,

    blend: f64,
}

impl<'a, EB, EO> Render for OverlayRenderer<'a, EB, EO>
    where EB: Lerp + 'static,
          EO: Into<EB> {
    type Element = EB;

    fn get(&self, index: usize) -> Self::Element {
        let base = self.base.get(index);
        let overlay = self.overlay.get(index).into();

        // TODO: Blending modes
        return EB::lerp(base,
                        overlay,
                        self.blend);
    }
}

pub struct OverlayNodeDecl<Base, Overlay> {
    pub base: Handle<Base>,
    pub overlay: Handle<Overlay>,

    pub blend: Box<BoundValueDecl<f64>>,
}

pub struct OverlayNode<Base, Overlay> {
    base: Handle<Base>,
    overlay: Handle<Overlay>,

    blend: Box<Value<f64>>,
}

impl<Base, Overlay, EB, EO> NodeDecl for OverlayNodeDecl<Base, Overlay>
    where Base: Node<Element=EB>,
          Overlay: Node<Element=EO>,
          EB: Lerp + 'static,
          EO: Into<EB> + 'static {
    type Element = EB;
    type Target = OverlayNode<Base, Overlay>;

    fn new(self, _size: usize) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            base: self.base,
            overlay: self.overlay,
            blend: self.blend.new(Bounds::norm())?,
        });
    }
}

impl<Base, Overlay> Dynamic for OverlayNode<Base, Overlay> {
    fn update(&mut self, duration: &Duration) {
        self.blend.update(duration);
    }
}

impl<Base, Overlay, EB, EO> Node for OverlayNode<Base, Overlay>
    where Base: Node<Element=EB>,
          Overlay: Node<Element=EO>,
          EB: Lerp + 'static,
          EO: Into<EB> + 'static {
    type Element = EB;

    fn render<'a>(&'a self, renderer: &'a Renderer) -> Box<Render<Element=Self::Element> + 'a> {
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

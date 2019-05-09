use std::time::Duration;

use failure::Error;

use photonic_core::core::*;
use photonic_core::math::Lerp;
use photonic_core::value::*;

pub struct OverlayRenderer<Base, Overlay> {
    base: Base,
    overlay: Overlay,

    blend: f64,
}

impl<Base, Overlay> Render for OverlayRenderer<Base, Overlay>
    where Base: Render,
          Overlay: Render,
          Base::Element: Lerp,
          Overlay::Element: Into<Base::Element> {
    type Element = Base::Element;

    fn get(&self, index: usize) -> Self::Element {
        let base = self.base.get(index);
        let overlay = self.overlay.get(index).into();

        // TODO: Blending modes
        return Self::Element::lerp(base,
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
          EB: Lerp,
          EO: Into<EB> {
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

impl<'a, Base, Overlay> RenderType<'a> for OverlayNode<Base, Overlay>
    where Base: RenderType<'a>,
          Overlay: RenderType<'a>,
          Base::Element: Lerp,
          Overlay::Element: Into<Base::Element> {
    type Element = Base::Element;
    type Render = OverlayRenderer<Base::Render, Overlay::Render>;
}

impl<Base, Overlay, EB, EO> Node for OverlayNode<Base, Overlay>
    where Base: Node<Element=EB>,
          Overlay: Node<Element=EO>,
          EB: Lerp,
          EO: Into<EB> {
    fn render<'a>(&'a self, renderer: &'a Renderer) -> <Self as RenderType<'a>>::Render {
        return OverlayRenderer {
            base: renderer.render(&self.base),
            overlay: renderer.render(&self.overlay),
            blend: self.blend.get(),
        };
    }
}

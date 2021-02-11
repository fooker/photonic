use std::time::Duration;

use failure::Error;

use photonic_core::scene::{NodeBuilder, NodeHandle};
use photonic_core::math::Lerp;
use photonic_core::attr::{BoundAttrDecl, Attr, Bounds};
use photonic_core::node::{RenderType, Node, NodeDecl, Render};

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

pub struct OverlayNodeDecl<Base, Overlay, Blend>
    where Base: NodeDecl,
          Overlay: NodeDecl {
    pub base: NodeHandle<Base>,
    pub overlay: NodeHandle<Overlay>,

    pub blend: Blend,
}

pub struct OverlayNode<Base, Overlay, Blend> {
    base: Base,
    overlay: Overlay,

    blend: Blend,
}

impl<Base, Overlay, Blend, EB, EO> NodeDecl for OverlayNodeDecl<Base, Overlay, Blend>
    where Base: NodeDecl<Element=EB>,
          Overlay: NodeDecl<Element=EO>,
          Blend: BoundAttrDecl<f64>,
          EB: Lerp,
          EO: Into<EB> {
    type Element = EB;
    type Target = OverlayNode<Base::Target, Overlay::Target, Blend::Target>;

    fn materialize(self, _size: usize, builder: &mut NodeBuilder) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            base: builder.node("base", self.base)?,
            overlay: builder.node("overlay", self.overlay)?,
            blend: builder.bound_attr("blend", self.blend, Bounds::normal())?,
        });
    }
}

impl<'a, Base, Overlay, Blend> RenderType<'a> for OverlayNode<Base, Overlay, Blend>
    where Base: RenderType<'a>,
          Overlay: RenderType<'a>,
          Base::Element: Lerp,
          Overlay::Element: Into<Base::Element> {
    type Element = Base::Element;
    type Render = OverlayRenderer<Base::Render, Overlay::Render>;
}

impl<Base, Overlay, Blend, EB, EO> Node for OverlayNode<Base, Overlay, Blend>
    where Base: Node<Element=EB>,
          Overlay: Node<Element=EO>,
          Blend: Attr<f64>,
          EB: Lerp,
          EO: Into<EB> {
    const KIND: &'static str = "overlay";

    fn update(&mut self, duration: &Duration) {
        self.blend.update(duration);
    }

    fn render(&mut self) -> <Self as RenderType>::Render {
        return OverlayRenderer {
            base: self.base.render(),
            overlay: self.overlay.render(),
            blend: self.blend.get(),
        };
    }
}

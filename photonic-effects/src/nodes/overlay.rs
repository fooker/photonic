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

pub struct OverlayNodeDecl<Base, Overlay, Blend> {
    pub base: NodeRef<Base>,
    pub overlay: NodeRef<Overlay>,

    pub blend: Blend,
}

pub struct OverlayNode<Base, Overlay, Blend> {
    base: NodeHandle<Base>,
    overlay: NodeHandle<Overlay>,

    blend: Blend,
}

impl<Base, Overlay, Blend, EB, EO> NodeDecl for OverlayNodeDecl<Base, Overlay, Blend>
    where Base: Node<Element=EB>,
          Overlay: Node<Element=EO>,
          Blend: BoundValueDecl<f64>,
          EB: Lerp,
          EO: Into<EB> {
    type Element = EB;
    type Target = OverlayNode<Base, Overlay, Blend::Value>;

    fn materialize(self, _size: usize, mut builder: SceneBuilder) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            base: builder.node("base", self.base)?,
            overlay: builder.node("overlay", self.overlay)?,
            blend: builder.bound_value("blend", self.blend, Bounds::normal())?,
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
          Blend: Value<f64>,
          EB: Lerp,
          EO: Into<EB> {
    fn update(&mut self, duration: &Duration) {
        self.blend.update(duration);
    }

    fn render<'a>(&'a self, renderer: &'a Renderer) -> <Self as RenderType<'a>>::Render {
        return OverlayRenderer {
            base: renderer.render(&self.base),
            overlay: renderer.render(&self.overlay),
            blend: self.blend.get(),
        };
    }
}

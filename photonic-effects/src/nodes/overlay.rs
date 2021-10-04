use std::time::Duration;

use anyhow::Result;

use photonic_core::attr::{Attr, BoundAttrDecl, Bounds};
use photonic_core::math::Lerp;
use photonic_core::node::{Node, NodeDecl, Render, RenderType};
use photonic_core::scene::{NodeBuilder, NodeHandle};

pub struct OverlayRenderer<Base, Overlay> {
    base: Base,
    overlay: Overlay,

    blend: f64,
}

impl<Base, Overlay> Render for OverlayRenderer<Base, Overlay>
where
    Base: Render,
    Overlay: Render,
    Base::Element: Lerp,
    Overlay::Element: Into<Base::Element>,
{
    type Element = Base::Element;

    fn get(&self, index: usize) -> Result<Self::Element> {
        let base = self.base.get(index)?;
        let overlay = self.overlay.get(index)?.into();

        // TODO: Blending modes
        return Ok(Self::Element::lerp(base, overlay, self.blend));
    }
}

pub struct OverlayNodeDecl<Base, Overlay, Blend>
where
    Base: NodeDecl,
    Overlay: NodeDecl,
{
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
where
    Base: NodeDecl<Element = EB>,
    Overlay: NodeDecl<Element = EO>,
    Blend: BoundAttrDecl<Element = f64>,
    EB: Lerp,
    EO: Into<EB>,
{
    type Element = EB;
    type Target = OverlayNode<Base::Target, Overlay::Target, Blend::Target>;

    fn materialize(self, _size: usize, builder: &mut NodeBuilder) -> Result<Self::Target> {
        return Ok(Self::Target {
            base: builder.node("base", self.base)?,
            overlay: builder.node("overlay", self.overlay)?,
            blend: builder.bound_attr("blend", self.blend, Bounds::normal())?,
        });
    }
}

impl<'a, Base, Overlay, Blend> RenderType<'a, Self> for OverlayNode<Base, Overlay, Blend>
where
    Base: Node,
    Overlay: Node,
    Blend: Attr<Element = f64>,
    Base::Element: Lerp,
    Overlay::Element: Into<Base::Element>,
{
    type Render = OverlayRenderer<
        <Base as RenderType<'a, Base>>::Render,
        <Overlay as RenderType<'a, Overlay>>::Render,
    >;
}

impl<Base, Overlay, Blend> Node for OverlayNode<Base, Overlay, Blend>
where
    Base: Node,
    Overlay: Node,
    Blend: Attr<Element = f64>,
    Base::Element: Lerp,
    Overlay::Element: Into<Base::Element>,
{
    const KIND: &'static str = "overlay";

    type Element = Base::Element;

    fn update(&mut self, duration: Duration) -> Result<()> {
        self.base.update(duration)?;
        self.overlay.update(duration)?;

        self.blend.update(duration);

        return Ok(());
    }

    fn render(&mut self) -> Result<<Self as RenderType<Self>>::Render> {
        return Ok(OverlayRenderer {
            base: self.base.render()?,
            overlay: self.overlay.render()?,
            blend: self.blend.get(),
        });
    }
}

#[cfg(feature = "dyn")]
pub mod model {
    use photonic_dyn::config;
    use photonic_dyn::model::NodeModel;
    use photonic_dyn::builder::NodeBuilder;
    use photonic_core::boxed::{BoxedNodeDecl, Wrap};
    use photonic_core::color;

    use anyhow::Result;
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct OverlayConfig {
        pub base: config::Node,
        pub overlay: config::Node,
        pub blend: config::Attr,
    }

    impl NodeModel for OverlayConfig {
        fn assemble(self, builder: &mut impl NodeBuilder) -> Result<BoxedNodeDecl<color::RGBColor>> {
            return Ok(BoxedNodeDecl::wrap(
                super::OverlayNodeDecl {
                    base: builder.node("base", self.base)?,
                    overlay: builder.node("overlay", self.overlay)?,
                    blend: builder.bound_attr("blend", self.blend)?,
                },
            ));
        }
    }
}

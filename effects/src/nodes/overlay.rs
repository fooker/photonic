use anyhow::Result;
use palette::IntoColor;

use photonic::attr::Bounds;
use photonic::math::Lerp;
use photonic::{
    Attr, BoundAttrDecl, Buffer, BufferReader, Node, NodeBuilder, NodeDecl, NodeHandle, NodeRef, RenderContext,
};

// TODO: Support blend modes

pub struct Overlay<Base, Pave, Blend>
where
    Base: NodeDecl,
    Pave: NodeDecl,
{
    pub base: NodeHandle<Base>,
    pub pave: NodeHandle<Pave>,
    pub blend: Blend,
}

pub struct OverlayNode<Base, Pave, Blend>
where
    Base: Node + 'static,
    Pave: Node + 'static,
{
    base: NodeRef<Base>,
    pave: NodeRef<Pave>,

    blend: Blend,
}

impl<Base, Pave, Blend> NodeDecl for Overlay<Base, Pave, Blend>
where
    Base: NodeDecl + 'static,
    Pave: NodeDecl + 'static,
    Blend: BoundAttrDecl<f32>,
    <<Base as NodeDecl>::Node as Node>::Element: Lerp,
    <<Pave as NodeDecl>::Node as Node>::Element: IntoColor<<<Base as NodeDecl>::Node as Node>::Element>,
{
    const KIND: &'static str = "overlay";

    type Node = OverlayNode<Base::Node, Pave::Node, Blend::Attr>;

    async fn materialize(self, builder: &mut NodeBuilder<'_>) -> Result<Self::Node> {
        return Ok(Self::Node {
            base: builder.node("base", self.base).await?,
            pave: builder.node("pave", self.pave).await?,
            blend: builder.bound_attr("blend", self.blend, Bounds::normal())?,
        });
    }
}

impl<Base, Pave, Blend> Node for OverlayNode<Base, Pave, Blend>
where
    Base: Node,
    Pave: Node,
    Blend: Attr<f32>,
    Base::Element: Lerp,
    Pave::Element: IntoColor<Base::Element>,
{
    type Element = Base::Element;

    fn update(&mut self, ctx: &RenderContext, out: &mut Buffer<Self::Element>) -> Result<()> {
        let base = &ctx[self.base];
        let pave = &ctx[self.pave];

        let blend = self.blend.update(ctx);

        out.update(|i, _| {
            let base = base.get(i);
            let pave = pave.get(i);

            // TODO: Blending modes
            return Self::Element::lerp(base, pave.into_color(), blend);
        });

        return Ok(());
    }
}

#[cfg(feature = "dynamic")]
pub mod dynamic {
    use palette::rgb::Rgb;
    use serde::Deserialize;

    use photonic::boxed::{BoxedBoundAttrDecl, BoxedNodeDecl, DynNodeDecl};
    use photonic_dynamic::factory::Producible;
    use photonic_dynamic::registry::Registry;
    use photonic_dynamic::{builder, config};

    use super::*;

    #[derive(Deserialize, Debug)]
    pub struct Config {
        pub base: config::Node,
        pub pave: config::Node,
        pub blend: config::Attr<f32>,
    }

    impl Producible<dyn DynNodeDecl<Rgb>> for Config {
        type Product = Overlay<BoxedNodeDecl<Rgb>, BoxedNodeDecl<Rgb>, BoxedBoundAttrDecl<f32>>;
        fn produce<Reg: Registry>(config: Self, mut builder: builder::NodeBuilder<'_, Reg>) -> Result<Self::Product> {
            return Ok(Overlay {
                base: builder.node("base", config.base)?,
                pave: builder.node("pave", config.pave)?,
                blend: builder.bound_attr("blend", config.blend)?,
            });
        }
    }
}

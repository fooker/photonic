use anyhow::Result;
use palette::IntoColor;

use photonic::{Attr, BoundAttrDecl, Buffer, BufferReader, Context, Node, NodeBuilder, NodeDecl, NodeHandle, NodeRef};
use photonic::attr::Bounds;
use photonic::math::Lerp;

pub struct Overlay<Base, Pave, Blend>
    where Base: NodeDecl,
          Pave: NodeDecl,
{
    pub base: NodeHandle<Base>,
    pub pave: NodeHandle<Pave>,

    pub blend: Blend,
}

pub struct OverlayNode<Base, Pave, Blend>
    where Base: Node + 'static,
          Pave: Node + 'static,
          Blend: Attr<Value=f32>, {
    base: NodeRef<Base>,
    pave: NodeRef<Pave>,

    blend: Blend,
}

impl<Base, Pave, Blend> NodeDecl for Overlay<Base, Pave, Blend>
    where Base: NodeDecl + 'static,
          Pave: NodeDecl + 'static,
          Blend: BoundAttrDecl<Value=f32>,
          <<Base as NodeDecl>::Node as Node>::Element: Lerp,
          <<Pave as NodeDecl>::Node as Node>::Element: IntoColor<<<Base as NodeDecl>::Node as Node>::Element>,
{
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
        Blend: Attr<Value=f32>,
        Base::Element: Lerp,
        Pave::Element: IntoColor<Base::Element>,
{
    const KIND: &'static str = "overlay";

    type Element = Base::Element;

    fn update(&mut self, ctx: &Context, out: &mut Buffer<Self::Element>) -> Result<()> {
        let base = &ctx[self.base];
        let pave = &ctx[self.pave];

        let blend = self.blend.update(ctx.duration);

        out.update(|i, _| {
            let base = base.get(i);
            let pave = pave.get(i);

            // TODO: Blending modes
            return Self::Element::lerp(base, pave.into_color(), blend);
        });

        return Ok(());
    }
}

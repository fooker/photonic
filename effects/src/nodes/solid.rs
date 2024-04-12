use anyhow::Result;
use palette::rgb::Rgb;

use photonic::attr::Attr;
use photonic::decl::{FreeAttrDecl, NodeDecl};
use photonic::{Buffer, RenderContext, Node, NodeBuilder};
use photonic_dyn::DynamicNode;

#[derive(DynamicNode)]
pub struct Solid<Color> {
    #[photonic(attr)]
    pub color: Color,
}

pub struct SolidNode<Color>
where Color: Attr<Value = Rgb>
{
    color: Color,
}

impl<Color> NodeDecl for Solid<Color>
where Color: FreeAttrDecl<Value = Rgb>
{
    type Node = SolidNode<Color::Attr>;

    async fn materialize(self, builder: &mut NodeBuilder<'_>) -> Result<Self::Node> {
        return Ok(Self::Node {
            color: builder.unbound_attr("color", self.color)?,
        });
    }
}

impl<Color> Node for SolidNode<Color>
where Color: Attr<Value = Rgb>
{
    const KIND: &'static str = "solid";

    type Element = Rgb;

    fn update(&mut self, ctx: &RenderContext, out: &mut Buffer<Self::Element>) -> Result<()> {
        let color = self.color.update(ctx.duration);

        out.fill(color);

        return Ok(());
    }
}

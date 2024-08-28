use anyhow::Result;
use palette::rgb::Rgb;

use photonic::{Buffer, Node, NodeBuilder, RenderContext};
use photonic::attr::Attr;
use photonic::decl::{FreeAttrDecl, NodeDecl};

pub struct Solid<Color> {
    pub color: Color,
}

pub struct SolidNode<Color>
    where Color: Attr<Value=Rgb>
{
    color: Color,
}

impl<Color> NodeDecl for Solid<Color>
    where Color: FreeAttrDecl<Value=Rgb>
{
    type Node = SolidNode<Color::Attr>;

    async fn materialize(self, builder: &mut NodeBuilder<'_>) -> Result<Self::Node> {
        return Ok(Self::Node {
            color: builder.unbound_attr("color", self.color)?,
        });
    }
}

impl<Color> Node for SolidNode<Color>
    where Color: Attr<Value=Rgb>
{
    const KIND: &'static str = "solid";

    type Element = Rgb;

    fn update(&mut self, ctx: &RenderContext, out: &mut Buffer<Self::Element>) -> Result<()> {
        let color = self.color.update(ctx.duration);

        out.fill(color);

        return Ok(());
    }
}

#[cfg(feature = "dynamic")]
pub mod dynamic {
    use serde::Deserialize;

    use photonic_dynamic::{BoxedFreeAttrDecl, config};
    use photonic_dynamic::factory::Producible;

    use super::*;

    #[derive(Deserialize, Debug)]
    pub struct Config {
        pub color: config::Attr<Rgb>,
    }

    impl Producible for Solid<BoxedFreeAttrDecl<Rgb>> {
        type Config = Config;
    }

    pub fn node<B>(config: Config, builder: &mut B) -> Result<Solid<BoxedFreeAttrDecl<Rgb>>>
        where
            B: photonic_dynamic::NodeBuilder,
    {
        return Ok(Solid {
            color: builder.free_attr("color", config.color)?,
        });
    }
}

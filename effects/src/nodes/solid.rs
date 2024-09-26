use anyhow::Result;
use palette::rgb::Rgb;

use photonic::attr::Attr;
use photonic::decl::{FreeAttrDecl, NodeDecl};
use photonic::{Buffer, Node, NodeBuilder, RenderContext};

pub struct Solid<Color> {
    pub color: Color,
}

pub struct SolidNode<Color>
where Color: Attr<Rgb>
{
    color: Color,
}

impl<Color> NodeDecl for Solid<Color>
where Color: FreeAttrDecl<Rgb>
{
    type Node = SolidNode<Color::Attr>;

    async fn materialize(self, builder: &mut NodeBuilder<'_>) -> Result<Self::Node> {
        return Ok(Self::Node {
            color: builder.unbound_attr("color", self.color)?,
        });
    }
}

impl<Color> Node for SolidNode<Color>
where Color: Attr<Rgb>
{
    const KIND: &'static str = "solid";

    type Element = Rgb;

    fn update(&mut self, ctx: &RenderContext, out: &mut Buffer<Self::Element>) -> Result<()> {
        let color = self.color.update(ctx);

        out.fill(color);

        return Ok(());
    }
}

#[cfg(feature = "dynamic")]
pub mod dynamic {
    use serde::Deserialize;

    use photonic_dynamic::factory::Producible;
    use photonic_dynamic::registry::Registry;
    use photonic_dynamic::{builder, config, BoxedFreeAttrDecl, DynNodeDecl};

    use super::*;

    #[derive(Deserialize, Debug)]
    pub struct Config {
        pub color: config::Attr<Rgb>,
    }

    impl Producible<dyn DynNodeDecl> for Config {
        type Product = Solid<BoxedFreeAttrDecl<Rgb>>;
        fn produce<Reg: Registry>(config: Self, mut builder: builder::NodeBuilder<'_, Reg>) -> Result<Self::Product> {
            return Ok(Solid {
                color: builder.free_attr("color", config.color)?,
            });
        }
    }
}

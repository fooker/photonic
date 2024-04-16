use anyhow::Result;
use palette::rgb::Rgb;
use serde::Deserialize;

use photonic::attr::Attr;
use photonic::decl::{FreeAttrDecl, NodeDecl};
use photonic::{Buffer, Node, NodeBuilder, RenderContext};
use photonic_dynamic::boxed::DynNodeDecl;
use photonic_dynamic::{config, NodeFactory};

pub struct Solid<Color> {
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

#[cfg(feature = "dynamic")]
pub fn factory<B>() -> Box<NodeFactory<B>>
where B: photonic_dynamic::NodeBuilder {
    #[derive(Deserialize, Debug)]
    struct Config {
        pub color: config::Attr,
    }

    return Box::new(|config: config::Anything, builder: &mut B| {
        let config: Config = Deserialize::deserialize(config)?;

        return Ok(Box::new(Solid {
            color: builder.free_attr("color", config.color)?,
        }) as Box<dyn DynNodeDecl>);
    });
}

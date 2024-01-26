use anyhow::Result;
use palette::Darken;

use photonic::{Buffer, Context, Node, NodeBuilder, NodeDecl, NodeHandle, NodeRef};

pub struct Brightness<Source>
    where Source: NodeDecl + 'static,
{
    pub brightness: f32,
    // TODO: Make this an attr
    pub source: NodeHandle<Source>,
}

pub struct BrightnessNode<Source>
    where Source: Node + 'static,
{
    brightness: f32,
    // TODO: Make this an attr
    source: NodeRef<Source>,
}

impl<Source> NodeDecl for Brightness<Source>
    where Source: NodeDecl,
          <Source::Node as Node>::Element: Darken<Scalar=f32> + Default, // TODO: Remove default constrain
{
    type Node = BrightnessNode<Source::Node>;

    fn materialize(self, builder: &mut NodeBuilder) -> Result<Self::Node> {
        return Ok(Self::Node {
            brightness: self.brightness,
            source: builder.node("source", self.source)?,
        });
    }
}

impl<Source> Node for BrightnessNode<Source>
    where Source: Node + 'static,
          Source::Element: Darken<Scalar=f32>,
{
    const KIND: &'static str = "brightness";
    type Element = Source::Element;

    fn update(&mut self, ctx: &Context, out: &mut Buffer<Self::Element>) -> Result<()> {
        let source = &ctx[&self.source];

        out.update_from(source.iter()
            .map(|c| c.darken(1.0 - self.brightness)));

        return Ok(());
    }
}
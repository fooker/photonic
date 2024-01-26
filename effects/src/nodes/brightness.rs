use anyhow::Result;
use palette::Darken;

use photonic::{Attr, BoundAttrDecl, Buffer, Context, Node, NodeBuilder, NodeDecl, NodeHandle, NodeRef};
use photonic::attr::Bounds;

pub struct Brightness<Source, Value>
    where Source: NodeDecl,
          Value: BoundAttrDecl<Value=f32>,
{
    pub value: Value,
    pub source: NodeHandle<Source>,
}

pub struct BrightnessNode<Source, Value>
    where Source: Node + 'static,
          Value: Attr<Value=f32>,
{
    value: Value,
    source: NodeRef<Source>,
}

impl<Source, Value> NodeDecl for Brightness<Source, Value>
    where Source: NodeDecl + 'static,
          Value: BoundAttrDecl<Value=f32>,
          <Source::Node as Node>::Element: Darken<Scalar=f32> + Default, // TODO: Remove default constrain
{
    type Node = BrightnessNode<Source::Node, Value::Attr>;

    async fn materialize(self, builder: &mut NodeBuilder<'_>) -> Result<Self::Node> {
        return Ok(Self::Node {
            value: builder.bound_attr("value", self.value, Bounds::normal())?,
            source: builder.node("source", self.source).await?,
        });
    }
}

impl<Source, Value> Node for BrightnessNode<Source, Value>
    where Source: Node,
          Value: Attr<Value=f32>,
          Source::Element: Darken<Scalar=f32>,
{
    const KIND: &'static str = "brightness";
    type Element = Source::Element;

    fn update(&mut self, ctx: &Context, out: &mut Buffer<Self::Element>) -> Result<()> {
        let value = self.value.update(ctx.duration);
        let source = &ctx[&self.source];

        out.update_from(source.iter()
            .map(|c| c.darken(1.0 - value)));

        return Ok(());
    }
}
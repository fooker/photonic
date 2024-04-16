use anyhow::Result;
use serde::Deserialize;
use std::ops::Range;

use photonic::attr::Bounds;
use photonic::math::Lerp;
use photonic::{
    Attr, BoundAttrDecl, Buffer, BufferReader, Node, NodeBuilder, NodeDecl, NodeHandle, NodeRef, RenderContext,
};
use photonic_dynamic::boxed::DynNodeDecl;
use photonic_dynamic::{config, NodeFactory};

pub struct Brightness<Source, Value>
where Source: NodeDecl
{
    pub source: NodeHandle<Source>,
    pub value: Value,

    pub range: Option<Range<usize>>,
}

pub struct BrightnessNode<Source, Value>
where
    Source: Node + 'static,
    Value: Attr<Value = f32>,
    Source::Element: Lerp,
{
    source: NodeRef<Source>,

    value: Value,
    range: Range<usize>,
}

impl<Source, Value> NodeDecl for Brightness<Source, Value>
where
    Source: NodeDecl + 'static,
    Value: BoundAttrDecl<Value = f32>,
    <Source::Node as Node>::Element: Lerp + Default, // TODO: Remove default constrain
{
    type Node = BrightnessNode<Source::Node, Value::Attr>;

    async fn materialize(self, builder: &mut NodeBuilder<'_>) -> Result<Self::Node> {
        return Ok(Self::Node {
            value: builder.bound_attr("value", self.value, Bounds::normal())?,
            source: builder.node("source", self.source).await?,
            range: self.range.unwrap_or(0..builder.size),
        });
    }
}

impl<Source, Value> Node for BrightnessNode<Source, Value>
where
    Source: Node,
    Value: Attr<Value = f32>,
    Source::Element: Lerp,
{
    const KIND: &'static str = "brightness";

    type Element = Source::Element;

    fn update(&mut self, ctx: &RenderContext, out: &mut Buffer<Self::Element>) -> Result<()> {
        let value = self.value.update(ctx.duration);
        let source = &ctx[self.source];

        // TODO: Use better brightness algo here
        out.blit_from(source.map_range(&self.range, |c| Lerp::lerp(Self::Element::default(), c, value)));

        return Ok(());
    }
}

#[cfg(feature = "dynamic")]
pub fn factory<B>() -> Box<NodeFactory<B>>
where B: photonic_dynamic::NodeBuilder {
    #[derive(Deserialize, Debug)]
    struct Config {
        pub source: config::Node,
        pub value: config::Attr,

        pub range: Option<Range<usize>>,
    }

    return Box::new(|config: config::Anything, builder: &mut B| {
        let config: Config = Deserialize::deserialize(config)?;

        return Ok(Box::new(Brightness {
            source: builder.node("source", config.source)?,
            value: builder.bound_attr("value", config.value)?,
            range: config.range,
        }) as Box<dyn DynNodeDecl>);
    });
}

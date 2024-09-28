use anyhow::Result;
use palette::rgb::Rgb;
use std::time::Duration;

use photonic::boxed::{Boxed, BoxedNode, BoxedNodeDecl, DynNodeDecl};
use photonic::{
    Attr, BoundAttrDecl, Buffer, BufferReader, Node, NodeBuilder, NodeDecl, NodeHandle, NodeRef, RenderContext,
};

use crate::easing::{Easing, Easings};

pub struct Select<Value>
where Value: BoundAttrDecl<usize>
{
    value: Value,

    sources: Vec<NodeHandle<BoxedNodeDecl>>,

    easing: Easing<f32>,
}

impl<Value> Select<Value>
where Value: BoundAttrDecl<usize>
{
    pub fn with_value(value: Value) -> Self {
        return Self {
            value,
            sources: vec![],
            easing: Easings::Instant.with_speed(Duration::ZERO),
        };
    }

    pub fn with_easing(mut self, easing: impl Into<Easing<f32>>) -> Self {
        self.easing = easing.into();
        return self;
    }

    pub fn with_source(mut self, source: NodeHandle<impl NodeDecl + Boxed<dyn DynNodeDecl>>) -> Self {
        self.sources.push(source.boxed());
        return self;
    }
}

pub struct SelectNode<Value>
where Value: Attr<usize>
{
    sources: Vec<NodeRef<BoxedNode>>,
    value: Value,

    last: Option<usize>,
    next: Option<usize>,

    fade: f32,

    easing: Easing<f32>,
}

impl<Value> NodeDecl for Select<Value>
where Value: BoundAttrDecl<usize>
{
    const KIND: &'static str = "select";

    type Node = SelectNode<Value::Attr>;

    async fn materialize(self, builder: &mut NodeBuilder<'_>) -> Result<Self::Node> {
        let mut sources = Vec::new();
        for (i, source) in self.sources.into_iter().enumerate() {
            sources.push(builder.node(format!("source-{i}"), source).await?);
        }

        let value = builder.bound_attr("value", self.value, (0, sources.len() - 1))?;

        return Ok(Self::Node {
            sources,
            value,
            last: None,
            next: None,
            fade: 0.0,
            easing: self.easing,
        });
    }
}

impl<Value> Node for SelectNode<Value>
where Value: Attr<usize>
{
    type Element = Rgb;

    fn update(&mut self, ctx: &RenderContext, out: &mut Buffer<Self::Element>) -> Result<()> {
        let curr = self.value.update(ctx);

        let Some(last) = self.last else {
            // First cycle - set initial current value
            self.last = Some(curr);

            out.blit_from(&ctx[self.sources[curr]]);
            return Ok(());
        };

        if let Some(next) = self.next {
            // In transition
            self.fade += ctx.duration.as_secs_f32() / self.easing.speed.as_secs_f32();

            if self.fade >= 1.0 {
                // Transition finished
                self.last = Some(next);
                self.next = None;

                out.blit_from(&ctx[self.sources[next]]);
                return Ok(());
            }

            let source = &ctx[self.sources[last]];
            let target = &ctx[self.sources[next]];

            out.blit_from(BufferReader::lerp(source, target, (self.easing.func)(self.fade)));
            return Ok(());
        }

        if curr != last {
            // Start transition
            self.next = Some(curr);
            self.fade = 0.0;
        }

        out.blit_from(&ctx[self.sources[last]]);
        return Ok(());
    }
}

#[cfg(feature = "dynamic")]
pub mod dynamic {
    use serde::Deserialize;

    use photonic::boxed::BoxedBoundAttrDecl;
    use photonic_dynamic::factory::Producible;
    use photonic_dynamic::registry::Registry;
    use photonic_dynamic::{builder, config};

    use super::*;

    #[derive(Deserialize, Debug)]
    pub struct Config {
        pub sources: Vec<config::Node>,
        pub value: config::Attr<usize>,
        pub easing: Easing<f32>,
    }

    impl Producible<dyn DynNodeDecl> for Config {
        type Product = Select<BoxedBoundAttrDecl<usize>>;
        fn produce<Reg: Registry>(config: Self, mut builder: builder::NodeBuilder<'_, Reg>) -> Result<Self::Product> {
            let sources = config
                .sources
                .into_iter()
                .enumerate()
                .map(|(i, source)| builder.node(&format!("sources.{}", i), source))
                .collect::<Result<_>>()?;

            return Ok(Select {
                sources,
                value: builder.bound_attr("value", config.value)?,
                easing: config.easing,
            });
        }
    }
}

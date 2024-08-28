use std::time::Duration;

use anyhow::Result;

use photonic::{
    Attr, BoundAttrDecl, Buffer, BufferReader, Node, NodeBuilder, NodeDecl, NodeHandle, NodeRef, RenderContext,
};
use photonic::math::Lerp;

use crate::easing::Easing;

pub struct Switch<Source, Value>
    where
        Source: NodeDecl,
        Value: BoundAttrDecl<Value=usize>,
        <Source::Node as Node>::Element: Lerp + Default, // TODO: Remove default constrain
{
    pub sources: Vec<NodeHandle<Source>>,
    pub value: Value,

    pub easing: Easing<f32>,
}

pub struct SwitchNode<Source, Value>
    where
        Source: Node + 'static,
        Value: Attr<Value=usize>,
        Source::Element: Lerp,
{
    sources: Vec<NodeRef<Source>>,
    value: Value,

    last: usize,
    next: usize,

    fade: f32,

    easing: Easing<f32>,
}

impl<Source, Value> NodeDecl for Switch<Source, Value>
    where
        Source: NodeDecl + 'static,
        Value: BoundAttrDecl<Value=usize>,
        <Source::Node as Node>::Element: Lerp + Default, // TODO: Remove default constrain
{
    type Node = SwitchNode<Source::Node, Value::Attr>;

    async fn materialize(self, builder: &mut NodeBuilder<'_>) -> Result<Self::Node> {
        let mut sources = Vec::new();
        for (i, source) in self.sources.into_iter().enumerate() {
            sources.push(builder.node(format!("source-{}", i), source).await?);
        }

        let mut value = builder.bound_attr("value", self.value, (0, sources.len() - 1))?;

        let current = value.update(Duration::ZERO);

        return Ok(Self::Node {
            sources,
            value,
            last: current,
            next: current,
            fade: 0.0,
            easing: self.easing,
        });
    }
}

impl<Source, Value> Node for SwitchNode<Source, Value>
    where
        Source: Node,
        Value: Attr<Value=usize>,
        Source::Element: Lerp,
{
    const KIND: &'static str = "switch";

    type Element = Source::Element;

    fn update(&mut self, ctx: &RenderContext, out: &mut Buffer<Self::Element>) -> Result<()> {
        // This handles full transitions only - as long as a transition is in progress, we do not care about value
        // updates but keep running the transition
        if self.last == self.next {
            // No transition in progress
            let next = self.value.update(ctx.duration);
            if next != self.next {
                // Start a new transition
                self.next = next;
                self.fade = 0.0;
            }

            out.blit_from(&ctx[self.sources[self.next]]);
        } else {
            // Transition in progress
            self.fade += ctx.duration.as_secs_f32() / self.easing.speed.as_secs_f32();

            if self.fade >= 1.0 {
                // Transition finished
                self.last = self.next;
                self.fade = 0.0;
            }

            let source = &ctx[self.sources[self.last]];
            let target = &ctx[self.sources[self.next]];

            out.blit_from(BufferReader::lerp(source, target, (self.easing.func)(self.fade)));
        }

        return Ok(());
    }
}

#[cfg(feature = "dynamic")]
pub mod dynamic {
    use serde::Deserialize;

    use photonic_dynamic::{BoxedBoundAttrDecl, BoxedNodeDecl, config};
    use photonic_dynamic::factory::Producible;

    use crate::easing::Easings;

    use super::*;

    #[derive(Deserialize, Debug)]
    pub struct Config {
        pub sources: Vec<config::Node>,
        pub value: config::Attr<i64>,
        pub easing_func: Easings,
        pub easing_speed: Duration,
    }

    impl Producible for Switch<BoxedNodeDecl, BoxedBoundAttrDecl<usize>> {
        type Config = Config;
    }

    pub fn node<B>(config: Config, builder: &mut B) -> Result<Switch<BoxedNodeDecl, BoxedBoundAttrDecl<usize>>>
        where
            B: photonic_dynamic::NodeBuilder,
    {
        let sources = config
            .sources
            .into_iter()
            .enumerate()
            .map(|(i, source)| builder.node(&format!("sources.{}", i), source))
            .collect::<Result<_>>()?;

        return Ok(Switch {
            sources,
            value: todo!(), // builder.bound_attr("value", config.value)?,
            easing: config.easing_func.with_speed(config.easing_speed),
        });
    }
}

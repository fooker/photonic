use std::ops::Range;

use anyhow::Result;

use photonic::attr::Bounds;
use photonic::math::Lerp;
use photonic::{
    Attr, BoundAttrDecl, Buffer, BufferReader, Node, NodeBuilder, NodeDecl, NodeHandle, NodeRef, RenderContext,
};

pub struct Blackout<Source, Active>
where
    Source: NodeDecl + 'static,
    Active: BoundAttrDecl<f32>,
{
    pub source: NodeHandle<Source>,
    pub active: Active,

    pub value: <<Source as NodeDecl>::Node as Node>::Element,
    pub range: Option<Range<usize>>,
}

pub struct BlackoutNode<Source, Active>
where
    Source: Node + 'static,
    Active: Attr<f32>,
{
    source: NodeRef<Source>,
    active: Active,

    value: <Source as Node>::Element,
    range: Range<usize>,
}

impl<Source, Active> NodeDecl for Blackout<Source, Active>
where
    Source: NodeDecl + 'static,
    <<Source as NodeDecl>::Node as Node>::Element: Lerp,
    Active: BoundAttrDecl<f32>,
{
    type Node = BlackoutNode<Source::Node, Active::Attr>;

    async fn materialize(self, builder: &mut NodeBuilder<'_>) -> Result<Self::Node> {
        return Ok(Self::Node {
            source: builder.node("source", self.source).await?,
            active: builder.bound_attr("active", self.active, Bounds::normal())?,
            value: self.value,
            range: self.range.unwrap_or(0..builder.size),
        });
    }
}

impl<Source, Active> Node for BlackoutNode<Source, Active>
where
    Source: Node,
    <Source as Node>::Element: Lerp,
    Active: Attr<f32>,
{
    const KIND: &'static str = "blackout";

    type Element = Source::Element;

    fn update(&mut self, ctx: &RenderContext, out: &mut Buffer<Self::Element>) -> Result<()> {
        let source = &ctx[self.source];

        let active = self.active.update(ctx);

        if active > 0.0 {
            let source = source.map_range(&self.range, |e| Lerp::lerp(e, self.value, active));
            out.blit_from(source);
        } else {
            out.blit_from(source);
        }

        return Ok(());
    }
}

#[cfg(feature = "dynamic")]
pub mod dynamic {
    use palette::rgb::Rgb;
    use serde::Deserialize;

    use photonic_dynamic::factory::Producible;
    use photonic_dynamic::registry::Registry;
    use photonic_dynamic::{builder, config, BoxedBoundAttrDecl, BoxedNodeDecl, DynNodeDecl};

    use super::*;

    #[derive(Deserialize, Debug)]
    pub struct Config {
        pub source: config::Node,
        pub active: config::Attr<f32>,

        pub value: Rgb,
        pub range: Option<Range<usize>>,
    }

    impl Producible<dyn DynNodeDecl> for Config {
        type Product = Blackout<BoxedNodeDecl, BoxedBoundAttrDecl<f32>>;
        fn produce<Reg: Registry>(config: Self, mut builder: builder::NodeBuilder<'_, Reg>) -> Result<Self::Product> {
            return Ok(Blackout {
                source: builder.node("source", config.source)?,
                active: builder.bound_attr("active", config.active)?,
                value: config.value,
                range: config.range,
            });
        }
    }
}

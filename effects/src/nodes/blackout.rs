use anyhow::Result;
use palette::IntoColor;

use photonic::{Attr, Buffer, Context, FreeAttrDecl, Node, NodeBuilder, NodeDecl, NodeHandle, NodeRef};
use photonic::math::Lerp;

pub struct Blackout<Source, Active, Element>
    where Source: NodeDecl,
          Active: FreeAttrDecl<Value=bool>,
          Element: IntoColor<<<Source as NodeDecl>::Node as Node>::Element>,
{
    pub source: NodeHandle<Source>,
    pub active: Active,

    pub value: Element,
    pub range: Option<(usize, usize)>,
}

pub struct BlackoutNode<Source, Active, Element>
    where Source: Node + 'static,
          Active: Attr<Value=bool>,
          Element: IntoColor<<Source as Node>::Element> + Clone,
{
    source: NodeRef<Source>,
    active: Active,

    value: Element,
    range: (usize, usize),
}

impl<Source, Active, Element> NodeDecl for Blackout<Source, Active, Element>
    where Source: NodeDecl + 'static,
          Active: FreeAttrDecl<Value=bool>,
          Element: IntoColor<<<Source as NodeDecl>::Node as Node>::Element> + Clone,
{
    type Node = BlackoutNode<Source::Node, Active::Attr, Element>;

    async fn materialize(self, builder: &mut NodeBuilder<'_>) -> Result<Self::Node> {
        return Ok(Self::Node {
            source: builder.node("source", self.source).await?,
            active: builder.unbound_attr("active", self.active)?,
            value: self.value,
            range: self.range.unwrap_or((0, builder.size - 1)),
        });
    }
}

impl<Source, Active, Element> Node for BlackoutNode<Source, Active, Element>
    where Source: Node,
          Active: Attr<Value=bool>,
          Element: IntoColor<<Source as Node>::Element> + Clone,
{
    const KIND: &'static str = "blackout";

    type Element = Source::Element;

    fn update(&mut self, ctx: &Context, out: &mut Buffer<Self::Element>) -> Result<()> {
        let source = &ctx[&self.source];

        let active = self.active.update(ctx.duration);

        out.update(|i, _| if self.range.0 <= i && i <= self.range.1 && active {
            self.value.clone().into_color()
        } else {
            *source.get(i)
        });

        return Ok(());
    }
}

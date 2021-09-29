use std::marker::PhantomData;
use std::time::Duration;

use anyhow::Error;
use noise::{NoiseFn, Perlin, Seedable};

use photonic_core::attr::{Attr, AttrValue, Range, UnboundAttrDecl};
use photonic_core::math;
use photonic_core::math::Lerp;
use photonic_core::node::{Node, NodeDecl, Render, RenderType};
use photonic_core::scene::NodeBuilder;

pub struct PlasmaRenderer<'a, E>
where
    E: AttrValue + Lerp,
{
    noise: &'a Perlin,

    range: Range<E>,
    scale: f64,

    time: f64,
}

impl<'a, E> Render for PlasmaRenderer<'a, E>
where
    E: AttrValue + Lerp,
{
    type Element = E;

    fn get(&self, index: usize) -> Self::Element {
        let i = self
            .noise
            .get([index as f64 / self.scale, self.time / self.scale]);
        let i = math::remap(i, (-1.0, 1.0), (0.0, 1.0));

        return E::lerp(self.range.0, self.range.1, i);
    }
}

pub struct PlasmaNodeDecl<Range, Scale, Speed, E>
where
    E: AttrValue + Lerp,
{
    pub range: Range,
    pub scale: Scale,
    pub speed: Speed,

    pub phantom: PhantomData<E>,
}

pub struct PlasmaNode<Range, Scale, Speed, E>
where
    E: AttrValue + Lerp,
{
    perlin: Perlin,

    range: Range,
    scale: Scale,
    speed: Speed,

    time: f64,

    phantom: PhantomData<E>,
}

impl<Range, Scale, Speed, E> NodeDecl for PlasmaNodeDecl<Range, Scale, Speed, E>
where
    Range: UnboundAttrDecl<self::Range<E>>,
    Scale: UnboundAttrDecl<f64>,
    Speed: UnboundAttrDecl<f64>,
    E: AttrValue + Lerp,
{
    type Element = E;
    type Target = PlasmaNode<Range::Target, Scale::Target, Speed::Target, E>;

    fn materialize(self, _size: usize, builder: &mut NodeBuilder) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            perlin: Perlin::new().set_seed(1),

            range: builder.unbound_attr("range", self.range)?,
            scale: builder.unbound_attr("scale", self.scale)?,
            speed: builder.unbound_attr("speed", self.speed)?,

            time: 0.0,

            phantom: self.phantom,
        });
    }
}

impl<'a, Range, Scale, Speed, E> RenderType<'a, Self> for PlasmaNode<Range, Scale, Speed, E>
where
    Range: Attr<self::Range<E>>,
    Scale: Attr<f64>,
    Speed: Attr<f64>,
    E: AttrValue + Lerp,
{
    type Render = PlasmaRenderer<'a, E>;
}

impl<Range, Scale, Speed, E> Node for PlasmaNode<Range, Scale, Speed, E>
where
    Range: Attr<self::Range<E>>,
    Scale: Attr<f64>,
    Speed: Attr<f64>,
    E: AttrValue + Lerp,
{
    const KIND: &'static str = "plasma";

    type Element = E;

    fn update(&mut self, duration: Duration) {
        self.range.update(duration);
        self.scale.update(duration);
        self.speed.update(duration);

        self.time += duration.as_secs_f64() * self.speed.get();
    }

    fn render(&mut self) -> <Self as RenderType<Self>>::Render {
        return PlasmaRenderer {
            noise: &self.perlin,
            range: self.range.get(),
            scale: self.scale.get(),
            time: self.time,
        };
    }
}

#[cfg(feature = "dyn")]
pub mod model {
    use photonic_dyn::config;
    use photonic_dyn::model::NodeModel;
    use photonic_dyn::builder::NodeBuilder;
    use photonic_core::boxed::{BoxedNodeDecl, Wrap};
    use photonic_core::color;

    use anyhow::Result;
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct PlasmaConfig {
        pub range: config::Attr,
        pub scale: config::Attr,
        pub speed: config::Attr,
    }

    impl NodeModel for PlasmaConfig {
        fn assemble(self, builder: &mut impl NodeBuilder) -> Result<BoxedNodeDecl<color::RGBColor>> {
            return Ok(BoxedNodeDecl::wrap(
                super::PlasmaNodeDecl {
                    range: builder.unbound_attr("range", self.range)?,
                    scale: builder.unbound_attr("scale", self.scale)?,
                    speed: builder.unbound_attr("speed", self.speed)?,
                    phantom: Default::default(),
                },
            ));
        }
    }
}

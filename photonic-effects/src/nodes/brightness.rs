use std::time::Duration;

use anyhow::Error;

use photonic_core::attr::{Attr, BoundAttrDecl, Bounds};
use photonic_core::color::Black;
use photonic_core::math::Lerp;
use photonic_core::node::{Node, NodeDecl, Render, RenderType};
use photonic_core::scene::{NodeBuilder, NodeHandle};

pub struct BrightnessRenderer<Source> {
    source: Source,

    range: (usize, usize),
    brightness: f64,
}

impl<Source> Render for BrightnessRenderer<Source>
where
    Source: Render,
    Source::Element: Lerp + Black,
{
    type Element = Source::Element;

    fn get(&self, index: usize) -> Self::Element {
        let value = self.source.get(index);

        if self.range.0 <= index && index <= self.range.1 {
            return Self::Element::lerp(Self::Element::black(), value, self.brightness);
        } else {
            return value;
        }
    }
}

pub struct BrightnessNodeDecl<Source, Brightness>
where
    Source: NodeDecl,
    Brightness: BoundAttrDecl<f64>,
{
    pub source: NodeHandle<Source>,
    pub brightness: Brightness,
    pub range: Option<(usize, usize)>,
}

pub struct BrightnessNode<Source, Brightness> {
    source: Source,
    brightness: Brightness,
    range: (usize, usize),
}

impl<Source, Brightness, E> NodeDecl for BrightnessNodeDecl<Source, Brightness>
where
    Source: NodeDecl<Element = E>,
    Brightness: BoundAttrDecl<f64>,
    E: Lerp + Black,
{
    type Element = E;
    type Target = BrightnessNode<Source::Target, Brightness::Target>;

    fn materialize(self, size: usize, builder: &mut NodeBuilder) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            source: builder.node("source", self.source)?,
            brightness: builder.bound_attr("brightness", self.brightness, Bounds::normal())?,
            range: self.range.unwrap_or((0, size - 1)),
        });
    }
}

impl<'a, Source, Brightness> RenderType<'a, Self> for BrightnessNode<Source, Brightness>
where
    Source: Node,
    Brightness: self::Attr<f64>,
    Source::Element: Lerp + Black,
{
    type Render = BrightnessRenderer<<Source as RenderType<'a, Source>>::Render>;
}

impl<Source, Brightness> Node for BrightnessNode<Source, Brightness>
where
    Source: Node,
    Brightness: self::Attr<f64>,
    Source::Element: Lerp + Black,
{
    const KIND: &'static str = "brightness";

    type Element = Source::Element;

    fn update(&mut self, duration: Duration) {
        self.source.update(duration);

        self.brightness.update(duration);
    }

    fn render(&mut self) -> <Self as RenderType<Self>>::Render {
        return BrightnessRenderer {
            source: self.source.render(),
            brightness: self.brightness.get(),
            range: self.range,
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
    pub struct BrightnessConfig {
        pub source: config::Node,
        pub brightness: config::Attr,
        pub range: Option<(usize, usize)>,
    }

    impl NodeModel for BrightnessConfig {
        fn assemble(self, builder: &mut impl NodeBuilder) -> Result<BoxedNodeDecl<color::RGBColor>> {
            return Ok(BoxedNodeDecl::wrap(
                super::BrightnessNodeDecl {
                    source: builder.node("source", self.source)?,
                    brightness: builder.bound_attr("brightness", self.brightness)?,
                    range: self.range,
                },
            ));
        }
    }
}

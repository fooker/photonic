use std::time::Duration;

use anyhow::Result;

use photonic_core::attr::{Attr, BoundAttrDecl, UnboundAttrDecl};
use photonic_core::color::HSVColor;
use photonic_core::node::{Node, NodeDecl, Render, RenderType};
use photonic_core::scene::NodeBuilder;

pub struct LarsonRenderer {
    hue: f64,
    width: f64,
    position: f64,
}

impl Render for LarsonRenderer {
    type Element = HSVColor;

    fn get(&self, index: usize) -> Result<Self::Element> {
        // Calculate value as the linear distance between the pixel and the current
        // position scaled from 0.0 for ±width/2 to 1.0 for center
        let value = f64::max(
            0.0,
            ((self.width / 2.0) - f64::abs((index as f64) - self.position)) / (self.width / 2.0),
        );

        return Ok(HSVColor::new(self.hue, 1.0, value));
    }
}

enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn switched(&self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

pub struct LarsonNodeDecl<Hue, Speed, Width> {
    pub hue: Hue,
    pub speed: Speed,
    pub width: Width,
}

pub struct LarsonNode<Hue, Speed, Width> {
    size: usize,

    hue: Hue,
    speed: Speed,
    width: Width,

    position: f64,
    direction: Direction,
}

impl<Hue, Speed, Width> NodeDecl for LarsonNodeDecl<Hue, Speed, Width>
where
    Hue: BoundAttrDecl<Element = f64>,
    Speed: UnboundAttrDecl<Element = f64>,
    Width: BoundAttrDecl<Element = f64>,
{
    type Element = HSVColor;
    type Target = LarsonNode<Hue::Target, Speed::Target, Width::Target>;

    fn materialize(self, size: usize, builder: &mut NodeBuilder) -> Result<Self::Target> {
        return Ok(Self::Target {
            size,
            hue: builder.bound_attr("hue", self.hue, (0.0, 360.0))?,
            speed: builder.unbound_attr("speed", self.speed)?,
            width: builder.bound_attr("width", self.width, (0.0, size as f64))?,
            position: 0.0,
            direction: Direction::Right,
        });
    }
}

impl<Hue, Speed, Width> RenderType<'_, Self> for LarsonNode<Hue, Speed, Width>
where
    Hue: Attr<Element = f64>,
    Speed: Attr<Element = f64>,
    Width: Attr<Element = f64>,
{
    type Render = LarsonRenderer;
}

impl<Hue, Speed, Width> Node for LarsonNode<Hue, Speed, Width>
where
    Hue: Attr<Element = f64>,
    Speed: Attr<Element = f64>,
    Width: Attr<Element = f64>,
{
    const KIND: &'static str = "larson";

    type Element = HSVColor;

    fn update(&mut self, duration: Duration) -> Result<()> {
        self.speed.update(duration);
        self.width.update(duration);

        let size = self.size as f64;

        match self.direction {
            Direction::Right => {
                self.position += self.speed.get() * duration.as_secs_f64();
                if self.position > size {
                    self.position = size - (self.position - size);
                    self.direction = self.direction.switched();
                }
            }
            Direction::Left => {
                self.position -= self.speed.get() * duration.as_secs_f64();
                if self.position < 0.0 {
                    self.position = -self.position;
                    self.direction = self.direction.switched();
                }
            }
        }

        return Ok(());
    }

    fn render(&self) -> Result<<Self as RenderType<Self>>::Render> {
        return Ok(LarsonRenderer {
            hue: self.hue.get(),
            width: self.width.get(),
            position: self.position,
        });
    }
}

#[cfg(feature = "dyn")]
pub mod model {
    use photonic_core::boxed::{BoxedNodeDecl, Wrap};
    use photonic_core::{color, NodeDecl};
    use photonic_dyn::builder::NodeBuilder;
    use photonic_dyn::config;
    use photonic_dyn::model::NodeModel;

    use anyhow::Result;
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct LarsonConfig {
        pub hue: config::Attr,
        pub speed: config::Attr,
        pub width: config::Attr,
    }

    impl NodeModel for LarsonConfig {
        fn assemble(
            self,
            builder: &mut impl NodeBuilder,
        ) -> Result<BoxedNodeDecl<color::RGBColor>> {
            return Ok(BoxedNodeDecl::wrap(
                super::LarsonNodeDecl {
                    hue: builder.bound_attr("hue", self.hue)?,
                    speed: builder.unbound_attr("speed", self.speed)?,
                    width: builder.bound_attr("width", self.width)?,
                }
                .map(Into::into),
            ));
        }
    }
}

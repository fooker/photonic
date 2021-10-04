use std::time::Duration;

use anyhow::Result;

use photonic_core::node::{Node, NodeDecl, Render, RenderType};
use photonic_core::scene::NodeBuilder;
use photonic_core::{UnboundAttrDecl, Attr};

pub struct SolidRenderer<E>(E);

impl<E> Render for SolidRenderer<E>
    where
        E: Copy,
{
    type Element = E;

    fn get(&self, _index: usize) -> Result<Self::Element> {
        Ok(self.0)
    }
}

pub struct SolidNodeDecl<Solid> {
    pub solid: Solid,
}

impl<Solid> NodeDecl for SolidNodeDecl<Solid>
    where
        Solid: UnboundAttrDecl,
{
    type Element = Solid::Element;
    type Target = SolidNode<Solid::Target>;

    fn materialize(self, _size: usize, builder: &mut NodeBuilder) -> Result<Self::Target> {
        return Ok(Self::Target {
            solid: builder.unbound_attr("solid", self.solid)?,
        });
    }
}

pub struct SolidNode<Solid> {
    solid: Solid,
}

impl<'a, Solid> RenderType<'a, Self> for SolidNode<Solid>
    where
        Solid: Attr,
{
    type Render = SolidRenderer<Solid::Element>;
}

impl<Solid> Node for SolidNode<Solid>
    where
        Solid: Attr,
{
    const KIND: &'static str = "solid";

    type Element = Solid::Element;

    fn update(&mut self, duration: Duration) -> Result<()> {
        self.solid.update(duration);

        return Ok(());
    }

    fn render(&mut self) -> Result<<Self as RenderType<Self>>::Render> {
        return Ok(SolidRenderer(self.solid.get()));
    }
}

#[cfg(feature = "dyn")]
pub mod model {
    use anyhow::Result;
    use serde::Deserialize;

    use photonic_core::boxed::{BoxedNodeDecl, Wrap};
    use photonic_core::color;
    use photonic_dyn::builder::NodeBuilder;
    use photonic_dyn::model::NodeModel;
    use photonic_dyn::config;

    #[derive(Deserialize)]
    pub struct SolidConfig {
        pub solid: config::Attr,
    }

    impl NodeModel for SolidConfig {
        fn assemble(self, builder: &mut impl NodeBuilder) -> Result<BoxedNodeDecl<color::RGBColor>> {
            return Ok(BoxedNodeDecl::wrap(
                super::SolidNodeDecl {
                    solid: builder.unbound_attr("solid", self.solid)?,
                },
            ));
        }
    }
}

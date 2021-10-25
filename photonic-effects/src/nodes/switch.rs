use std::time::Duration;

use anyhow::Result;

use photonic_core::animation::{Animation, Easing, Transition};
use photonic_core::attr::{Attr, BoundAttrDecl, Update};
use photonic_core::math::Lerp;
use photonic_core::node::{Node, NodeDecl, Render, RenderType};
use photonic_core::scene::{NodeBuilder, NodeHandle};

pub enum SwitchRenderer<Source> {
    Blending { source: Source, target: Source, blend: f64 },

    Full(Source),
}

impl<Source> Render for SwitchRenderer<Source>
where
    Source: Render,
    Source::Element: Lerp,
{
    type Element = Source::Element;

    fn get(&self, index: usize) -> Result<Self::Element> {
        match self {
            SwitchRenderer::Blending {
                source,
                target,
                blend,
            } => {
                let source = source.get(index)?;
                let target = target.get(index)?;

                // TODO: Blending modes
                return Ok(Self::Element::lerp(source, target, *blend));
            }

            SwitchRenderer::Full(source) => {
                return source.get(index);
            }
        }
    }
}

pub struct SwitchNodeDecl<Source, Fade>
where
    Source: NodeDecl,
{
    // TODO: Make sources an iterator?
    pub sources: Vec<NodeHandle<Source>>,
    pub fade: Fade,
    pub easing: Option<Easing<f64>>,
}

pub struct SwitchNode<Source, Fade> {
    sources: Vec<Source>,

    fade: Fade,

    source: usize,
    target: usize,
    blend: f64,

    easing: Option<Easing<f64>>,
    transition: Animation<f64>,
}

impl<Source, Fade, E> NodeDecl for SwitchNodeDecl<Source, Fade>
where
    Source: NodeDecl<Element = E>,
    Fade: BoundAttrDecl<Element = i64>,
    E: Lerp,
{
    type Element = E;
    type Target = SwitchNode<Source::Target, Fade::Target>;

    fn materialize(self, _size: usize, builder: &mut NodeBuilder) -> Result<Self::Target> {
        let sources = self
            .sources
            .into_iter()
            .enumerate()
            .map(|(i, source)| builder.node(&format!("source-{}", i), source))
            .collect::<Result<Vec<_>>>()?;

        let fade = builder.bound_attr("fade", self.fade, (0, (sources.len() - 1) as i64))?;

        return Ok(Self::Target {
            sources,
            fade,
            source: 0,
            target: 0,
            blend: 0.0,
            easing: self.easing,
            transition: Animation::idle(),
        });
    }
}

impl<'a, Source, Fade> RenderType<'a, Self> for SwitchNode<Source, Fade>
where
    Source: Node,
    Fade: Attr<Element = i64>,
    Source::Element: Lerp,
{
    type Render = SwitchRenderer<<Source as RenderType<'a, Source>>::Render>;
}

impl<Source, Fade> Node for SwitchNode<Source, Fade>
where
    Source: Node,
    Fade: Attr<Element = i64>,
    Source::Element: Lerp,
{
    const KIND: &'static str = "switch";

    type Element = Source::Element;

    fn update(&mut self, duration: Duration) -> Result<()> {
        for source in &mut self.sources {
            source.update(duration)?;
        }

        if let Update::Changed(fade) = self.fade.update(duration) {
            if let Some(easing) = self.easing {
                self.source = self.target;
                self.target = fade as usize;
                self.blend = 0.0;
                self.transition.start(easing, 0.0, 1.0);
            } else {
                self.source = fade as usize;
                self.target = fade as usize;
            }
        }

        if let Transition::Running(value) = self.transition.update(duration) {
            self.blend = value;
        } else {
            self.source = self.target;
            self.blend = 0.0;
        }

        return Ok(());
    }

    fn render(&self) -> Result<<Self as RenderType<Self>>::Render> {
        if self.source == self.target {
            return Ok(SwitchRenderer::Full(self.sources[self.source].render()?));
        } else {
            let sources = self.sources.as_ptr();

            // We guarantee to have two distinct indices in bounds
            let (source, target) =
                unsafe { (&*sources.add(self.source), &*sources.add(self.target)) };

            return Ok(SwitchRenderer::Blending {
                source: source.render()?,
                target: target.render()?,
                blend: self.blend,
            });
        }
    }
}

#[cfg(feature = "dyn")]
pub mod model {
    use photonic_core::boxed::{BoxedNodeDecl, Wrap};
    use photonic_core::color;
    use photonic_dyn::builder::NodeBuilder;
    use photonic_dyn::config;
    use photonic_dyn::model::NodeModel;

    use crate::attrs::fader::model::EasingModel;
    use anyhow::Result;
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct SwitchConfig {
        pub sources: Vec<config::Node>,
        pub fade: config::Attr,
        pub easing: Option<EasingModel>,
    }

    impl NodeModel for SwitchConfig {
        fn assemble(
            self,
            builder: &mut impl NodeBuilder,
        ) -> Result<BoxedNodeDecl<color::RGBColor>> {
            return Ok(BoxedNodeDecl::wrap(super::SwitchNodeDecl {
                sources: self
                    .sources
                    .into_iter()
                    .map(|source| builder.node("source", source))
                    .collect::<Result<_>>()?,
                fade: builder.bound_attr("fade", self.fade)?,
                easing: self.easing.map(EasingModel::resemble).transpose()?,
            }));
        }
    }
}

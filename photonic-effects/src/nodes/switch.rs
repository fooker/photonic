use std::time::Duration;

use failure::Error;

use photonic_core::animation::*;
use photonic_core::core::*;
use photonic_core::math::Lerp;
use photonic_core::value::*;

pub struct SwitchRenderer<Source> {
    source: Source,
    target: Source,

    blend: f64,
}

impl<Source> Render for SwitchRenderer<Source>
    where Source: Render,
          Source::Element: Lerp {
    type Element = Source::Element;

    fn get(&self, index: usize) -> Self::Element {
        let source = self.source.get(index);
        let target = self.target.get(index);

        // TODO: Blending modes
        return Self::Element::lerp(source,
                                   target,
                                   self.blend);
    }
}

pub struct SwitchNodeDecl<Source, Fade> {
    // TODO: Make sources an iterator?

    pub sources: Vec<NodeRef<Source>>,
    pub fade: Fade,
    pub easing: Option<Easing<f64>>,
}

pub struct SwitchNode<Source, Fade> {
    sources: Vec<NodeHandle<Source>>,

    fade: Fade,

    source: usize,
    target: usize,
    blend: f64,

    easing: Option<Easing<f64>>,
    transition: Animation<f64>,
}

impl<Source, Fade, E> NodeDecl for SwitchNodeDecl<Source, Fade>
    where Source: Node<Element=E>,
          Fade: BoundValueDecl<usize>,
          E: Lerp {
    type Element = E;
    type Target = SwitchNode<Source, Fade::Value>;

    fn materialize(self, _size: usize, mut builder: SceneBuilder) -> Result<Self::Target, Error> {
        let sources = self.sources.into_iter()
            .enumerate()
            .map(|(i, source)| builder.node(&format!("source-{}", i), source))
            .collect::<Result<Vec<_>, Error>>()?;

        let fade = builder.bound_value("fade", self.fade, (0, sources.len() - 1))?;

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

impl<'a, Source, Fade> RenderType<'a> for SwitchNode<Source, Fade>
    where Source: RenderType<'a>,
          Source::Element: Lerp {
    type Element = Source::Element;
    type Render = SwitchRenderer<Source::Render>;
}

impl<Source, Fade, E> Node for SwitchNode<Source, Fade>
    where Source: Node<Element=E>,
          Fade: Value<usize>,
          E: Lerp {
    fn update(&mut self, duration: &Duration) {
        if let Update::Changed(fade) = self.fade.update(duration) {
            if let Some(easing) = self.easing {
                self.source = self.target;
                self.target = fade;
                self.blend = 0.0;
                self.transition.start(easing, 0.0, 1.0);
            } else {
                self.source = fade;
                self.target = fade;
            }
        }

        if let Transition::Running(value) = self.transition.update(duration) {
            self.blend = value;
        } else {
            self.source = self.target;
            self.blend = 0.0;
        }
    }

    fn render<'a>(&'a self, renderer: &'a Renderer) -> <Self as RenderType<'a>>::Render {
        return SwitchRenderer {
            source: renderer.render(&self.sources[self.source]),
            target: renderer.render(&self.sources[self.target]),
            blend: self.blend,
        };
    }
}

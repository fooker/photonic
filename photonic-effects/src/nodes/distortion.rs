use std::time::Duration;

use failure::Error;

use photonic_core::core::*;
use photonic_core::math::Lerp;
use photonic_core::attr::*;

pub struct DistortionRenderer<'a, Source, F>
    where Source: Render,
          F: Fn(&Source::Element, f64) -> Source::Element {
    source: Source,
    distortion: &'a F,
    value: f64,
    time: f64,
}

impl<'a, Source, F> Render for DistortionRenderer<'a, Source, F>
    where Source: Render,
          Source::Element: Lerp,
          F: Fn(&Source::Element, f64) -> Source::Element {
    type Element = Source::Element;

    fn get(&self, index: usize) -> Self::Element {
        let c1 = self.source.get(index);
        let c2 = (self.distortion)(&c1, self.time);
        return Self::Element::lerp(c1, c2, self.value);
    }
}

pub struct DistortionNodeDecl<Source, Value, F>
    where Source: NodeDecl {
    pub source: NodeRef<Source>,
    pub value: Value,
    pub distortion: F,
}

pub struct DistortionNode<Source, Value, F> {
    source: NodeHandle<Source>,
    value: Value,
    distortion: F,

    time: f64,
}

impl<Source, Value, F, E> NodeDecl for DistortionNodeDecl<Source, Value, F>
    where Source: NodeDecl<Element=E>,
          Value: BoundAttrDecl<f64>,
          E: Lerp,
          F: Fn(&E, f64) -> E + 'static {
    type Element = E;
    type Target = DistortionNode<Source::Target, Value::Target, F>;

    fn materialize(self, _size: usize, builder: &mut SceneBuilder) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            source: builder.node("source", self.source)?,
            value: builder.bound_attr("value", self.value, Bounds::normal())?,
            distortion: self.distortion,
            time: 0.0,
        });
    }
}

impl<'a, Source, Value, F> RenderType<'a> for DistortionNode<Source, Value, F>
    where Source: RenderType<'a>,
          Source::Element: Lerp,
          F: Fn(&Source::Element, f64) -> Source::Element + 'a {
    type Element = Source::Element;
    type Render = DistortionRenderer<'a, Source::Render, F>;
}

impl<Source, Value, E, F> Node for DistortionNode<Source, Value, F>
    where Source: Node<Element=E>,
          Value: self::Attr<f64>,
          E: Lerp,
          F: Fn(&E, f64) -> E + 'static {
    const TYPE: &'static str = "distortion";

    fn update(&mut self, duration: &Duration) {
        self.value.update(duration);

        self.time += duration.as_secs_f64();
    }

    fn render<'a>(&'a self, renderer: &'a Renderer) -> <Self as RenderType<'a>>::Render {
        return DistortionRenderer {
            source: renderer.render(&self.source),
            distortion: &self.distortion,
            value: self.value.get(),
            time: self.time,
        };
    }
}

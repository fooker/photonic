use std::time::Duration;

use failure::Error;
use noise::{NoiseFn, Perlin, Seedable};

use photonic_core::core::*;
use photonic_core::math;
use photonic_core::value::*;
use std::marker::PhantomData;
use photonic_core::math::Lerp;

pub struct PlasmaRenderer<'a, E> {
    noise: &'a Perlin,

    range: Range<E>,
    scale: f64,

    time: f64,
}

impl<'a, E> Render for PlasmaRenderer<'a, E>
    where E: Lerp + Copy {
    type Element = E;

    fn get(&self, index: usize) -> Self::Element {
        let i = self.noise.get([index as f64 / self.scale, self.time / self.scale]);
        let i = math::remap(i, (-1.0, 1.0), (0.0, 1.0));

        return E::lerp(self.range.0, self.range.1, i);
    }
}

pub struct PlasmaNodeDecl<Range, Scale, Speed, E> {
    pub range: Range,
    pub scale: Scale,
    pub speed: Speed,

    pub phantom: PhantomData<E>,
}

pub struct PlasmaNode<Range, Scale, Speed, E> {
    perlin: Perlin,

    range: Range,
    scale: Scale,
    speed: Speed,

    time: f64,

    phantom: PhantomData<E>,
}

impl<Range, Scale, Speed, E> NodeDecl for PlasmaNodeDecl<Range, Scale, Speed, E>
    where Range: UnboundValueDecl<self::Range<E>>,
          Scale: UnboundValueDecl<f64>,
          Speed: UnboundValueDecl<f64>,
          E: Lerp + Copy + 'static {
    type Element = E;
    type Target = PlasmaNode<Range::Value, Scale::Value, Speed::Value, E>;

    fn new(self, _size: usize) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            perlin: Perlin::new()
                .set_seed(1),

            range: self.range.new()?,
            scale: self.scale.new()?,
            speed: self.speed.new()?,

            time: 0.0,

            phantom: self.phantom,
        });
    }
}

impl<Range, Scale, Speed, E> Dynamic for PlasmaNode<Range, Scale, Speed, E>
    where Range: Value<self::Range<E>>,
          Scale: Value<f64>,
          Speed: Value<f64>,
          E: Lerp + Copy + 'static {
    fn update(&mut self, duration: &Duration) {
        self.range.update(duration);
        self.scale.update(duration);
        self.speed.update(duration);

        self.time += duration.as_secs_f64() * self.speed.get();
    }
}

impl<'a, Range, Scale, Speed, E> RenderType<'a> for PlasmaNode<Range, Scale, Speed, E>
    where E: Lerp + Copy + 'static {
    type Element = E;
    type Render = PlasmaRenderer<'a, Self::Element>;
}

impl<Range, Scale, Speed, E> Node for PlasmaNode<Range, Scale, Speed, E>
    where Range: Value<self::Range<E>>,
          Scale: Value<f64>,
          Speed: Value<f64>,
          E: Lerp + Copy + 'static {
    fn render<'a>(&'a self, _renderer: &'a Renderer) -> <Self as RenderType<'a>>::Render {
        return PlasmaRenderer {
            noise: &self.perlin,
            range: self.range.get(),
            scale: self.scale.get(),
            time: self.time,
        };
    }
}

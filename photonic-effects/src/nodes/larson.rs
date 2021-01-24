use std::time::Duration;

use failure::Error;

use photonic_core::color::*;
use photonic_core::core::*;
use photonic_core::value::*;

pub struct LarsonRenderer {
    hue: f64,
    width: f64,
    position: f64,
}

impl Render for LarsonRenderer {
    type Element = HSVColor;

    fn get(&self, index: usize) -> Self::Element {
        // Calculate value as the linear distance between the pixel and the current
        // position scaled from 0.0 for ±width/2 to 1.0 for center
        let value = f64::max(0.0, ((self.width / 2.0) - f64::abs((index as f64) - self.position)) / (self.width / 2.0));

        return HSVColor::new(self.hue, 1.0, value);
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
    where Hue: BoundValueDecl<f64>,
          Speed: UnboundValueDecl<f64>,
          Width: BoundValueDecl<f64>
{
    type Element = HSVColor;
    type Target = LarsonNode<Hue::Value, Speed::Value, Width::Value>;

    fn materialize(self, size: usize, mut builder: SceneBuilder) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            size,
            hue: builder.bound_value("hue", self.hue,(0.0, 360.0))?,
            speed: builder.unbound_value("speed", self.speed)?,
            width: builder.bound_value("width", self.width, (0.0, size as f64))?,
            position: 0.0,
            direction: Direction::Right,
        });
    }
}

impl<Hue, Speed, Width> RenderType<'_> for LarsonNode<Hue, Speed, Width> {
    type Element = HSVColor;
    type Render = LarsonRenderer;
}

impl<Hue, Speed, Width> Node for LarsonNode<Hue, Speed, Width>
    where Hue: Value<f64>,
          Speed: Value<f64>,
          Width: Value<f64> {
    fn update(&mut self, duration: &Duration) {
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
    }

    fn render<'a>(&'a self, _renderer: &Renderer) -> <Self as RenderType<'a>>::Render {
        return LarsonRenderer {
            hue: self.hue.get(),
            width: self.width.get(),
            position: self.position,
        };
    }
}

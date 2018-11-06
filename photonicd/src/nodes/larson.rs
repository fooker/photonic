use photonic::values::*;
use photonic::color::*;
use photonic::core::*;
use std::time::Duration;

struct LarsonRenderer {
    size: usize,
    hue: f64,
    width: f64,
    position: f64,
}

impl Renderer for LarsonRenderer {
    fn get(&self, index: usize) -> MainColor {
        // Calculate value as the linear distance between the pixel and the current
        // position scaled from 0.0 for ±width/2 to 1.0 for center
        let value = f64::max(0.0, ((self.width / 2.0) - f64::abs((index as f64) - self.position)) / (self.width / 2.0));

        return HSVColor {
            h: self.hue,
            s: 1.0,
            v: value,
        }.convert();
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

#[derive(Inspection)]
pub struct LarsonNode {
    size: usize,

    #[value()] hue: FloatValue,
    #[value()] speed: FloatValue,
    #[value()] width: FloatValue,

    position: f64,
    direction: Direction,
}

impl LarsonNode {
    const CLASS: &'static str = "larson";

    pub fn new(size: usize,
               hue: FloatValueFactory,
               speed: FloatValueFactory,
               width: FloatValueFactory,
    ) -> Result<Self, String> {
        Ok(Self {
            size,
            hue: hue(FloatValueDecl{name: "hue", min: Some(0.0), max: Some(360.0)})?,
            speed: speed(FloatValueDecl{name: "speed", min: Some(0.0), max: None})?,
            width: width(FloatValueDecl{name: "width", min: Some(0.0), max: Some(size as f64)})?,
            position: 0.0,
            direction: Direction::Right,
        })
    }
}

impl Node for LarsonNode {
    fn class(&self) -> &'static str {
        Self::CLASS
    }
}

impl Source for LarsonNode {
    fn render<'a>(&'a self) -> Box<Renderer + 'a> {
        Box::new(LarsonRenderer {
            size: self.size,
            hue: self.hue.get(),
            width: self.width.get(),
            position: self.position,
        })
    }
}

impl Dynamic for LarsonNode {
    fn update(&mut self, duration: &Duration) {
        self.speed.update(duration);
        self.width.update(duration);

        let size = self.size as f64;

        match self.direction {
            Direction::Right => {
                self.position += self.speed.get() * duration.as_float_secs();
                if self.position > size {
                    self.position = size - (self.position - size);
                    self.direction = self.direction.switched();
                }
            }
            Direction::Left => {
                self.position -= self.speed.get() * duration.as_float_secs();
                if self.position < 0.0 {
                    self.position = -self.position;
                    self.direction = self.direction.switched();
                }
            }
        }
    }
}

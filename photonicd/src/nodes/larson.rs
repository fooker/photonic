use photonic::attributes::*;
use photonic::color::*;
use photonic::core::*;
use photonic::utils::FractionalDuration;
use std::time::Duration;

struct LarsonRenderer {
    size: usize,
    hue: f64,
    width: f64,
    position: f64,
}

impl Renderer for LarsonRenderer {
    fn size(&self) -> usize {
        self.size
    }

    fn get(&self, index: usize) -> MainColor {
        self.get_interpolated(index as f64)
    }

    fn get_interpolated(&self, index: f64) -> MainColor {
        // Calculate value as the linear distance between the pixel and the current
        // position scaled from 0.0 for ±width/2 to 1.0 for center
        let value = f64::max(0.0, ((self.width / 2.0) - f64::abs(index - self.position)) / (self.width / 2.0));

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

    #[attr()] hue: Attribute,
    #[attr()] speed: Attribute,
    #[attr()] width: Attribute,

    position: f64,
    direction: Direction,
}

impl LarsonNode {
    const CLASS: &'static str = "larson";

    pub fn new(size: usize,
               hue: Attribute,
               speed: Attribute,
               width: Attribute,
    ) -> Self {
        Self {
            size,
            hue,
            speed,
            width,
            position: 0.0,
            direction: Direction::Right,
        }
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
                self.position += self.speed.get() * duration.as_fractional_secs();
                if self.position > size {
                    self.position = size - (self.position - size);
                    self.direction = self.direction.switched();
                }
            }
            Direction::Left => {
                self.position -= self.speed.get() * duration.as_fractional_secs();
                if self.position < 0.0 {
                    self.position = -self.position;
                    self.direction = self.direction.switched();
                }
            }
        }
    }
}

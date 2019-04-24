use crate::values::*;
use crate::color::*;
use crate::core::*;
use std::time::Duration;
use failure::Error;

struct LarsonRenderer {
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


pub struct LarsonNodeDecl {
    pub hue: Box<BoundValueDecl<f64>>,
    pub speed: Box<UnboundValueDecl<f64>>,
    pub width: Box<BoundValueDecl<f64>>,
}

pub struct LarsonNode {
    size: usize,

    hue: Box<Value<f64>>,
    speed: Box<Value<f64>>,
    width: Box<Value<f64>>,

    position: f64,
    direction: Direction,
}

impl NodeDecl for LarsonNodeDecl {
    type Node = LarsonNode;

    fn new(self, size: usize) -> Result<Self::Node, Error> {
        return Ok(Self::Node {
            size,
            hue: self.hue.new((0.0, 360.0).into())?,
            speed: self.speed.new()?,
            width: self.width.new((0.0, size as f64).into())?,
            position: 0.0,
            direction: Direction::Right,
        });
    }
}

impl Node for LarsonNode {
    const TYPE: &'static str = "larson";

    fn render<'a>(&'a self) -> Box<Renderer + 'a> {
        Box::new(LarsonRenderer {
            hue: self.hue.get(),
            width: self.width.get(),
            position: self.position,
        })
    }

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
}

//impl Inspection for LarsonNode {
//    fn children(&self) -> Vec<NodeRef> { vec![] }
//
//    fn values(&self) -> Vec<ValueRef> {
//        vec![
////            ValueRef { name: "hue", ptr: self.hue },
////            ValueRef { name: "speed", ptr: self.speed },
////            ValueRef { name: "width", ptr: self.width },
//        ]
//    }
//}

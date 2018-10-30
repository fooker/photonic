use photonic::buffer::*;
use photonic::color::HSVColor;
use photonic::core::*;
use std::time::Duration;

#[derive(Inspection)]
pub struct ColorwheelNode(Buffer<MainColor>);

impl ColorwheelNode {
    const CLASS: &'static str = "colorwheel";

    pub fn new_delta(offset: f64, delta: f64) -> Self {
        let size = (360.0 / delta) as usize;

        return Self(Self::create_buffer(size, offset, delta));
    }

    pub fn new_full(size: usize, offset: f64) -> Self {
        let delta = 360.0 / size as f64;

        return Self(Self::create_buffer(size, offset, delta));
    }

    fn create_buffer(size: usize, offset: f64, delta: f64) -> Buffer<MainColor> {
        return Buffer::from_generator(size,
                                      |i| HSVColor {
                                          h: offset + i as f64 * delta,
                                          s: 1.0,
                                          v: 1.0,
                                      });
    }
}

impl Node for ColorwheelNode {
    fn class(&self) -> &'static str {
        Self::CLASS
    }
}

impl Source for ColorwheelNode {
    fn render<'a>(&'a self) -> Box<Renderer + 'a> {
        Box::new(&self.0)
    }
}

impl Dynamic for ColorwheelNode {
    fn update(&mut self, _: &Duration) {}
}
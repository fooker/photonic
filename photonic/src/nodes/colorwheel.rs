use crate::buffer::*;
use crate::color::HSVColor;
use crate::core::*;

pub struct ColorwheelNode(Buffer<MainColor>);

impl ColorwheelNode {
    pub fn new_delta(offset: f64, delta: f64) -> Result<Self, String> {
        let size = (360.0 / delta) as usize;

        return Ok(Self(Self::create_buffer(size, offset, delta)));
    }

    pub fn new_full(size: usize, offset: f64) -> Result<Self, String> {
        let delta = 360.0 / size as f64;

        return Ok(Self(Self::create_buffer(size, offset, delta)));
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

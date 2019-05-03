use failure::Error;

use photonic_core::buffer::*;
use photonic_core::color::HSVColor;
use photonic_core::core::*;

pub struct ColorwheelNodeDecl {
    pub offset: f64,
}

impl NodeDecl for ColorwheelNodeDecl {
    type Target = Buffer<MainColor>;

    fn new(self, size: usize) -> Result<Self::Target, Error> {
        let delta = 360.0 / size as f64;

        let buffer = Buffer::from_generator(size,
                                            |i| HSVColor {
                                                h: self.offset + i as f64 * delta,
                                                s: 1.0,
                                                v: 1.0,
                                            });

        return Ok(buffer);
    }
}

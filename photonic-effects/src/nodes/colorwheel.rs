use failure::Error;

use photonic_core::buffer::*;
use photonic_core::color::HSVColor;
use photonic_core::core::*;

pub struct ColorwheelNodeDecl {
    pub offset: f64,
    pub scale: f64,
}

impl NodeDecl for ColorwheelNodeDecl {
    type Element = HSVColor;
    type Target = Buffer<Self::Element>;

    fn materialize(self, size: usize, _builder: &mut SceneBuilder) -> Result<Self::Target, Error> {
        let delta = 360.0 / size as f64 * self.scale;

        let buffer = Buffer::from_generator(size,
                                            |i| HSVColor::new(
                                                self.offset + i as f64 * delta,
                                                1.0,
                                                1.0,
                                            ));

        return Ok(buffer);
    }
}

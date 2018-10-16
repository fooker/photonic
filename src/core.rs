use math::{self, Lerp};
use color::RGBColor;
use std::time::Duration;

pub type MainColor = RGBColor;

pub trait Dynamic {
    fn update(&mut self, duration: Duration);
}

pub trait Value: Dynamic {
    fn name(&self) -> &str;

    fn get(&self) -> f64;

    fn get_clamped(&self, range: (f64, f64)) -> f64 {
        math::clamp(self.get(), range)
    }
}

pub trait Renderer {
    fn size(&self) -> usize;
    fn get(&self, index: usize) -> MainColor;

    fn get_interpolated(&self, index: f64) -> MainColor {
        // FIXME: Allow negative indices
        let index = math::wrap(index, self.size());

        let i = (index.trunc() as usize, index.fract());

        let c1 = self.get((i.0 + 0) % self.size());
        let c2 = self.get((i.0 + 1) % self.size());

        return MainColor::lerp(c1, c2, i.1);
    }
}

pub trait Node: Dynamic {
    fn render<'a>(&'a self) -> Box<Renderer + 'a>;
}

pub trait Output {
    fn render(&mut self, renderer: &Renderer);
}

use anyhow::Error;

use photonic_core::color::RGBColor;
use photonic_core::node::Render;
use photonic_core::Output;

pub struct LedStripOutput;

impl Output for LedStripOutput {
    type Element = RGBColor;

    const KIND: &'static str = "Unsupported LED Strip";

    fn render(&mut self, _render: &dyn Render<Element=Self::Element>) {
        panic!("Not supported")
    }
}

pub fn materialize(_desc: super::LedStripOutputDecl, _size: usize) -> Result<LedStripOutput, Error> {
    panic!("Not supported")
}
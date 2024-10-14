use crate::{Buffer, RenderContext};

use anyhow::Result;

pub mod convert;
pub mod map;

pub trait Node {
    type Element: Copy + Default;

    fn update(&mut self, ctx: &RenderContext, out: &mut Buffer<Self::Element>) -> Result<()>;
}

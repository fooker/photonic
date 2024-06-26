use crate::{Buffer, RenderContext};

use anyhow::Result;

pub trait Node: Sized {
    const KIND: &'static str;

    type Element: Copy + Default;

    fn update(&mut self, ctx: &RenderContext, out: &mut Buffer<Self::Element>) -> Result<()>;
}

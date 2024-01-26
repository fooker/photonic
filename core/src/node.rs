use crate::{Buffer, Context};

use anyhow::Result;

pub trait Node: Sized
{
    const KIND: &'static str;

    type Element: Copy + Default;

    fn update(&mut self, ctx: &Context, out: &mut Buffer<Self::Element>) -> Result<()>;
}

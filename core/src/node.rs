use crate::{Buffer, Context};

pub trait Node: Sized
{
    const KIND: &'static str;

    type Element: Copy;

    fn update(&mut self, ctx: &Context, out: &mut Buffer<Self::Element>) -> anyhow::Result<()>;
}

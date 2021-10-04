#![allow(clippy::needless_return)]

use std::io::{stdout, Write};

use anyhow::Error;

use photonic_core::color::palette::LinSrgb;
use photonic_core::color::RGBColor;
use photonic_core::node::{Node, NodeDecl, Render, RenderType};
use photonic_core::output::{Output, OutputDecl};

pub mod registry;

#[cfg_attr(feature = "dyn", derive(serde::Deserialize))]
pub struct ConsoleOutputDecl {
    pub waterfall: bool,
}

pub struct ConsoleOutput {
    size: usize,
    waterfall: bool,
}

impl<Node> OutputDecl<Node> for ConsoleOutputDecl
    where
        Node: self::NodeDecl,
        Node::Element: Into<RGBColor>,
{
    type Target = ConsoleOutput;

    fn materialize(self, size: usize) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            size,
            waterfall: self.waterfall,
        });
    }
}

impl<Node> Output<Node> for ConsoleOutput
    where
        Node: self::Node,
        Node::Element: Into<RGBColor>,
{
    const KIND: &'static str = "console";

    fn render(&mut self, render: <Node as RenderType<'_, Node>>::Render) -> Result<(), Error> {
        // TODO: Maybe with inline replacement?
        let mut buf = Vec::with_capacity(self.size * 20 + 5);

        for i in 0..self.size {
            let rgb: LinSrgb<u8> = render.get(i)?.into().into_format();
            let (r, g, b) = rgb.into_components();
            write!(
                &mut buf,
                "\x1b[48;2;{:03};{:03};{:03}m ",
                r, g, b
            )?;
        }

        write!(&mut buf, "\x1b[0m")?;
        write!(&mut buf, "{}", if self.waterfall { "\n" } else { "\r" })?;

        let out = stdout();
        let mut out = out.lock();
        out.write_all(&buf)?;
        out.flush()?;

        return Ok(());
    }
}

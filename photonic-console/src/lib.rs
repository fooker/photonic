#![allow(clippy::needless_return)]

use std::io::{stdout, Write};

use anyhow::Error;
use serde::Deserialize;

use photonic_core::color::{RGBColor, palette::Component};
use photonic_core::node::Render;
use photonic_core::output::{Output, OutputDecl};

#[derive(Deserialize)]
pub struct ConsoleOutputDecl {
    pub waterfall: bool,
}

pub struct ConsoleOutput {
    size: usize,
    waterfall: bool,
}

impl OutputDecl for ConsoleOutputDecl {
    type Element = RGBColor;
    type Target = ConsoleOutput;

    fn materialize(self, size: usize) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            size,
            waterfall: self.waterfall,
        });
    }
}

impl Output for ConsoleOutput {
    type Element = RGBColor;

    const KIND: &'static str = "console";

    fn render(&mut self, render: &dyn Render<Element = Self::Element>) -> Result<(), Error> {
        // TODO: Maybe with inline replacement?
        let mut buf = Vec::with_capacity(self.size * 20 + 5);

        for i in 0..self.size {
            let rgb: RGBColor = render.get(i);
            let (r, g, b) = rgb.into_components();
            write!(
                &mut buf,
                "\x1b[48;2;{:03};{:03};{:03}m ",
                r.convert::<u8>(),
                g.convert::<u8>(),
                b.convert::<u8>()
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

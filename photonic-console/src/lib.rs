use std::io::{stdout, Write};

use failure::Error;

use palette::Component;
use photonic_core::scene::*;
use photonic_core::color::RGBColor;
use photonic_core::output::{OutputDecl, Output};
use photonic_core::node::Render;

pub struct ConsoleOutputDecl {
    pub whaterfall: bool,
}

pub struct ConsoleOutput {
    size: usize,
    whaterfall: bool,
}

impl OutputDecl for ConsoleOutputDecl {
    type Element = RGBColor;
    type Target = ConsoleOutput;

    fn materialize(self, size: usize) -> Result<Self::Target, Error> {
        return Ok(Self::Target {
            size,
            whaterfall: self.whaterfall,
        });
    }
}

impl Output for ConsoleOutput {
    type Element = RGBColor;

    fn render<E: Into<Self::Element>>(&mut self, render: &dyn Render<Element=E>) {
        // TODO: Maybe with inline replacement?
        let mut buf = Vec::with_capacity(self.size * 20 + 5);

        for i in 0..self.size {
            let rgb: RGBColor = render.get(i).into();
            let (r, g, b) = rgb.into_components();
            write!(&mut buf, "\x1b[48;2;{:03};{:03};{:03}m ", r.convert::<u8>(), g.convert::<u8>(), b.convert::<u8>()).unwrap();
        }

        write!(&mut buf, "\x1b[0m").unwrap();
        if self.whaterfall {
            write!(&mut buf, "\n").unwrap();
        } else {
            write!(&mut buf, "\r").unwrap();
        }

        let out = stdout();
        let mut out = out.lock();
        out.write_all(&buf).unwrap();
        out.flush().unwrap();
    }
}

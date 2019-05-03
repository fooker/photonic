use std::io::{stdout, Write};

use failure::Error;

use photonic_core::core::*;

pub struct ConsoleOutputDecl {
    pub whaterfall: bool,
}

pub struct ConsoleOutput {
    size: usize,
    whaterfall: bool,
}

impl OutputDecl for ConsoleOutputDecl {
    type Output = ConsoleOutput;

    fn new(self, size: usize) -> Result<Self::Output, Error> {
        return Ok(Self::Output {
            size,
            whaterfall: self.whaterfall,
        });
    }
}

impl Output for ConsoleOutput {
    fn render(&mut self, render: &Render) {
        // TODO: Maybe with inline replacement?
        let mut buf = Vec::with_capacity(self.size * 20 + 5);

        for i in 0..self.size {
            let (r, g, b) = render.get(i).int_rgb_tup();
            write!(&mut buf, "\x1b[48;2;{:03};{:03};{:03}m ", r, g, b).unwrap();
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

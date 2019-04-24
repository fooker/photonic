use crate::core::*;
use std::io::{Write,stdout};
use failure::Error;

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
    fn render(&mut self, renderer: &Renderer) {
        // TODO: Maybe with inline replacement?
        let mut out = Vec::with_capacity(self.size * 20 + 5);

        for i in 0..self.size {
            let (r, g, b) = renderer.get(i).int_rgb_tup();
            write!(&mut out, "\x1b[48;2;{:03};{:03};{:03}m ", r, g, b);
        }

        write!(&mut out, "\x1b[0m");
        if self.whaterfall {
            write!(&mut out, "\n");
        } else {
            write!(&mut out, "\r");
        }

        let mut stdout = stdout();
        let mut stdout = stdout.lock();
        stdout.write_all(&out).unwrap();
        stdout.flush().unwrap();

    }
}

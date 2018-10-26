use photonic::core::*;
use std::io::{Write,stdout};


pub struct ConsoleOutput {
    size: usize,
}

impl ConsoleOutput {
    pub fn new(size: usize) -> Self {
        Self {
            size,
        }
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

        writeln!(&mut out, "\x1b[0m");

        let mut stdout = stdout();
        let mut stdout = stdout.lock();
        stdout.write_all(&out).unwrap();
        stdout.flush().unwrap();

    }
}

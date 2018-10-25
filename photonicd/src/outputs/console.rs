use photonic::core::*;
use std::io::{Write,stdout};


pub struct ConsoleOutput {}

impl ConsoleOutput {
    pub fn new() -> Self {
        Self {}
    }
}

impl Output for ConsoleOutput {
    fn render(&mut self, renderer: &Renderer) {
        // TODO: Maybe with inline replacement?
        let mut out = Vec::with_capacity(renderer.size() * 20 + 5);

        for i in 0..renderer.size() {
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

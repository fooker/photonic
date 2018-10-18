use photonic::core::*;


pub struct ConsoleOutput {
}

impl ConsoleOutput {
    pub fn new() -> Self {
        Self {}
    }
}

impl Output for ConsoleOutput {
    fn render(&mut self, renderer: &Renderer) {
        // TODO: Create a buffer and flush it out as one
        // TODO: Maybe with a known size and inline replacement

        for i in 0..renderer.size() {
            let (r, g, b) = renderer.get(i).int_rgb_tup();
            print!("\x1b[48;2;{};{};{}m ", r, g, b);
        }

        println!("\x1b[0m");
    }
}

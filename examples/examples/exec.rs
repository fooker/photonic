#![feature(never_type)]

use failure::Error;

use photonic_console::ConsoleOutputDecl;
use photonic_core::core::Scene;
use photonic_exec::ExecNodeDecl;

const SIZE: usize = 120;
const FPS: usize = 60;

fn main() -> Result<!, Error> {
    let mut scene = Scene::new(SIZE);

    let exec = scene.node("exec", ExecNodeDecl {
        command: "target/debug/examples/exec-sub".to_string(),
    })?;

    let (main, _) = scene.output(exec, ConsoleOutputDecl {
        whaterfall: true
    })?;

    main.run(FPS)?;
}
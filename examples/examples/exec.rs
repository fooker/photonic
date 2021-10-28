use anyhow::Error;

use photonic_console::ConsoleOutputDecl;
use photonic_core::scene::Scene;
use photonic_exec::io::IOExecNodeDecl;

const SIZE: usize = 120;
const FPS: usize = 60;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut scene = Scene::new(SIZE);

    let exec = scene.node("exec", IOExecNodeDecl {
        command: "target/debug/examples/exec-sub".to_string(),
    })?;

    let (main, _) = scene.run(exec, ConsoleOutputDecl {
        waterfall: true,
    })?;

    return main.run(FPS).await;
}

use anyhow::Result;

use photonic_console::ConsoleOutputDecl;
use photonic_core::Scene;
use photonic_wasm::WasmNodeDecl;

const SIZE: usize = 120;
const FPS: usize = 60;

#[tokio::main]
async fn main() -> Result<()> {
    let mut scene = Scene::new(SIZE);

    let exec = scene.node("wasm", WasmNodeDecl {
        path: "examples/example.wat",
    })?;

    let (main, _) = scene.run(exec, ConsoleOutputDecl {
        waterfall: false,
    })?;

    main.run(FPS).await?;

    return Ok(());
}

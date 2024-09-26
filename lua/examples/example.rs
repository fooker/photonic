use anyhow::Result;

use photonic::Scene;
use photonic_lua::node::Lua;
use photonic_output_terminal::Terminal;

#[tokio::main]
async fn main() -> Result<()> {
    let mut scene = Scene::new();

    let lua = scene.node("example", Lua::with_path("lua/examples/example.lua"))?;

    let output = Terminal::new(80).with_waterfall(true);

    let scene = scene.run(lua, output).await?;

    return scene.run(60).await;
}

use anyhow::Result;

use photonic::Scene;
use photonic_lua::Lua;
use photonic_output_terminal::Terminal;

#[tokio::main]
async fn main() -> Result<()> {
    let mut scene = Scene::new();

    let lua = scene.node("example", Lua {
        script: "lua/examples/example.lua".into(),
    })?;

    let output = Terminal::new(80).with_waterfall(true);

    let scene = scene.run(lua, output).await?;

    return scene.run(60).await;
}

use std::cell::RefCell;
use std::path::PathBuf;

use anyhow::{Context, Result};
use ezlua::error::ToLuaResult;
use ezlua::prelude::{Lua as VM, LuaError, LuaFunction, LuaState};
use ezlua::userdata::{UserData, UserdataRegistry};

use photonic::color::palette::rgb::Rgb;
use photonic::{Buffer, Node, NodeBuilder, NodeDecl, RenderContext};

pub struct Lua {
    pub script: PathBuf,
}

pub struct LuaNode {
    lua: VM,
}

impl NodeDecl for Lua {
    type Node = LuaNode;

    async fn materialize(self, _builder: &mut NodeBuilder<'_>) -> Result<Self::Node> {
        let lua = VM::with_open_libs();

        let script = tokio::fs::read(&self.script)
            .await
            .with_context(|| format!("Failed to read script: {}", self.script.display()))?;

        lua.do_string(script, Some("update"))
            .with_context(|| format!("Failed to execute script: {}", self.script.display()))?;

        return Ok(LuaNode {
            lua,
        });
    }
}

impl Node for LuaNode {
    const KIND: &'static str = "lua";
    type Element = Rgb;

    fn update(&mut self, ctx: &RenderContext, out: &mut Buffer<Self::Element>) -> Result<()> {
        let ctx = LuaRenderContext(ctx);
        let out = LuaBuffer(out);

        let update = self.lua.global().get("update")?;
        update.pcall_void((ctx, out))?;

        return Ok(());
    }
}

struct LuaRenderContext<'a, 'ctx>(&'a RenderContext<'ctx>);

impl LuaRenderContext<'_, '_> {
    fn duration(_state: &LuaState, ctx: &Self, _: ()) -> Result<f32, LuaError> {
        return Ok(ctx.0.duration.as_secs_f32());
    }
}

impl UserData for LuaRenderContext<'_, '_> {
    const TYPE_NAME: &'static str = "RenderContext";
    type Trans = RefCell<Self>;

    fn getter(fields: UserdataRegistry<Self>) -> Result<(), LuaError> {
        fields.add_method("duration", Self::duration)?;
        return Ok(());
    }
}

struct LuaBuffer<'a>(&'a mut Buffer<Rgb>);

impl LuaBuffer<'_> {
    fn size(_state: &LuaState, buf: &Self) -> Result<usize, LuaError> {
        return Ok(buf.0.size());
    }

    fn get(_state: &LuaState, buf: &Self, index: usize) -> Result<(f32, f32, f32), LuaError> {
        let value = buf.0.get(index);
        return Ok(value.into_components());
    }

    fn set(_state: &LuaState, buf: &mut Self, (index, r, g, b): (usize, f32, f32, f32)) -> Result<(), LuaError> {
        let value = Rgb::from_components((r, g, b));
        return Ok(buf.0.set(index, value));
    }

    fn update(_state: &LuaState, buf: &mut Self, f: LuaFunction) -> Result<(), LuaError> {
        let f = |index, value: &Rgb<_, f32>| {
            let (r, g, b) = value.into_components();
            let (r, g, b) = f.pcall((index, r, g, b))?;
            let value = Rgb::from_components((r, g, b));
            return anyhow::Ok(value);
        };

        return buf.0.try_update(f).lua_result();
    }
}

impl UserData for LuaBuffer<'_> {
    const TYPE_NAME: &'static str = "Buffer";
    type Trans = RefCell<Self>;

    fn getter(fields: UserdataRegistry<Self>) -> Result<(), LuaError> {
        fields.add_field_get("size", Self::size)?;
        return Ok(());
    }

    fn methods(mt: UserdataRegistry<Self>) -> Result<(), LuaError> {
        mt.add_method("get", Self::get)?;
        mt.add_method_mut("set", Self::set)?;
        mt.add_method_mut("update", Self::update)?;

        return Ok(());
    }
}

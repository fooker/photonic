use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::{Context, Result};
use mlua::{FromLua, IntoLua, TableExt, UserData, UserDataFields, UserDataMethods, Value};

use photonic::color::palette::rgb::Rgb;
use photonic::{Buffer, Node, NodeBuilder, NodeDecl, RenderContext};

// TODO: Reload module source on change using inotify
// TODO: Make this generic over all element types that can be cast to an array
// TODO: Rename `settings` to `globals`, make accessible and allow arbitrary types
// TODO: Allow source nodes, attrs and inputs to be defined and to be accessed

pub struct Lua {
    path: PathBuf,

    settings: HashMap<String, String>,
}

impl Lua {
    pub fn with_path(path: impl Into<PathBuf>) -> Self {
        return Self {
            path: path.into(),
            settings: HashMap::new(),
        };
    }
}

pub struct LuaNode {
    lua: mlua::Lua,

    module: mlua::RegistryKey,
}

impl NodeDecl for Lua {
    const KIND: &'static str = "lua";

    type Node = LuaNode;

    async fn materialize(self, builder: &mut NodeBuilder<'_>) -> Result<Self::Node> {
        let lua = mlua::Lua::new();

        {
            let globals = lua.globals();
            for (k, v) in self.settings {
                globals.set(k, v)?;
            }
        }

        let module = tokio::fs::read(&self.path)
            .await
            .with_context(|| format!("Failed to read script: {}", self.path.display()))?;

        let module: mlua::Table = lua
            .load(module)
            .set_name(builder.name())
            .eval_async()
            .await
            .with_context(|| format!("Failed to evaluate script: {}", self.path.display()))?;

        let module_key = lua.create_registry_value(&module)?;

        drop(module);

        return Ok(LuaNode {
            lua,
            module: module_key,
        });
    }
}

impl Node for LuaNode {
    type Element = Rgb;

    fn update(&mut self, ctx: &RenderContext, out: &mut Buffer<Self::Element>) -> Result<()> {
        let module: mlua::Table = self.lua.registry_value(&self.module)?;

        let ctx = LuaRenderContext(ctx);
        let out = LuaBuffer(out);

        self.lua.scope(|scope| {
            let ctx = scope.create_nonstatic_userdata(ctx)?;
            let out = scope.create_nonstatic_userdata(out)?;

            module.call_method("update", (ctx, out))?;

            return Ok(());
        })?;

        return Ok(());
    }
}

struct LuaRenderContext<'ctx>(&'ctx RenderContext<'ctx>);

impl<'ctx> UserData for LuaRenderContext<'ctx> {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("duration", |_, buf| Ok(buf.0.duration.as_secs_f64()));
    }
}

struct LuaBuffer<'buf>(&'buf mut Buffer<Rgb>);

impl<'buf> UserData for LuaBuffer<'buf> {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("size", |_, buf| Ok(buf.0.size()));
    }

    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method("__index", |_, buf, (i,)| Ok(LuaElement(*buf.0.get(i))));
        methods.add_meta_method_mut("__newindex", |_, buf, (i, v): (usize, LuaElement)| Ok(buf.0.set(i, v.0)));
    }
}

struct LuaElement(Rgb);

impl IntoLua<'_> for LuaElement {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<Value> {
        let array: [f32; 3] = self.0.into();
        let value = array.into_lua(lua)?;

        return Ok(value);
    }
}

impl FromLua<'_> for LuaElement {
    fn from_lua(value: Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        let array = <[f32; 3]>::from_lua(value, lua)?;
        return Ok(Self(array.into()));
    }
}

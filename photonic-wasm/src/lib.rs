use std::path::Path;
use std::time::Duration;

use anyhow::{anyhow, Result};
use wasmtime::{Engine, Instance, Module, Store, TypedFunc};

use photonic_core::{Node, NodeDecl};
use photonic_core::buffer::Buffer;
use photonic_core::color::RGBColor;
use photonic_core::node::RenderType;
use photonic_core::scene::NodeBuilder;

pub struct WasmNodeDecl<P> {
    pub path: P,
}

pub struct WasmNode {
    store: Store<()>,

    update: TypedFunc<f64, ()>,
    render: TypedFunc<u64, (f64, f64, f64)>,

    buffer: Buffer<RGBColor>,
}

impl<P> NodeDecl for WasmNodeDecl<P>
    where P: AsRef<Path> {
    type Element = RGBColor;
    type Target = WasmNode;

    fn materialize(self, size: usize, _builder: &mut NodeBuilder) -> Result<Self::Target> {
        let engine = Engine::default();

        let mut store = Store::new(&engine, ());

        let module = Module::from_file(&engine, self.path)?;

        let instance = Instance::new(&mut store, &module, &[])?;

        let init = instance.get_func(&mut store, "init")
            .ok_or(anyhow!("No function 'init' in wasm module"))?
            .typed::<u64, (), _>(&store)?;

        let update = instance.get_func(&mut store, "update")
            .ok_or(anyhow!("No function 'init' in wasm module"))?
            .typed::<f64, (), _>(&store)?;

        let render = instance.get_func(&mut store, "render")
            .ok_or(anyhow!("No function 'init' in wasm module"))?
            .typed::<u64, (f64, f64, f64), _>(&store)?;

        init.call(&mut store, size as u64)?;

        let buffer = Buffer::new(size);

        return Ok(Self::Target {
            store,
            update,
            render,
            buffer,
        });
    }
}

impl<'a> RenderType<'a, Self> for WasmNode {
    type Render = &'a Buffer<RGBColor>;
}

impl Node for WasmNode {
    type Element = RGBColor;

    const KIND: &'static str = "wasm";

    fn update(&mut self, duration: Duration) -> Result<()> {
        self.update.call(&mut self.store, duration.as_secs_f64())?;
        for i in 0..self.buffer.size() {
            let color = RGBColor::from_components(self.render.call(&mut self.store, i as u64)?);

            self.buffer.set(i, color);
        }

        return Ok(());
    }

    fn render(&mut self) -> Result<<Self as RenderType<Self>>::Render> {
        return Ok(&self.buffer);
    }
}

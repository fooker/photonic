#![allow(clippy::needless_return)]

use std::sync::Arc;
use std::time::Duration;

use num::ToPrimitive;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use photonic_core::boxed::{BoxedNode, BoxedNodeDecl, BoxedOutputDecl};
use photonic_core::element::palette::LinSrgb;
use photonic_core::element::RGBColor;
use photonic_core::input::InputSender;
use photonic_core::node::{Node, NodeDecl, Render, RenderType};
use photonic_core::{Introspection, Loop, Output, OutputDecl, Result};
use photonic_dyn::builder::{Builder, NodeBuilder, OutputBuilder};
use photonic_dyn::registry::{Factory, OutputRegistry};
use photonic_dyn::{config, registry};

struct Registry;

impl OutputRegistry for Registry {
    fn manufacture<Builder: OutputBuilder>(
        _kind: &str,
    ) -> Option<Box<dyn Factory<BoxedOutputDecl<BoxedNodeDecl<RGBColor>>, Builder>>> {
        return None;
    }
}

impl registry::Registry for Registry {
    type Output = Self;
    type Node = photonic_effects::registry::Registry;
    type BoundAttr = photonic_effects::registry::Registry;
    type UnboundAttr = photonic_effects::registry::Registry;
}

#[wasm_bindgen]
pub struct System {
    main: Loop<BoxedNode<RGBColor>, CanvasOutput>,
    registry: Arc<Introspection>,
}

#[wasm_bindgen]
impl System {
    pub fn render(&mut self, duration: u64) -> Result<(), JsValue> {
        return self.main.frame(Duration::from_micros(duration)).map_err(|e| e.to_string().into());
    }

    pub fn send(&mut self, name: &str, value: JsValue) -> Result<(), JsValue> {
        let input = self.registry.inputs.get(name).ok_or("No such input")?;
        match &input.sender {
            InputSender::Trigger(sink) => sink.send(()),

            InputSender::Boolean(sink) => {
                sink.send(value.as_bool().ok_or("value is not bool")?);
            }

            InputSender::Integer(sink) => {
                sink.send(value.as_f64().and_then(|f| f.to_i64()).ok_or("value is not integer")?);
            }

            InputSender::Decimal(sink) => {
                sink.send(value.as_f64().ok_or("value is not decimal")?);
            }
        }

        return Ok(());
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen()]
pub fn render(canvas: HtmlCanvasElement, root: JsValue, size: usize) -> Result<System, JsValue> {
    let context = canvas
        .get_context("2d")?
        .ok_or("Canvas is not capable of 2d rendering")?
        .dyn_into::<CanvasRenderingContext2d>()?;

    let root: config::Node = root.into_serde().map_err(|e| format!("{:#?}", e))?;

    log(&format!("{:#?}", root));

    let mut builder = Builder::<Registry>::new(size);
    let root = builder.node("root", root).map_err(|e| format!("{:#?}", e))?;
    let scene = builder.finish();

    let output = CanvasOutputDecl {
        ctx: context,
    };

    let (main, registry) = scene.run(root, output).map_err(|e| format!("{:#?}", e))?;

    return Ok(System {
        main,
        registry,
    });
}

pub struct CanvasOutputDecl {
    ctx: CanvasRenderingContext2d,
}

impl<Node> OutputDecl<Node> for CanvasOutputDecl
where
    Node: self::NodeDecl,
    Node::Element: Into<RGBColor>,
{
    type Target = CanvasOutput;

    fn materialize(self, size: usize) -> Result<Self::Target>
    where
        Self::Target: Sized,
    {
        return Ok(Self::Target {
            size,
            ctx: self.ctx,
        });
    }
}

pub struct CanvasOutput {
    size: usize,

    ctx: CanvasRenderingContext2d,
}

impl<Node> Output<Node> for CanvasOutput
where
    Node: self::Node,
    Node::Element: Into<RGBColor>,
{
    const KIND: &'static str = "web-canvas";

    fn render(&mut self, render: <Node as RenderType<'_, Node>>::Render) -> Result<()> {
        let width: u32 = self.ctx.canvas().unwrap().width();
        let height: u32 = self.ctx.canvas().unwrap().height();

        let space = width / (self.size as u32 + 1);
        let size = u32::min(space, height);

        self.ctx.clear_rect(0.0, 0.0, width as f64, height as f64);

        for i in 0..self.size {
            let rgb: LinSrgb<u8> = render.get(i)?.into().into_format();

            self.ctx.begin_path();
            self.ctx.set_fill_style(&format!("#{:#x}", rgb).into());
            self.ctx
                .arc(
                    space as f64 * (i + 1) as f64,
                    height as f64 / 2.0,
                    size as f64 / 2.0,
                    0.0,
                    std::f64::consts::PI * 2.0,
                )
                .unwrap();
            self.ctx.fill();
        }

        return Ok(());
    }
}

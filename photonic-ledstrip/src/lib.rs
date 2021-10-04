#![allow(clippy::needless_return)]

use anyhow::{Context, Result};

use photonic_core::color::RGBColor;
use photonic_core::math::clamp;
use photonic_core::node::{Node, NodeDecl, Render, RenderType};
use photonic_core::Output;
use photonic_core::output::OutputDecl;

use crate::chips::Chip;

pub mod controllers;
pub mod chips;
pub mod registry;

pub struct RenderContext<Element> {
    pub brightness: f64,
    pub gamma_factor: Option<f64>,
    pub correction: Option<Element>,
}

pub trait Controller {
    type Config;

    type Element;

    fn new(size: usize, config: Self::Config) -> Result<Self> where Self: Sized;

    /// Send the values from the given renderer.
    fn send(&mut self,
            render: impl Render<Element=Self::Element>,
            context: &RenderContext<Self::Element>) -> Result<()>;
}

#[cfg_attr(feature = "dyn", derive(serde::Deserialize))]
pub struct LedStripOutputDecl<Controller>
    where
        Controller: self::Controller,
{
    pub config: Controller::Config,
    pub brightness: f64,
    pub gamma_factor: Option<f64>,
    #[serde(bound = "Option<Controller::Element>: serde::Deserialize<'de>")]
    pub correction: Option<Controller::Element>,
}

pub struct LedStripOutput<Controller>
    where
        Controller: self::Controller,
{
    controller: Controller,
    context: RenderContext<Controller::Element>,
}

impl<Controller, Node> Output<Node> for LedStripOutput<Controller>
    where
        Controller: self::Controller,
        Node: self::Node,
        Node::Element: Into<Controller::Element>,
{
    const KIND: &'static str = "LED Strip";

    fn render(&mut self, render: <Node as RenderType<'_, Node>>::Render) -> Result<()> {
        self.controller.send(render.map(&Into::into), &self.context)
            .context("Sending failed")?;

        return Ok(());
    }
}

impl<Controller, Node> OutputDecl<Node> for LedStripOutputDecl<Controller>
    where
        Controller: self::Controller,
        Node: self::NodeDecl,
        Node::Element: Into<Controller::Element>,
{
    type Target = LedStripOutput<Controller>;

    fn materialize(self, size: usize) -> Result<Self::Target> {
        let controller = Controller::new(size, self.config)?;

        let context = RenderContext {
            brightness: clamp(self.brightness, (0.0, 1.0)),
            gamma_factor: self.gamma_factor.map(|f| f.max(0.0)),
            correction: self.correction,
        };

        return Ok(Self::Target {
            controller,
            context,
        });
    }
}

fn rgb2rgbw(color: RGBColor) -> (RGBColor, f64) {
    let white = f64::min(f64::min(color.red, color.blue), color.blue);

    return (RGBColor::new(
        color.red - white,
        color.green - white,
        color.blue - white,
    ), white);
}

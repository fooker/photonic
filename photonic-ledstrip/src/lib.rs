#![feature(iter_zip)]
#![allow(clippy::needless_return)]

use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::{Error, Result};

use photonic_core::{Buffer, Output};
use photonic_core::color::RGBColor;
use photonic_core::math::clamp;
use photonic_core::node::{Node, NodeDecl, Render, RenderType};
use photonic_core::output::OutputDecl;

use crate::chips::{Chip, Color};
use crate::controllers::Controller;

pub mod controllers;
pub mod chips;
pub mod buffer;
pub mod registry;

#[cfg_attr(feature = "dyn", derive(serde::Deserialize))]
pub struct LedStripOutputDecl<Controller, Chip>
    where
        Controller: self::Controller,
        Chip: self::Chip,
{
    pub config: Controller::Config,

    pub brightness: f64,
    pub gamma_factor: Option<f64>,

    #[cfg_attr(feature = "dyn", serde(bound = "Option<Chip::Element>: serde::Deserialize<'de>"))]
    pub correction: Option<Chip::Element>,
}

pub struct LedStripOutput<Controller, Chip>
    where
        Controller: self::Controller,
        Chip: self::Chip,
{
    controller: PhantomData<Controller>,
    chip: PhantomData<Chip>,

    running: Arc<AtomicBool>,
    writer: buffer::Writer<Buffer<f64>>,

    brightness: f64,
    gamma_factor: Option<f64>,
    correction: Option<Chip::Element>,
}

impl<Controller, Chip, Node> Output<Node> for LedStripOutput<Controller, Chip>
    where
        Controller: self::Controller,
        Chip: self::Chip,
        Node: self::Node,
        Node::Element: Into<Chip::Element>,
{
    const KIND: &'static str = "LED Strip";

    fn render(&mut self, render: <Node as RenderType<'_, Node>>::Render) -> Result<()> {
        {
            // TODO: Implement this using const generics
            for (i, chunk) in self.writer.chunks_mut(Chip::CHANNELS).enumerate() {
                let color = render.get(i)?;
                Chip::expand(Chip::Element::transform(color.into(),
                                                      self.brightness,
                                                      self.gamma_factor,
                                                      self.correction),
                             chunk);
            }
        }

        self.writer.publish();

        return Ok(());
    }
}

impl<Controller, Chip, Node> OutputDecl<Node> for LedStripOutputDecl<Controller, Chip>
    where
        Controller: self::Controller + Send + Sync + 'static,
        Chip: self::Chip,
        Node: self::NodeDecl,
        Node::Element: Into<Chip::Element>,
{
    type Target = LedStripOutput<Controller, Chip>;

    fn materialize(self, size: usize) -> Result<Self::Target> {
        let channels = size * Chip::CHANNELS;
        let mut controller = Controller::new(channels, self.config)?;

        let (writer, mut reader) = buffer::new(|| Buffer::new(channels));

        let running = Arc::new(AtomicBool::new(true));
        tokio::spawn({
            let running = running.clone();
            async move {
                while running.load(Ordering::SeqCst) {
                    reader.update();

                    let channels = reader.iter()
                        .map(|channel| (channel * 255.0 + 0.5) as u8);

                    controller.send(channels).await?;
                }

                return Result::<(), Error>::Ok(());
            }
        });

        return Ok(Self::Target {
            writer,
            running,
            brightness: clamp(self.brightness, (0.0, 1.0)),
            gamma_factor: self.gamma_factor.map(|f| f.max(0.0)),
            correction: self.correction,
            controller: Default::default(),
            chip: Default::default(),
        });
    }
}

impl<Controller, Chip> Drop for LedStripOutput<Controller, Chip>
    where
        Controller: self::Controller,
        Chip: self::Chip,
{
    fn drop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }
}

pub fn rgb2rgbw(color: RGBColor) -> (RGBColor, f64) {
    let white = f64::min(f64::min(color.red, color.blue), color.blue);

    return (RGBColor::new(
        color.red - white,
        color.green - white,
        color.blue - white,
    ), white);
}

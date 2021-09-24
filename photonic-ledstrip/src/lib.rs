#![allow(clippy::needless_return)]

use anyhow::{Result, Context};
use serde::de::DeserializeOwned;
use serde::Deserialize;

use photonic_core::color::RGBColor;
use photonic_core::math::clamp;
use photonic_core::node::Render;
use photonic_core::Output;
use photonic_core::output::OutputDecl;

pub mod controllers;

pub enum Channels {
    White(Offsets),
    Plain(Offsets),
}

impl Channels {
    pub fn count(&self) -> usize {
        return match self {
            Self::White(_) => 4,
            Self::Plain(_) => 3,
        };
    }

    pub fn expand(&self, color: RGBColor) -> Vec<f64> {
        return match self {
            Self::White(offsets) => {
                // TODO: Do white blending here
                vec![
                    offsets.get0(color),
                    offsets.get1(color),
                    offsets.get2(color),
                    0.0,
                ]
            },

            Self::Plain(offsets) => {
                vec![
                    offsets.get0(color),
                    offsets.get1(color),
                    offsets.get2(color),
                ]
            },
        };
    }
}

pub enum Offsets {
    Rgb,
    Rbg,
    Gbr,
    Grb,
    Brg,
    Bgr,
}

impl Offsets {
    pub fn get0(&self, color: RGBColor) -> f64 {
        return match self {
            Self::Rgb | Self::Rbg => color.red,
            Self::Gbr | Self::Grb => color.green,
            Self::Brg | Self::Bgr => color.blue,
        };
    }

    pub fn get1(&self, color: RGBColor) -> f64 {
        return match self {
            Self::Grb | Self::Brg => color.red,
            Self::Rgb | Self::Bgr => color.green,
            Self::Rbg | Self::Gbr => color.blue,
        };
    }

    pub fn get2(&self, color: RGBColor) -> f64 {
        return match self {
            Self::Gbr | Self::Bgr => color.red,
            Self::Rbg | Self::Brg => color.green,
            Self::Rgb | Self::Grb => color.blue,
        };
    }
}

// Stolen from rs_ws2812x::utils::StripType
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
pub enum Chip {
    Sk6812Rgbw,
    Sk6812Rbgw,
    Sk6812Gbrw,
    Sk6812Grbw,
    Sk6812Brgw,
    Sk6812Bgrw,
    Ws2811Rgb,
    Ws2811Rbg,
    Ws2811Grb,
    Ws2811Gbr,
    Ws2811Brg,
    Ws2811Bgr,
    Ws2812,
    Sk6812,
    Sk6812W,
}

impl Chip {
    pub fn channels(&self) -> Channels {
        return match self {
            Chip::Sk6812Rgbw => Channels::White(Offsets::Rgb),
            Chip::Sk6812Rbgw => Channels::White(Offsets::Rbg),
            Chip::Sk6812Gbrw => Channels::White(Offsets::Gbr),
            Chip::Sk6812Grbw => Channels::White(Offsets::Grb),
            Chip::Sk6812Brgw => Channels::White(Offsets::Brg),
            Chip::Sk6812Bgrw => Channels::White(Offsets::Bgr),

            Chip::Ws2811Rgb => Channels::Plain(Offsets::Rgb),
            Chip::Ws2811Rbg => Channels::Plain(Offsets::Rbg),
            Chip::Ws2811Grb => Channels::Plain(Offsets::Gbr),
            Chip::Ws2811Gbr => Channels::Plain(Offsets::Grb),
            Chip::Ws2811Brg => Channels::Plain(Offsets::Brg),
            Chip::Ws2811Bgr => Channels::Plain(Offsets::Bgr),

            Chip::Ws2812 => Channels::Plain(Offsets::Grb),
            Chip::Sk6812 => Channels::Plain(Offsets::Grb),

            Chip::Sk6812W => Channels::White(Offsets::Grb),
        };
    }
}

pub struct RenderContext {
    pub brightness: f64,
    pub gamma_factor: Option<f64>,
}

impl RenderContext {
    pub fn transform(&self, value: f64) -> u8 {
        // Apply brightness
        let value = value * self.brightness;

        // Apply gamma correction
        return if let Some(gamma_factor) = self.gamma_factor {
            (f64::powf(value, gamma_factor) * 255.00 + 0.5) as u8
        } else {
            (value * 255.0) as u8
        }
    }
}

pub trait Controller {
    type Config: DeserializeOwned;

    fn new(chip: Chip, size: usize, config: Self::Config) -> Result<Self> where Self: Sized;

    /// Update the buffer with the values from the given renderer.
    fn update(&mut self,
              render: &dyn Render<Element=RGBColor>,
              context: &RenderContext) -> Result<()>;

    /// Send out buffer.
    fn send(&mut self) -> Result<()>;
}

#[derive(Deserialize)]
pub struct LedStripOutputDecl<Controller>
    where Controller: self::Controller {
    pub chip: Chip,
    pub config: Controller::Config,
    pub brightness: f64,
    pub gamma_factor: Option<f64>,
}

pub struct LedStripOutput<Controller>
    where Controller: self::Controller {
    controller: Controller,
    context: RenderContext,
}

impl<Controller> Output for LedStripOutput<Controller>
    where Controller: self::Controller {
    type Element = RGBColor;

    const KIND: &'static str = "LED Strip";

    fn render(&mut self, render: &dyn Render<Element=Self::Element>) -> Result<()> {
        self.controller.update(render, &self.context)
            .context("Updating buffer failed")?;

        self.controller.send()
            .context("Sending buffer failed")?;

        return Ok(())
    }
}

impl<Controller> OutputDecl for LedStripOutputDecl<Controller>
    where Controller: self::Controller {
    type Element = RGBColor;
    type Target = LedStripOutput<Controller>;

    fn materialize(self, size: usize) -> Result<Self::Target> {
        let controller = Controller::new(self.chip, size, self.config)?;

        let context = RenderContext {
            brightness: clamp(self.brightness, (0.0, 1.0)),
            gamma_factor: self.gamma_factor.map(|f| f.max(0.0)),
        };

        return Ok(Self::Target {
            controller,
            context,
        });
    }
}

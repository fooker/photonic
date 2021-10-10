#![cfg(feature = "dyn")]

use photonic_core::boxed::{BoxedNodeDecl, BoxedOutputDecl};
use photonic_core::color::RGBColor;
use photonic_dyn::registry::{Factory, OutputRegistry};

use crate::{chips, controllers, LedStripOutputDecl};
use photonic_dyn::builder::OutputBuilder;

pub struct Registry;

impl OutputRegistry for Registry {
    fn manufacture<Builder: OutputBuilder>(kind: &str) -> Option<Box<dyn Factory<BoxedOutputDecl<BoxedNodeDecl<RGBColor>>, Builder>>> {
        return Some(match kind {
            "led-strip:spi:ws2811rgb" => Factory::output::<LedStripOutputDecl<controllers::spi::SPI, chips::Ws2811Rgb>>(),
            "led-strip:spi:ws2811rbg" => Factory::output::<LedStripOutputDecl<controllers::spi::SPI, chips::Ws2811Rbg>>(),
            "led-strip:spi:ws2811grb" => Factory::output::<LedStripOutputDecl<controllers::spi::SPI, chips::Ws2811Grb>>(),
            "led-strip:spi:ws2811gbr" => Factory::output::<LedStripOutputDecl<controllers::spi::SPI, chips::Ws2811Gbr>>(),
            "led-strip:spi:ws2811brg" => Factory::output::<LedStripOutputDecl<controllers::spi::SPI, chips::Ws2811Brg>>(),
            "led-strip:spi:ws2811bgr" => Factory::output::<LedStripOutputDecl<controllers::spi::SPI, chips::Ws2811Bgr>>(),
            "led-strip:spi:sk6812rgbw" => Factory::output::<LedStripOutputDecl<controllers::spi::SPI, chips::Sk6812Rgbw>>(),
            "led-strip:spi:sk6812rbgw" => Factory::output::<LedStripOutputDecl<controllers::spi::SPI, chips::Sk6812Rbgw>>(),
            "led-strip:spi:sk6812gbrw" => Factory::output::<LedStripOutputDecl<controllers::spi::SPI, chips::Sk6812Gbrw>>(),
            "led-strip:spi:sk6812grbw" => Factory::output::<LedStripOutputDecl<controllers::spi::SPI, chips::Sk6812Grbw>>(),
            "led-strip:spi:sk6812brgw" => Factory::output::<LedStripOutputDecl<controllers::spi::SPI, chips::Sk6812Brgw>>(),
            "led-strip:spi:sk6812bgrw" => Factory::output::<LedStripOutputDecl<controllers::spi::SPI, chips::Sk6812Bgrw>>(),
            _ => return None,
        });
    }
}
#![cfg(feature = "dyn")]

use photonic_core::boxed::{BoxedNodeDecl, BoxedOutputDecl};
use photonic_core::element::RGBColor;
use photonic_dyn::builder::OutputBuilder;
use photonic_dyn::registry::{self, Factory, OutputRegistry};

use crate::{chips, controllers, LedStripOutputDecl};

pub struct Registry;

impl OutputRegistry for Registry {
    fn manufacture<Builder: OutputBuilder>(
        kind: &str,
    ) -> Option<Box<dyn Factory<BoxedOutputDecl<BoxedNodeDecl<RGBColor>>, Builder>>> {
        return Some(match kind {
            "led-strip:spi:ws2811rgb" => registry::output::<
                Builder,
                LedStripOutputDecl<controllers::spi::SPI, chips::Ws2811Rgb>,
            >(),
            "led-strip:spi:ws2811rbg" => registry::output::<
                Builder,
                LedStripOutputDecl<controllers::spi::SPI, chips::Ws2811Rbg>,
            >(),
            "led-strip:spi:ws2811grb" => registry::output::<
                Builder,
                LedStripOutputDecl<controllers::spi::SPI, chips::Ws2811Grb>,
            >(),
            "led-strip:spi:ws2811gbr" => registry::output::<
                Builder,
                LedStripOutputDecl<controllers::spi::SPI, chips::Ws2811Gbr>,
            >(),
            "led-strip:spi:ws2811brg" => registry::output::<
                Builder,
                LedStripOutputDecl<controllers::spi::SPI, chips::Ws2811Brg>,
            >(),
            "led-strip:spi:ws2811bgr" => registry::output::<
                Builder,
                LedStripOutputDecl<controllers::spi::SPI, chips::Ws2811Bgr>,
            >(),
            "led-strip:spi:sk6812rgbw" => registry::output::<
                Builder,
                LedStripOutputDecl<controllers::spi::SPI, chips::Sk6812Rgbw>,
            >(),
            "led-strip:spi:sk6812rbgw" => registry::output::<
                Builder,
                LedStripOutputDecl<controllers::spi::SPI, chips::Sk6812Rbgw>,
            >(),
            "led-strip:spi:sk6812gbrw" => registry::output::<
                Builder,
                LedStripOutputDecl<controllers::spi::SPI, chips::Sk6812Gbrw>,
            >(),
            "led-strip:spi:sk6812grbw" => registry::output::<
                Builder,
                LedStripOutputDecl<controllers::spi::SPI, chips::Sk6812Grbw>,
            >(),
            "led-strip:spi:sk6812brgw" => registry::output::<
                Builder,
                LedStripOutputDecl<controllers::spi::SPI, chips::Sk6812Brgw>,
            >(),
            "led-strip:spi:sk6812bgrw" => registry::output::<
                Builder,
                LedStripOutputDecl<controllers::spi::SPI, chips::Sk6812Bgrw>,
            >(),
            _ => return None,
        });
    }
}

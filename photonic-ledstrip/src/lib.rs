#![allow(clippy::needless_return)]

use anyhow::Error;

use photonic_core::color::RGBColor;
use photonic_core::output::OutputDecl;
use serde::Deserialize;

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

cfg_if::cfg_if! {
    if #[cfg(any(
        all(target_arch = "arm", target_os = "linux"),
        all(target_arch = "aarch64", target_os = "linux")
    ))] {
        mod rpi;
        use rpi as r#impl;
    } else {
        mod unsupported;
        use unsupported as r#impl;
    }
}

#[derive(Deserialize)]
pub struct LedStripOutputDecl {
    pub pin: u8,
    pub chip: Chip,
    pub brightness: f64,
}

pub use r#impl::LedStripOutput;

impl OutputDecl for LedStripOutputDecl {
    type Element = RGBColor;
    type Target = LedStripOutput;

    fn materialize(self, size: usize) -> Result<Self::Target, Error> {
        return r#impl::materialize(self, size);
    }
}

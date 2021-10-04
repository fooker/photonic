use std::marker::PhantomData;
use std::path::PathBuf;

use anyhow::Result;
use spidev::{Spidev, SpidevOptions, SpidevTransfer, SpiModeFlags};

use photonic_core::node::Render;

use crate::{Controller, RenderContext};
use crate::chips::Color;

pub struct SPI<Chip: crate::Chip> {
    size: usize,
    spi: Spidev,

    chip: PhantomData<Chip>,
}

impl<Chip: crate::Chip> SPI<Chip> {
    // 8 bits per channel, each bit encoded as 4 bits
    const PATTERNS: [u8; 4] = [
        0b1000_1000, // 0 0
        0b1000_1110, // 0 1
        0b1110_1000, // 1 0
        0b1110_1110, // 1 1
    ];

    const BUS_FREQ: usize = 3_200_000; // SYMBOL_SIZE / 1.25μs

    const RESET_BYTES: usize = 100 * Self::BUS_FREQ / 1000000 / 8;
}

#[cfg_attr(feature = "dyn", derive(serde::Deserialize))]
pub struct Config {
    pub dev: PathBuf,
}

impl<Chip> Controller for SPI<Chip>
    where
        Chip: crate::Chip,
{
    type Config = self::Config;

    type Element = Chip::Element;

    fn new(size: usize, config: Self::Config) -> Result<Self> {
        let mut spi = Spidev::open(config.dev)?;
        spi.configure(&SpidevOptions::new()
            .bits_per_word(8)
            .max_speed_hz(Self::BUS_FREQ as u32)
            .mode(SpiModeFlags::SPI_MODE_0)
            .build())?;

        return Ok(Self {
            size,
            spi,
            chip: Default::default(),
        });
    }

    fn send(&mut self,
              render: impl Render<Element=Self::Element>,
              context: &RenderContext<Self::Element>) -> Result<()> {
        let mut bytes = Vec::new();

        for i in 0..self.size {
            let color = render.get(i)?;
            for channel in Chip::expand(Chip::Element::transform(color, context)) {
                let data = (channel * 255.0 + 0.5) as u8;

                bytes.extend([
                    Self::PATTERNS[((data >> 6) & 0b0000_0011) as usize],
                    Self::PATTERNS[((data >> 4) & 0b0000_0011) as usize],
                    Self::PATTERNS[((data >> 2) & 0b0000_0011) as usize],
                    Self::PATTERNS[((data >> 0) & 0b0000_0011) as usize],
                ]);
            }
        }

        bytes.extend(vec![0; Self::RESET_BYTES]);

        let mut tx = SpidevTransfer::write(&bytes);

        self.spi.transfer(&mut tx)?;

        return Ok(());
    }
}
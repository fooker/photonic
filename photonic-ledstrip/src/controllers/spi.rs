use std::path::PathBuf;

use anyhow::Result;
use serde::Deserialize;
use spidev::{Spidev, SpidevOptions, SpidevTransfer, SpiModeFlags};

use photonic_core::color::RGBColor;
use photonic_core::node::Render;

use crate::{Chip, Controller, RenderContext};

pub struct SPI {
    size: usize,
    chip: Chip,
    spi: Spidev,
    buffer: Box<[u8]>,
}

impl SPI {
    const PATTERN_SIZE: usize = 4;

    // 8 bits per channel, each bit encoded as 4 bits
    const PATTERNS: [u8; SPI::PATTERN_SIZE] = [
        0b1000_1000, // 0 0
        0b1000_1110, // 0 1
        0b1110_1000, // 1 0
        0b1110_1110, // 1 1
    ];

    const BUS_FREQ: usize = 3_200_000; // SYMBOL_SIZE / 1.25μs

    const RESET_BYTES: usize = 100 * SPI::BUS_FREQ / 1000000 / 8;
}

#[derive(Deserialize)]
pub struct Config {
    pub dev: PathBuf,
}

impl Controller for SPI {
    type Config = self::Config;

    fn new(chip: Chip, size: usize, config: Self::Config) -> Result<Self> {
        let mut spi = Spidev::open(config.dev)?;
        spi.configure(&SpidevOptions::new()
            .bits_per_word(8)
            .max_speed_hz(SPI::BUS_FREQ as u32)
            .mode(SpiModeFlags::SPI_MODE_0)
            .build())?;

        let buffer = vec![0; size * chip.channels().count() * SPI::PATTERN_SIZE + SPI::RESET_BYTES].into_boxed_slice();

        return Ok(Self {
            size,
            chip,
            spi,
            buffer,
        });
    }

    fn update(&mut self,
              render: &dyn Render<Element=RGBColor>,
              context: &RenderContext) -> Result<()> {
        let channels = self.chip.channels();

        let bytes = (0..self.size)
            .map(|i| render.get(i))
            .flat_map(|color| channels.expand(context.transform(color)))
            .map(|channel| (channel * 255.0 + 0.5) as u8)
            .enumerate();

        for (i, b) in bytes {
            self.buffer[i * SPI::PATTERN_SIZE + 0] = Self::PATTERNS[((b >> 6) & 0b0000_0011) as usize];
            self.buffer[i * SPI::PATTERN_SIZE + 1] = Self::PATTERNS[((b >> 4) & 0b0000_0011) as usize];
            self.buffer[i * SPI::PATTERN_SIZE + 2] = Self::PATTERNS[((b >> 2) & 0b0000_0011) as usize];
            self.buffer[i * SPI::PATTERN_SIZE + 3] = Self::PATTERNS[((b >> 6) & 0b0000_0011) as usize];
        }

        return Ok(());
    }

    fn send(&mut self) -> Result<()> {
        let mut tx = SpidevTransfer::write(&self.buffer);

        self.spi.transfer(&mut tx)?;
        return Ok(());
    }
}
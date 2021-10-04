use std::marker::PhantomData;
use std::path::PathBuf;

use anyhow::Result;
use spidev::{Spidev, SpidevOptions, SpidevTransfer, SpiModeFlags};

use photonic_core::node::Render;

use crate::{Controller, RenderContext};

pub struct SPI<Chip: crate::Chip> {
    size: usize,
    spi: Spidev,
    buffer: Vec<u8>,

    chip: PhantomData<Chip>,
}

impl<Chip: crate::Chip> SPI<Chip> {
    const PATTERN_SIZE: usize = 4;

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

        let buffer = Vec::new(); // vec![0; size * Chip::CHANNELS * Self::PATTERN_SIZE + Self::RESET_BYTES].into_boxed_slice();

        return Ok(Self {
            size,
            spi,
            buffer,
            chip: Default::default(),
        });
    }

    fn send(&mut self,
              render: impl Render<Element=Self::Element>,
              context: &RenderContext<Self::Element>) -> Result<()> {
        let bytes = (0..self.size)
            .map(|i| render.get(i))
            // .flat_map(|color| Chip::expand(context.transform(color))) // TODO: Add transform
            .flat_map(|color| Chip::expand(color))
            .map(|channel| (channel * 255.0 + 0.5) as u8)
            .flat_map(|b| [
                Self::PATTERNS[((b >> 6) & 0b0000_0011) as usize],
                Self::PATTERNS[((b >> 4) & 0b0000_0011) as usize],
                Self::PATTERNS[((b >> 2) & 0b0000_0011) as usize],
                Self::PATTERNS[((b >> 6) & 0b0000_0011) as usize],
            ])
            .collect::<Vec<_>>();

        let mut tx = SpidevTransfer::write(&bytes);

        self.spi.transfer(&mut tx)?;

        return Ok(());
    }
}
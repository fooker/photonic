//! Implementation of the WS281x protocol using linux `spidev` device driver.
//!
//! The driver generates the WS281x signal on the data pin of a SPI master controller by generating
//! a pattern that ensembles the required signal.
//!
//! # Async
//! The `spidev` kernel driver currently does not allow asynchronous operations on the SPI
//! interface. Therefore this controller makes access to the `spidev` file-handle nonblocking but
//! sole relies on a timer to ensure all data has been flushed out before sending new one.

use std::path::PathBuf;

use anyhow::Result;
use spidev::{Spidev, SpidevOptions, SpidevTransfer, SpiModeFlags};

use super::Controller;
use std::os::unix::prelude::AsRawFd;
use nix::fcntl;
use std::time::Duration;

// struct AsyncSpidev {
//     inner: AsyncFd<Spidev>,
// }
//
// impl AsyncSpidev {
//     pub fn new(spidev: Spidev) -> std::io::Result<Self> {
//         return Ok(Self {
//             inner: AsyncFd::new(spidev)?,
//         });
//     }
//
//     pub async fn write(&self, bytes: &[u8]) -> std::io::Result<()> {
//         let mut tx = SpidevTransfer::write(&bytes);
//
//         loop {
//             let mut guard = self.inner.writable().await?;
//             match guard.try_io(|inner| inner.get_ref().transfer(&mut tx)) {
//                 Ok(result) => return result,
//                 Err(_would_block) => continue,
//             }
//         }
//     }
// }

pub struct SPI {
    spi: Spidev,

    buffer: Box<[u8]>,
    send_time: Duration,
}

impl SPI {
    /// The overall duration of a single WS281x bit.
    const BIT_DURATION: Duration = Duration::from_nanos(1250); // 1.25μs

    /// Number of SPI bits for each WS281x bit.
    const PATTERN_BITS: usize = 4;

    /// Encoded WS281x data.
    /// Each pattern is selected depending on 2 bits to send out and expands to 4 SPI bits for each
    /// WS281x bit.
    const PATTERNS: [u8; Self::PATTERN_BITS] = [
        0b1000_1000, // 0 0
        0b1000_1110, // 0 1
        0b1110_1000, // 1 0
        0b1110_1110, // 1 1
    ];

    /// Expected frequency of the SPI bus.
    /// Each `PATTERN_BITS` bits must be send out in `BIT_DURATION`.
    const BUS_FREQ: u32 = Self::PATTERN_BITS as u32 * 1_000_000_000 / Self::BIT_DURATION.as_nanos() as u32;

    /// Time the data line is held low to trigger a reset of the WS281x bus.
    const RESET_DURATION: Duration = Duration::from_micros(100);
}

#[cfg_attr(feature = "dyn", derive(serde::Deserialize))]
pub struct Config {
    pub dev: PathBuf,
}

#[async_trait::async_trait]
impl Controller for SPI
{
    type Config = self::Config;

    fn new(channels: usize, config: Self::Config) -> Result<Self> {
        let mut spi = Spidev::open(config.dev)?;
        spi.configure(&SpidevOptions::new()
            .bits_per_word(8)
            .max_speed_hz(Self::BUS_FREQ as u32)
            .mode(SpiModeFlags::SPI_MODE_0)
            .build())?;

        // Make the underlying file non blocking
        {
            let flags = fcntl::OFlag::from_bits_truncate(fcntl::fcntl(spi.as_raw_fd(), fcntl::F_GETFL)?);
            fcntl::fcntl(spi.as_raw_fd(), fcntl::F_SETFL(flags | fcntl::OFlag::O_NONBLOCK))?;
        }

        // Initialize buffer for SPI transmission
        let buffer = vec![0u8; channels * Self::PATTERN_BITS].into_boxed_slice();

        // Time to send data including reset
        let send_time = Self::BIT_DURATION * 8 * channels as u32 + Self::RESET_DURATION;

        return Ok(Self {
            spi,
            buffer,
            send_time,
        });
    }

    async fn send(&mut self, channels: impl Iterator<Item=u8> + Send + 'async_trait) -> Result<()> {
        let data = channels.flat_map(|channel| [
            Self::PATTERNS[((channel >> 6) & 0b0000_0011) as usize],
            Self::PATTERNS[((channel >> 4) & 0b0000_0011) as usize],
            Self::PATTERNS[((channel >> 2) & 0b0000_0011) as usize],
            Self::PATTERNS[((channel >> 0) & 0b0000_0011) as usize],
        ]);

        for (buffer, data) in std::iter::zip(self.buffer.iter_mut(), data) {
            *buffer = data;
        }

        // Send out the buffer non-blocking
        let mut tx = SpidevTransfer::write(&self.buffer);
        self.spi.transfer(&mut tx)?;

        // Asynchronously wait for the transmission to complete
        tokio::time::sleep(self.send_time).await;

        return Ok(());
    }
}
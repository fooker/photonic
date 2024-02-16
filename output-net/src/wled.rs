use byteorder::{BigEndian, WriteBytesExt};

use anyhow::Result;
use palette::rgb::Rgb;

use photonic::{BufferReader, Output, OutputDecl, WhiteMode};

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    DRGB,
    DRGBW { mode: WhiteMode },
    DNRGB { offset: u16 },
}

impl Default for Mode {
    fn default() -> Self {
        return Self::DRGB;
    }
}

impl Mode {
    pub fn element_size(&self) -> usize {
        return match self {
            Self::DRGB => 3,
            Self::DRGBW {
                ..
            } => 4,
            Self::DNRGB {
                ..
            } => 5,
        };
    }
}

#[derive(Default)]
pub struct WledSender {
    pub mode: Mode,
}

pub struct WledSenderOutput {
    mode: Mode,
}

impl OutputDecl for WledSender {
    type Output = WledSenderOutput;

    async fn materialize(self, _size: usize) -> Result<Self::Output>
    where Self::Output: Sized {
        return Ok(Self::Output {
            mode: self.mode,
        });
    }
}

impl Output for WledSenderOutput {
    const KIND: &'static str = "wled";

    type Element = Rgb;

    async fn render(&mut self, out: impl BufferReader<Element = Self::Element>) -> anyhow::Result<()> {
        let mut buffer = Vec::<u8>::with_capacity(2 + out.size() * self.mode.element_size()); // TODO: Allocate only once and re-use
        buffer.write_u8(match self.mode {
            Mode::DRGB => 2,
            Mode::DRGBW {
                ..
            } => 3,
            Mode::DNRGB {
                ..
            } => 4,
        })?;
        buffer.write_u8(255)?; // Never automatically switch back to default mode

        if let Mode::DNRGB {
            offset,
        } = self.mode
        {
            buffer.write_u16::<BigEndian>(offset)?;
        }

        match self.mode {
            Mode::DRGB
            | Mode::DNRGB {
                ..
            } => {
                for rgb in out.iter() {
                    let (r, g, b) = rgb.into_format::<u8>().into_components();

                    buffer.write_u8(r)?;
                    buffer.write_u8(g)?;
                    buffer.write_u8(b)?;
                }
            }
            Mode::DRGBW {
                mode,
            } => {
                for rgb in out.iter() {
                    let rgbw = mode.apply(rgb);
                    let (r, g, b, w) = rgbw.into_format::<u8>().into_components();

                    buffer.write_u8(r)?;
                    buffer.write_u8(g)?;
                    buffer.write_u8(b)?;
                    buffer.write_u8(w)?;
                }
            }
        }

        return Ok(());
    }
}

use std::net::SocketAddr;

use anyhow::{bail, Result};
use palette::rgb::Rgb;
use palette::encoding::Srgb;

use photonic::{BufferReader, Output, OutputDecl, WhiteMode, Rgbw, math::clamp};

pub enum Channel {
    Red,
    Green,
    Blue,
    White,
    Fixed(u8),
    Calibrated(Box<Channel>, f32),
}

impl Channel {
    pub fn calibrated(self, scale: f32) -> Self {
        return Self::Calibrated(Box::new(self), scale);
    }

    pub fn extract(&self, pixel: Rgbw<Srgb, u8>) -> u8 {
        return match self {
            Channel::Red => pixel.red,
            Channel::Green => pixel.green,
            Channel::Blue => pixel.blue,
            Channel::White => pixel.white,
            Channel::Fixed(value) => *value,
            Channel::Calibrated(channel, scale) => clamp(channel.extract(pixel) as f32 * *scale, (u8::MIN as f32, u8::MAX as f32)) as u8,
        };
    }
}

pub struct Fixture {
    pub dmx_address: usize,
    pub dmx_channels: Vec<Channel>,

    pub white_mode: WhiteMode,
}

pub struct NetDmxSender {
    pub address: SocketAddr,

    pub fixtures: Vec<Fixture>,
}

impl NetDmxSender {
    pub fn with_address(address: SocketAddr) -> Self {
        return Self {
            address,
            fixtures: Vec::new(),
        };
    }

    pub fn add_fixture(mut self, fixture: Fixture) -> Self {
        self.fixtures.push(fixture);
        return self;
    }

    pub fn add_fixtures(mut self, n: usize, fixture: impl Fn(usize) -> Fixture) -> Self {
        for n in 0..n {
            self.fixtures.push(fixture(n));
        }

        return self;
    }
}

pub struct NetDmxSenderOutput {
    socket: tokio::net::UdpSocket,
    address: SocketAddr,

    fixtures: Vec<Fixture>,

    buffer: [u8; 512],
}

impl OutputDecl for NetDmxSender {
    type Output = NetDmxSenderOutput;

    async fn materialize(self) -> Result<Self::Output>
    where Self::Output: Sized {
        let socket = tokio::net::UdpSocket::bind("127.0.0.0:0").await?;

        for fixture in &self.fixtures {
            // Check DMX addresses are in bounds
            let addresses = (fixture.dmx_address, fixture.dmx_address + fixture.dmx_channels.len());
            if addresses.0 < 1 || addresses.1 > 512 {
                bail!("Invalid fixture address {}:{}", addresses.0, addresses.1);
            }
        }

        return Ok(Self::Output {
            socket,
            address: self.address,
            fixtures: self.fixtures,
            buffer: [0u8; 512],
        });
    }
}

impl Output for NetDmxSenderOutput {
    const KIND: &'static str = "netdmx";

    type Element = Rgb;

    async fn render(&mut self, out: impl BufferReader<Element = Self::Element>) -> Result<()> {
        for (i, fixture) in self.fixtures.iter().enumerate() {
            let pixel = out.get(i);
            let pixel = fixture.white_mode.apply(pixel);
            let pixel = pixel.into_format::<u8>();

            for (i, channel) in fixture.dmx_channels.iter().enumerate() {
                self.buffer[fixture.dmx_address - 1 + i] = channel.extract(pixel);
            }
        }

        self.socket.send_to(&self.buffer, &self.address).await?;

        return Ok(());
    }

    fn size(&self) -> usize {
        return self.fixtures.len();
    }
}

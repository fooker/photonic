use std::net::SocketAddr;

use anyhow::{bail, Result};
use palette::rgb::Rgb;

use photonic::{BufferReader, Output, OutputDecl, WhiteMode};

pub enum Channel {
    Red,
    Green,
    Blue,
    White,
    Fixed(u8),
}

pub struct Fixture {
    pub pixel: usize,

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

    async fn materialize(self, size: usize) -> Result<Self::Output>
    where Self::Output: Sized {
        let socket = tokio::net::UdpSocket::bind("127.0.0.0:0").await?;

        for fixture in &self.fixtures {
            // Check DMX addresses are in bounds
            let addresses = (fixture.dmx_address, fixture.dmx_address + fixture.dmx_channels.len());
            if addresses.0 < 1 || addresses.1 > 512 {
                bail!("Invalid fixture address {}:{}", addresses.0, addresses.1);
            }

            // Check fixtures pixel is in bounds
            if fixture.pixel >= size {
                bail!("Fixture pixel out of bounds: {} >= {}", fixture.pixel, size);
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
        for fixture in self.fixtures.iter() {
            let pixel = out.get(fixture.pixel);
            let pixel = fixture.white_mode.apply(pixel);
            let pixel = pixel.into_format::<u8>();

            for (i, config) in fixture.dmx_channels.iter().enumerate() {
                self.buffer[fixture.dmx_address - 1 + i] = match config {
                    Channel::Red => pixel.red,
                    Channel::Green => pixel.green,
                    Channel::Blue => pixel.blue,
                    Channel::White => pixel.white,
                    Channel::Fixed(value) => *value,
                };
            }
        }

        self.socket.send_to(&self.buffer, &self.address).await?;

        return Ok(());
    }
}

use std::net::SocketAddr;

use anyhow::Result;
use palette::rgb::Rgb;

use photonic::{BufferReader, Output, OutputDecl, WhiteMode};

pub enum Channels {
    RGB,
    RGBW(WhiteMode),
}

pub struct Fixture {
    pub pixel: usize,

    pub dmx_address: usize,
    pub dmx_channels: Channels,
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

impl OutputDecl for NetDmxSender
{
    type Output = NetDmxSenderOutput;

    async fn materialize(self, size: usize) -> Result<Self::Output>
        where Self::Output: Sized,
    {
        let socket = tokio::net::UdpSocket::bind("127.0.0.0:0").await?;

        // TODO: Check fixtures pixel is in bounds
        // TODO: Check fixtures dmx address is in bounds

        return Ok(Self::Output {
            socket,
            address: self.address,
            fixtures: self.fixtures,
            buffer: [0u8; 512],
        });
    }
}

impl Output for NetDmxSenderOutput
{
    const KIND: &'static str = "netdmx";

    type Element = Rgb;

    async fn render(&mut self, out: impl BufferReader<Element=Self::Element>) -> Result<()> {
        for fixture in self.fixtures.iter() {
            let pixel = out.get(fixture.pixel);

            match fixture.dmx_channels {
                Channels::RGB => {
                    let pixel = pixel.into_format::<u8>();
                    self.buffer[fixture.dmx_address - 1 + 0] = pixel.red;
                    self.buffer[fixture.dmx_address - 1 + 1] = pixel.green;
                    self.buffer[fixture.dmx_address - 1 + 2] = pixel.blue;
                }

                Channels::RGBW(white_mode) => {
                    let pixel = white_mode.apply(pixel);
                    let pixel = pixel.into_format::<u8>();
                    self.buffer[fixture.dmx_address - 1 + 0] = pixel.red;
                    self.buffer[fixture.dmx_address - 1 + 1] = pixel.green;
                    self.buffer[fixture.dmx_address - 1 + 2] = pixel.blue;
                    self.buffer[fixture.dmx_address - 1 + 3] = pixel.white;
                }
            }
        }

        self.socket.send_to(&self.buffer, &self.address).await?;

        return Ok(());
    }
}

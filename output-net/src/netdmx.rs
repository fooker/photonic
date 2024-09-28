use std::net::SocketAddr;

use anyhow::{bail, Result};

use photonic::{BufferReader, Output, OutputDecl};

pub use self::channel::{Channel, Channels};
pub use self::fixture::Fixture;

mod channel;
mod fixture;

pub struct NetDmxSender<E> {
    pub address: SocketAddr,

    pub fixtures: Vec<Fixture<E>>,
}

impl<E> NetDmxSender<E> {
    pub fn with_address(address: SocketAddr) -> Self {
        return Self {
            address,
            fixtures: Vec::new(),
        };
    }

    pub fn add_fixture(mut self, fixture: Fixture<E>) -> Self {
        self.fixtures.push(fixture);
        return self;
    }

    pub fn add_fixtures(mut self, n: usize, fixture: impl Fn(usize) -> Fixture<E>) -> Self {
        for n in 0..n {
            self.fixtures.push(fixture(n));
        }

        return self;
    }
}

pub struct NetDmxSenderOutput<E> {
    socket: tokio::net::UdpSocket,
    address: SocketAddr,

    fixtures: Vec<Fixture<E>>,

    buffer: [u8; 512],
}

impl<E> OutputDecl for NetDmxSender<E> {
    const KIND: &'static str = "dmx";

    type Output = NetDmxSenderOutput<E>;

    async fn materialize(self) -> Result<Self::Output>
    where Self::Output: Sized {
        let socket = tokio::net::UdpSocket::bind("127.0.0.0:0").await?;

        for fixture in &self.fixtures {
            // Check DMX addresses are in bounds
            let addresses = fixture.addresses();
            if addresses.start < 1 || addresses.end > 512 {
                bail!("Invalid fixture address range {}..{}", addresses.start, addresses.end);
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

impl<E> Output for NetDmxSenderOutput<E> {
    const KIND: &'static str = "netdmx";

    type Element = E;

    async fn render(&mut self, out: impl BufferReader<Element = Self::Element>) -> Result<()> {
        for (i, fixture) in self.fixtures.iter().enumerate() {
            let pixel = out.get(i);

            for (address, channel) in fixture.channels() {
                self.buffer[address - 1] = channel.extract(&pixel);
            }
        }

        self.socket.send_to(&self.buffer, &self.address).await?;

        return Ok(());
    }

    fn size(&self) -> usize {
        return self.fixtures.len();
    }
}

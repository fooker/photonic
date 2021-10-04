use std::marker::PhantomData;
use std::net::{ToSocketAddrs, UdpSocket};
use std::time::Duration;

use anyhow::Result;

use photonic_core::buffer::Buffer;
use photonic_core::color::{Black, RGBColor};
use photonic_core::node::{Node, NodeDecl, RenderType};
use photonic_core::scene::*;
use photonic_core::color::palette::LinSrgb;

pub trait Format {
    type Element: Copy;
    const ELEMENT_SIZE: usize;

    fn load(b: &[u8]) -> Self::Element;
}

impl Format for RGBColor {
    type Element = RGBColor;
    const ELEMENT_SIZE: usize = 3;

    fn load(b: &[u8]) -> Self::Element {
        return LinSrgb::<u8>::from_components((b[0], b[1], b[2])).into_format();
    }
}

pub struct ReceiverNode<F>
where
    F: Format,
{
    socket: UdpSocket,
    buffer: Box<[u8]>,

    output: Buffer<F::Element>,

    format: PhantomData<F>,
}

pub struct ReceiverNodeDecl<A, F>
where
    A: ToSocketAddrs,
    F: Format,
{
    address: A,
    format: PhantomData<F>,
}

impl<A, F> ReceiverNodeDecl<A, F>
where
    A: ToSocketAddrs,
    F: Format,
    F::Element: Black,
{
    pub fn bind(address: A) -> Self {
        return Self {
            address,
            format: PhantomData,
        };
    }
}

impl<A, F> NodeDecl for ReceiverNodeDecl<A, F>
where
    A: ToSocketAddrs,
    F: Format + 'static,
    F::Element: Black + 'static,
{
    type Element = F::Element;
    type Target = ReceiverNode<F>;

    fn materialize(self, size: usize, _builder: &mut NodeBuilder) -> Result<Self::Target> {
        let socket = UdpSocket::bind(self.address)?;
        socket.set_nonblocking(true).unwrap();

        return Ok(ReceiverNode {
            socket,
            buffer: vec![0; size * F::ELEMENT_SIZE].into_boxed_slice(),
            output: Buffer::black(size),
            format: PhantomData,
        });
    }
}

impl<'a, F> RenderType<'a, Self> for ReceiverNode<F>
where
    F: Format + 'static,
{
    type Render = &'a Buffer<F::Element>;
}

impl<F> Node for ReceiverNode<F>
where
    F: Format + 'static,
{
    const KIND: &'static str = "udp";

    type Element = F::Element;

    fn update(&mut self, _duration: Duration) -> Result<()> {
        // Read all packets available without blocking but only use last one
        loop {
            match self.socket.recv(&mut self.buffer) {
                Ok(_) => continue,
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                Err(e) => panic!("io error: {}", e),
            }
        }

        Iterator::zip(self.output.iter_mut(), self.buffer.chunks(F::ELEMENT_SIZE))
            .for_each(|(o, b)| *o = F::load(b));

        return Ok(());
    }

    fn render(&mut self) -> Result<<Self as RenderType<Self>>::Render> {
        return Ok(&self.output);
    }
}

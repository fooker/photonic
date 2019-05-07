use std::marker::PhantomData;
use std::net::{ToSocketAddrs, UdpSocket};
use std::time::Duration;

use failure::Error;

use photonic_core::buffer::Buffer;
use photonic_core::color::{Black, Color, RGBColor};
use photonic_core::core::{Node, NodeDecl, Render, Renderer};

pub trait Format {
    type Color: Copy + Color;
    const ELEMENT_SIZE: usize;

    fn load(b: &[u8]) -> Self::Color;
}

impl Format for RGBColor {
    type Color = RGBColor;
    const ELEMENT_SIZE: usize = 3;

    fn load(b: &[u8]) -> Self::Color {
        return RGBColor::from((b[0], b[1], b[2]));
    }
}

pub struct ReceiverNode<F>
    where F: Format {
    socket: UdpSocket,
    buffer: Box<[u8]>,

    output: Buffer<F::Color>,

    format: PhantomData<F>,
}

pub struct ReceiverNodeDecl<A, F>
    where A: ToSocketAddrs,
          F: Format {
    address: A,
    format: PhantomData<F>,
}

impl<A, F> ReceiverNodeDecl<A, F>
    where A: ToSocketAddrs,
          F: Format,
          F::Color: Black {
    pub fn bind(address: A) -> Self {
        return Self {
            address: address,
            format: PhantomData,
        };
    }
}

impl<A, F> NodeDecl for ReceiverNodeDecl<A, F>
    where A: ToSocketAddrs,
          F: Format,
          F::Color: Black {
    type Target = ReceiverNode<F>;

    fn new(self, size: usize) -> Result<Self::Target, Error> {
        let socket = UdpSocket::bind(self.address)?;
        socket.set_nonblocking(true);

        return Ok(ReceiverNode {
            socket,
            buffer: vec![0; size * F::ELEMENT_SIZE].into_boxed_slice(),
            output: Buffer::black(size),
            format: PhantomData,
        });
    }
}

impl<F> Node for ReceiverNode<F>
    where F: Format {
    fn update(&mut self, _duration: &Duration) {
        // Read all packets available without blocking but only use last one
        loop {
            match self.socket.recv(&mut self.buffer) {
                Ok(_) => continue,
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                Err(e) => panic!("io error: {}", e),
            }
        }

        Iterator::zip(self.output.iter_mut(),
                      self.buffer.chunks(F::ELEMENT_SIZE))
            .for_each(|(o, b)| *o = F::load(b));
    }

    fn render<'a>(&'a self, _renderer: &'a Renderer) -> Box<Render + 'a> {
        return Box::new(&self.output);
    }
}

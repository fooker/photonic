use crate::netdmx::Channel;
use std::ops::Range;

pub struct Fixture<E: ?Sized> {
    address: usize,
    channels: Vec<Box<dyn Channel<E>>>,
}

impl<E: ?Sized> Fixture<E> {
    pub fn with_address(address: usize) -> Self {
        return Self {
            address,
            channels: Vec::new(),
        };
    }

    pub fn with_channel(mut self, channel: impl Channel<E> + 'static) -> Self {
        self.channels.push(Box::new(channel));
        return self;
    }

    pub fn addresses(&self) -> Range<usize> {
        return Range {
            start: self.address,
            end: self.address + self.channels.len(),
        };
    }

    pub fn channels(&self) -> impl Iterator<Item = (usize, &dyn Channel<E>)> {
        return self.channels.iter().enumerate().map(|(i, channel)| (self.address + i, channel.as_ref()));
    }
}

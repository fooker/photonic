use std::pin::pin;
use std::task::Context;

use futures::Future;
use tokio::sync::broadcast;

pub use sink::{AnyInputValue, InputSink, Sink};

mod attr;
mod sink;
mod values;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum InputValueType {
    Trigger,
    Boolean,
    Integer,
    Decimal,
    Color,
    IntegerRange,
    DecimalRange,
    ColorRange,
}

impl std::fmt::Display for InputValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Trigger => f.write_str("trigger"),
            Self::Boolean => f.write_str("boolean"),
            Self::Integer => f.write_str("integer"),
            Self::Decimal => f.write_str("decimal"),
            Self::Color => f.write_str("color"),
            Self::IntegerRange => f.write_str("range<integer>"),
            Self::DecimalRange => f.write_str("range<decimal>"),
            Self::ColorRange => f.write_str("range<color>"),
        };
    }
}

mod private {
    pub trait Sealed {}
}

pub trait InputValue: private::Sealed + Send + Copy + 'static {
    const TYPE: InputValueType;
    fn sink(sink: Sink<Self>) -> InputSink;
}

#[derive(Debug)]
pub enum Poll<T> {
    /// Represents that a value is immediately available.
    Update(T),

    /// Represents that a value has not changed.
    Pending,
}

#[derive(Debug)]
pub struct Input<V>
where V: InputValue
{
    //shared: Arc<Shared<V>>,
    tx: broadcast::Sender<V>,
    rx: broadcast::Receiver<V>,
}

impl<V> Input<V>
where V: InputValue
{
    pub fn new() -> Self {
        let (tx, rx) = broadcast::channel(1);

        return Self {
            tx,
            rx,
        };
    }

    pub fn poll(&mut self) -> Poll<V> {
        return match pin!(self.rx.recv()).as_mut().poll(&mut Context::from_waker(futures::task::noop_waker_ref())) {
            std::task::Poll::Ready(Ok(value)) => Poll::Update(value),
            std::task::Poll::Ready(Err(_)) => Poll::Pending, // We can ignore dangling or lagging here
            std::task::Poll::Pending => Poll::Pending,
        };
    }

    pub fn sink(&self) -> Sink<V> {
        return Sink {
            tx: self.tx.clone(),
        };
    }
}

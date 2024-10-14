use std::pin::pin;
use std::task::Context;

use anyhow::Result;
use futures::Future;
use tokio::sync::{broadcast, mpsc, oneshot};

pub use self::sink::{AnyInputValue, InputSink, Sink};
pub use self::trigger::Trigger;
pub use self::values::Coerced;

mod attr;
mod sink;
mod trigger;
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

struct UpdateRequest<V>
where V: InputValue
{
    value: V,
    responder: oneshot::Sender<Result<()>>,
}

#[derive(Debug)]
pub struct Input<V>
where V: InputValue
{
    update_tx: mpsc::Sender<UpdateRequest<V>>,
    update_rx: mpsc::Receiver<UpdateRequest<V>>,

    value_tx: broadcast::Sender<V>,
    value_rx: broadcast::Receiver<V>,
}

impl<V> Default for Input<V>
where V: InputValue
{
    fn default() -> Self {
        Self::new()
    }
}

impl<V> Input<V>
where V: InputValue
{
    pub fn new() -> Self {
        let (update_tx, update_rx) = mpsc::channel(1);

        let (value_tx, value_rx) = broadcast::channel(1);

        return Self {
            update_tx,
            update_rx,
            value_tx,
            value_rx,
        };
    }

    pub fn poll<F, R, E>(&mut self, validate: F) -> Poll<R>
    where
        F: Fn(V) -> Result<R, E>,
        E: Into<anyhow::Error>,
    {
        let ctx = &mut Context::from_waker(futures::task::noop_waker_ref());

        let std::task::Poll::Ready(Some(UpdateRequest {
            responder,
            value,
        })) = pin!(self.update_rx.recv()).as_mut().poll(ctx)
        else {
            return Poll::Pending;
        };

        match validate(value) {
            Ok(update) => {
                let _ = responder.send(Ok(()));
                let _ = self.value_tx.send(value);
                return Poll::Update(update);
            }
            Err(err) => {
                let _ = responder.send(Err(err.into()));
                return Poll::Pending;
            }
        }
    }

    pub fn sink(&self) -> Sink<V> {
        return Sink {
            update_tx: self.update_tx.clone(),
            value_rx: self.value_rx.resubscribe(),
        };
    }
}

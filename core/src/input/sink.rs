use futures::{Stream, StreamExt};
use palette::rgb::Rgb;
use std::pin::Pin;
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;

use crate::attr::Range;

use super::InputValue;

#[derive(Debug)]
pub struct Sink<V> {
    pub(super) tx: broadcast::Sender<V>,
}

impl<V> Sink<V>
where V: InputValue
{
    pub fn send(&self, next: V) {
        let _ = self.tx.send(next);
    }

    pub fn subscribe(&self) -> impl Stream<Item = V> + Send {
        return BroadcastStream::new(self.tx.subscribe()).filter_map(|r| async { r.ok() }); // Ignore lagging errors
    }
}

pub enum InputSink {
    Trigger(Sink<()>),
    Boolean(Sink<bool>),
    Integer(Sink<i64>),
    Decimal(Sink<f32>),
    Color(Sink<Rgb>),
    IntegerRange(Sink<Range<i64>>),
    DecimalRange(Sink<Range<f32>>),
    ColorRange(Sink<Range<Rgb>>),
}

impl std::fmt::Debug for InputSink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return std::fmt::Display::fmt(self, f);
    }
}

impl std::fmt::Display for InputSink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return f.write_str(match self {
            Self::Trigger(_) => "trigger",
            Self::Boolean(_) => "boolean",
            Self::Integer(_) => "integer",
            Self::Decimal(_) => "decimal",
            Self::Color(_) => "color",
            Self::IntegerRange(_) => "range<integer>",
            Self::DecimalRange(_) => "range<decimal>",
            Self::ColorRange(_) => "range<color>",
        });
    }
}

impl<V> From<Sink<V>> for InputSink
where V: InputValue
{
    fn from(sink: Sink<V>) -> Self {
        return V::sink(sink);
    }
}

impl InputSink {
    pub fn subscribe(&self) -> impl Stream<Item = AnyInputValue> + Send + Unpin {
        let result: Pin<Box<dyn Stream<Item = _> + Send>> = match self {
            InputSink::Trigger(sink) => Box::pin(sink.subscribe().map(AnyInputValue::from)),
            InputSink::Boolean(sink) => Box::pin(sink.subscribe().map(AnyInputValue::from)),
            InputSink::Integer(sink) => Box::pin(sink.subscribe().map(AnyInputValue::from)),
            InputSink::Decimal(sink) => Box::pin(sink.subscribe().map(AnyInputValue::from)),
            InputSink::Color(sink) => Box::pin(sink.subscribe().map(AnyInputValue::from)),
            InputSink::IntegerRange(sink) => Box::pin(sink.subscribe().map(AnyInputValue::from)),
            InputSink::DecimalRange(sink) => Box::pin(sink.subscribe().map(AnyInputValue::from)),
            InputSink::ColorRange(sink) => Box::pin(sink.subscribe().map(AnyInputValue::from)),
        };

        return result;
    }
}

#[derive(Debug, Copy, Clone)]
pub enum AnyInputValue {
    Trigger,
    Boolean(bool),
    Integer(i64),
    Decimal(f32),
    Color(Rgb),
    IntegerRange(Range<i64>),
    DecimalRange(Range<f32>),
    ColorRange(Range<Rgb>),
}

impl From<()> for AnyInputValue {
    fn from(_: ()) -> Self {
        return Self::Trigger;
    }
}

impl From<bool> for AnyInputValue {
    fn from(value: bool) -> Self {
        return Self::Boolean(value);
    }
}

impl From<i64> for AnyInputValue {
    fn from(value: i64) -> Self {
        return Self::Integer(value);
    }
}

impl From<f32> for AnyInputValue {
    fn from(value: f32) -> Self {
        return Self::Decimal(value);
    }
}

impl From<Rgb> for AnyInputValue {
    fn from(value: Rgb) -> Self {
        return Self::Color(value);
    }
}

impl From<Range<i64>> for AnyInputValue {
    fn from(value: Range<i64>) -> Self {
        return Self::IntegerRange(value);
    }
}

impl From<Range<f32>> for AnyInputValue {
    fn from(value: Range<f32>) -> Self {
        return Self::DecimalRange(value);
    }
}

impl From<Range<Rgb>> for AnyInputValue {
    fn from(value: Range<Rgb>) -> Self {
        return Self::ColorRange(value);
    }
}

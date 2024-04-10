use std::pin::Pin;
use std::str::FromStr;

use anyhow::Result;
use futures::{Stream, StreamExt, TryStreamExt};
use palette::rgb::Rgb;
use palette::Srgb;
use tokio::sync::broadcast;
use tokio::sync::broadcast::Receiver;
use tokio_stream::wrappers::BroadcastStream;

use crate::attr::Range;

use super::InputValue;

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
    pub fn send_from_str<P: InputValueParser>(&self, s: &str) -> Result<()> {
        return Ok(match self {
            Self::Trigger(sink) => sink.send(P::parse_trigger(s)?),
            Self::Boolean(sink) => sink.send(P::parse_boolean(s)?),
            Self::Integer(sink) => sink.send(P::parse_integer(s)?),
            Self::Decimal(sink) => sink.send(P::parse_decimal(s)?),
            Self::Color(sink) => sink.send(P::parse_color(s)?),
            Self::IntegerRange(sink) => sink.send(P::parse_integer_range(s)?),
            Self::DecimalRange(sink) => sink.send(P::parse_decimal_range(s)?),
            Self::ColorRange(sink) => sink.send(P::parse_color_range(s)?),
        });
    }

    pub fn subscribe(&self) -> impl Stream<Item = AnyInputValue> + Send + Unpin {
        return match self {
            InputSink::Trigger(sink) => {
                Box::pin(sink.subscribe().map(AnyInputValue::from)) as Pin<Box<dyn Stream<Item = _> + Send>>
            }
            InputSink::Boolean(sink) => Box::pin(sink.subscribe().map(AnyInputValue::from)),
            InputSink::Integer(sink) => Box::pin(sink.subscribe().map(AnyInputValue::from)),
            InputSink::Decimal(sink) => Box::pin(sink.subscribe().map(AnyInputValue::from)),
            InputSink::Color(sink) => Box::pin(sink.subscribe().map(AnyInputValue::from)),
            InputSink::IntegerRange(sink) => Box::pin(sink.subscribe().map(AnyInputValue::from)),
            InputSink::DecimalRange(sink) => Box::pin(sink.subscribe().map(AnyInputValue::from)),
            InputSink::ColorRange(sink) => Box::pin(sink.subscribe().map(AnyInputValue::from)),
        };
    }
}

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

impl AnyInputValue {
    pub fn format<F: InputValueFormatter>(self) -> String {
        return match self {
            AnyInputValue::Trigger => F::format_trigger(),
            AnyInputValue::Boolean(value) => F::format_boolean(value),
            AnyInputValue::Integer(value) => F::format_integer(value),
            AnyInputValue::Decimal(value) => F::format_decimal(value),
            AnyInputValue::Color(value) => F::format_color(value),
            AnyInputValue::IntegerRange(value) => F::format_integer_range(value),
            AnyInputValue::DecimalRange(value) => F::format_decimal_range(value),
            AnyInputValue::ColorRange(value) => F::format_color_range(value),
        };
    }
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

pub trait InputValueParser {
    fn parse_trigger(s: &str) -> Result<()>;
    fn parse_boolean(s: &str) -> Result<bool>;
    fn parse_integer(s: &str) -> Result<i64>;
    fn parse_decimal(s: &str) -> Result<f32>;
    fn parse_color(s: &str) -> Result<Rgb>;
    fn parse_integer_range(s: &str) -> Result<Range<i64>>;
    fn parse_decimal_range(s: &str) -> Result<Range<f32>>;
    fn parse_color_range(s: &str) -> Result<Range<Rgb>>;
}

pub trait InputValueFormatter {
    fn format_trigger() -> String;
    fn format_boolean(value: bool) -> String;
    fn format_integer(value: i64) -> String;
    fn format_decimal(value: f32) -> String;
    fn format_color(value: Rgb) -> String;
    fn format_integer_range(value: Range<i64>) -> String;
    fn format_decimal_range(value: Range<f32>) -> String;
    fn format_color_range(value: Range<Rgb>) -> String;
}

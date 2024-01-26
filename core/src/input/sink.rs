use std::str::FromStr;
use std::sync::Arc;
use std::sync::atomic::Ordering;

use anyhow::Result;
use palette::rgb::Rgb;
use palette::Srgb;

use crate::attr::Range;

use super::{InputValue, Poll, Shared};

#[derive(Clone)]
pub struct Sink<V>
{
    pub(super) shared: Arc<Shared<V>>,
}

impl<V> Sink<V>
    where V: InputValue {
    pub fn send(&self, next: V) {
        let mut value = self.shared.value.lock().expect("Failed to lock input value");
        *value = Poll::Update(next);
        self.shared.dirty.store(true, Ordering::Relaxed);
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
    pub fn send_str(&self, s: &str) -> Result<()> {
        fn parse_range<V>(s: &str) -> Result<Range<V>>
            where V: FromStr,
                  <V as FromStr>::Err: std::error::Error + Send + Sync + 'static,
        {
            let (s1, s2) = if let Some((s1, s2)) = s.split_once("..") {
                (s1, s2)
            } else {
                (s, s)
            };

            let v1 = s1.parse()?;
            let v2 = s2.parse()?;

            return Ok(Range(v1, v2));
        }

        return Ok(match self {
            Self::Trigger(sink) => {
                sink.send(());
            }
            Self::Boolean(sink) => {
                let value = s.parse()?;
                sink.send(value);
            }
            Self::Integer(sink) => {
                let value = s.parse()?;
                sink.send(value);
            }
            Self::Decimal(sink) => {
                let value = s.parse()?;
                sink.send(value);
            }
            Self::Color(sink) => {
                let value = s.parse::<Srgb<u8>>()?;
                sink.send(value.into_format());
            }
            Self::IntegerRange(sink) => {
                let value = parse_range(s)?;
                sink.send(value);
            }
            Self::DecimalRange(sink) => {
                let value = parse_range(s)?;
                sink.send(value);
            }
            Self::ColorRange(sink) => {
                let value = parse_range::<Srgb<u8>>(s)?;
                sink.send(value.map(|v| v.into_format()));
            }
        });
    }
}

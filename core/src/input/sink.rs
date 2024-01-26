use std::sync::Arc;
use std::sync::atomic::Ordering;

use palette::rgb::Rgb;

use crate::attr::Range;

use super::{InputValue, Poll, Shared};

#[derive(Clone)]
pub struct Sink<V>
{
    pub (super) shared: Arc<Shared<V>>,
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
    Decimal(Sink<f64>),
    Color(Sink<Rgb>),
    IntegerRange(Sink<Range<i64>>),
    DecimalRange(Sink<Range<f64>>),
    ColorRange(Sink<Range<Rgb>>),
}

impl std::fmt::Debug for InputSink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}()", match self {
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


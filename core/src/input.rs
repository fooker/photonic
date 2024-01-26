use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

pub use sink::{InputSink, Sink};

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
    Range(&'static InputValueType),
}

impl std::fmt::Display for InputValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Trigger => f.write_str("trigger"),
            Self::Boolean => f.write_str("boolean"),
            Self::Integer => f.write_str("integer"),
            Self::Decimal => f.write_str("decimal"),
            Self::Color => f.write_str("color"),
            Self::Range(t) => write!(f, "range<{}>", t),
        };
    }
}

pub trait InputValue: Send + Copy + 'static {
    const TYPE: InputValueType;
    fn sink(sink: Sink<Self>) -> InputSink;
}

pub enum Poll<T> {
    /// Represents that a value is immediately available.
    Update(T),

    /// Represents that a value has not changed.
    Pending,
}

struct Shared<V> {
    dirty: AtomicBool,
    value: Mutex<Poll<V>>,
}

pub struct Input<V>
    where V: InputValue,
{
    shared: Arc<Shared<V>>,
}

impl<V> Input<V>
    where V: InputValue,
{
    pub fn new() -> Self {
        return Self {
            shared: Arc::new(Shared {
                dirty: AtomicBool::new(false),
                value: Mutex::new(Poll::Pending),
            }),
        };
    }

    pub fn poll(&mut self) -> Poll<V> {
        if self.shared.dirty.swap(false, Ordering::Relaxed) {
            let mut value = self.shared.value.lock().expect("Failed to lock input value");
            return std::mem::replace(&mut *value, Poll::Pending);
        } else {
            return Poll::Pending;
        }
    }

    pub fn sink(&self) -> Sink<V> {
        return Sink {
            shared: self.shared.clone(),
        };
    }
}
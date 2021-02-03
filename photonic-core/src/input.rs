use std::sync::Arc;

use crossbeam::atomic::AtomicCell;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum InputValueType {
    Trigger,
    Bool,
    Integer,
    Decimal,
}

pub trait InputValue: Send + Copy + 'static {
    const TYPE: InputValueType;
}

pub enum Poll<V>
    where V: InputValue {
    Pending,
    Ready(V),
}

pub struct Input<V>
    where V: InputValue {
    value: Arc<AtomicCell<Poll<V>>>,
}

pub struct Sink<V>
    where V: InputValue {
    value: Arc<AtomicCell<Poll<V>>>,
}

impl<V> Input<V>
    where V: InputValue {
    pub fn new() -> (Self, Sink<V>) {
        let value = Arc::new(AtomicCell::new(Poll::Pending));

        return (
            Self { value: value.clone() },
            Sink { value: value.clone() },
        );
    }

    pub fn poll(&mut self) -> Poll<V> {
        return self.value.swap(Poll::Pending);
    }
}

impl<V> Sink<V>
    where V: InputValue {
    pub fn send(&mut self, next: V) {
        self.value.store(Poll::Ready(next));
    }
}

pub struct InputHandle<V>
    where V: InputValue {
    /// The scene-wide unique name of the input
    pub name: String,

    input: Input<V>,
}

impl<V> InputHandle<V>
    where V: InputValue {
    pub fn new(name: String) -> (Self, Sink<V>) {
        let (input, sink) = Input::new();

        return (Self {
            name,
            input,
        }, sink);
    }
}

impl<V> Into<Input<V>> for InputHandle<V>
    where V: InputValue {
    fn into(self) -> Input<V> {
        return self.input;
    }
}

impl InputValue for () {
    const TYPE: InputValueType = InputValueType::Trigger;
}

impl InputValue for bool {
    const TYPE: InputValueType = InputValueType::Bool;
}

impl InputValue for i64 {
    const TYPE: InputValueType = InputValueType::Integer;
}

impl InputValue for f64 {
    const TYPE: InputValueType = InputValueType::Decimal;
}
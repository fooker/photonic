use std::sync::Arc;

use crossbeam::atomic::AtomicCell;

pub enum InputType {
    Trigger,
    Bool,
    Integer,
    Decimal,
}

pub trait InputValue: Send + Copy + 'static {
    const TYPE: InputType;
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
    pub fn poll(&mut self) -> Poll<V> {
        return self.value.swap(Poll::Pending);
    }

    pub fn new() -> (Self, Sink<V>) {
        let value = Arc::new(AtomicCell::new(Poll::Pending));

        return (
            Self { value: value.clone() },
            Sink { value: value.clone() },
        );
    }
}

impl<V> Sink<V>
    where V: InputValue {
    pub fn send(&mut self, next: V) {
        self.value.store(Poll::Ready(next));
    }
}

impl InputValue for () {
    const TYPE: InputType = InputType::Trigger;
}

impl InputValue for bool {
    const TYPE: InputType = InputType::Bool;
}

impl InputValue for i64 {
    const TYPE: InputType = InputType::Integer;
}

impl InputValue for f64 {
    const TYPE: InputType = InputType::Decimal;
}

use std::sync::Arc;
use crossbeam::atomic::AtomicCell;

pub enum Poll<T> {
    Pending,
    Ready(T),
}

pub struct Input<T> {
    value: Arc<AtomicCell<Poll<T>>>,
}

pub struct Sink<T> {
    value: Arc<AtomicCell<Poll<T>>>,
}

impl<T> Input<T> {
    pub fn poll(&mut self) -> Poll<T> {
        return self.value.swap(Poll::Pending);
    }

    pub fn new() -> (Self, Sink<T>) {
        let value = Arc::new(AtomicCell::new(Poll::Pending));

        return (
            Self { value: value.clone() },
            Sink { value: value.clone() },
        );
    }
}

impl<T> Sink<T> {
    pub fn send(&mut self, next: T) {
        self.value.store(Poll::Ready(next));
    }
}

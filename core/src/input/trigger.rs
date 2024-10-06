use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Trigger(u64);

const NEXT: AtomicU64 = AtomicU64::new(0);

impl Trigger {
    pub fn next() -> Self {
        let next = NEXT.fetch_add(1, Ordering::Relaxed);
        return Self(next);
    }
}

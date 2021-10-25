use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;

struct Shared<T: Send> {
    buffers: [UnsafeCell<T>; 3],
    back_info: AtomicU8,
}

unsafe impl<T: Send> Sync for Shared<T> {}

impl<T: Send> Shared<T> {
    const BACK_DIRTY_MASK: u8 = 0b_1000_0000;
    const BACK_INDEX_MASK: u8 = u8::MAX ^ Self::BACK_DIRTY_MASK;
}

pub fn new<T: Send>(gen: impl Fn() -> T) -> (Writer<T>, Reader<T>) {
    let shared = Arc::new(Shared {
        buffers: [UnsafeCell::new(gen()), UnsafeCell::new(gen()), UnsafeCell::new(gen())],
        back_info: AtomicU8::new(0),
    });

    let writer = Writer {
        shared: shared.clone(),
        index: 1,
    };

    let reader = Reader {
        shared,
        index: 2,
    };

    return (writer, reader);
}

pub struct Writer<T: Send> {
    shared: Arc<Shared<T>>,
    index: u8,
}

impl<T: Send> Writer<T> {
    pub fn publish(&mut self) -> bool {
        let back_info =
            self.shared.back_info.swap(self.index | Shared::<T>::BACK_DIRTY_MASK, Ordering::AcqRel);
        self.index = back_info & Shared::<T>::BACK_INDEX_MASK;
        return back_info & Shared::<T>::BACK_DIRTY_MASK == 0;
    }
}

impl<T: Send> Deref for Writer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let ptr = self.shared.buffers[self.index as usize].get();
        return unsafe { &*ptr };
    }
}

impl<T: Send> DerefMut for Writer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let ptr = self.shared.buffers[self.index as usize].get();
        return unsafe { &mut *ptr };
    }
}

pub struct Reader<T: Send> {
    shared: Arc<Shared<T>>,
    index: u8,
}

impl<T: Send> Reader<T> {
    pub fn update(&mut self) -> bool {
        let updated =
            self.shared.back_info.load(Ordering::Relaxed) & Shared::<T>::BACK_DIRTY_MASK != 0;

        if updated {
            let back_info = self.shared.back_info.swap(self.index, Ordering::AcqRel);
            self.index = back_info & Shared::<T>::BACK_INDEX_MASK;
        }

        return updated;
    }
}

impl<T: Send> Deref for Reader<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let ptr = self.shared.buffers[self.index as usize].get();
        return unsafe { &*ptr };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_state() {
        let (w, r) = new(|| 42usize);

        assert_eq!(*r, 42);
        assert_eq!(*w, 42);
    }

    #[test]
    fn swap() {
        let (mut w, mut r) = new(|| 42usize);

        *w = 23;
        assert_eq!(w.publish(), true);
        assert_eq!(*r, 42);
        assert_eq!(r.update(), true);
        assert_eq!(*r, 23);

        *w = 1337;
        assert_eq!(w.publish(), true);
        assert_eq!(*r, 23);
        assert_eq!(r.update(), true);
        assert_eq!(*r, 1337);
    }

    #[test]
    fn fast_write() {
        let (mut w, mut r) = new(|| 42usize);

        *w = 23;
        assert_eq!(w.publish(), true);

        *w = 1337;
        assert_eq!(w.publish(), false);

        assert_eq!(*r, 42);
        assert_eq!(r.update(), true);
        assert_eq!(*r, 1337);
    }

    #[test]
    fn fast_read() {
        let (mut w, mut r) = new(|| 42usize);

        *w = 23;
        assert_eq!(*r, 42);
        assert_eq!(w.publish(), true);

        assert_eq!(*r, 42);
        assert_eq!(r.update(), true);
        assert_eq!(*r, 23);
        assert_eq!(r.update(), false);
        assert_eq!(*r, 23);

        *w = 1337;
        assert_eq!(*r, 23);
        assert_eq!(w.publish(), true);

        assert_eq!(*r, 23);
        assert_eq!(r.update(), true);
        assert_eq!(*r, 1337);
        assert_eq!(r.update(), false);
        assert_eq!(*r, 1337);
    }
}

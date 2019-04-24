use super::*;
use std::time::Duration;
use crate::math;
use std::sync::mpsc;
use crate::animation::Easing;

pub struct Fader<T> {
    bounds: Bounds<T>,

    current: T,

//    update: (mpsc::SyncSender<f64>, mpsc::Receiver<f64>),
}

impl<T> Value<T> for Fader<T>
    where T: Copy {
    fn get(&self) -> T {
        self.current
    }

    fn update(&mut self, duration: &Duration) -> Update<f64> {
//        if let Ok(update) = self.update.1.try_recv() {
//            self.current = update;
//            return Update::Changed(self.current);
//        } else {
//            return Update::Idle;
//        }
        unimplemented!()
    }
}

pub struct FaderDecl {}

impl<T> BoundValueDecl<T> for FaderDecl
    where T: Copy + 'static {
    fn new(self: Box<Self>, bounds: Bounds<T>) -> Result<Box<Value<T>>, Error> {
        let current = bounds.min;

        return Ok(Box::new(Fader {
            bounds,
            current,
        }));
    }
}

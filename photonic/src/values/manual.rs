use std::sync::mpsc;
use std::time::Duration;

use crate::math;
use crate::trigger::Timer;
use crate::values::Update;

use super::*;

pub struct Manual<T> {
    bounds: Option<Bounds<T>>,

    current: T,
}

impl<T> Value<T> for Manual<T>
    where T: Copy {
    fn get(&self) -> T {
        self.current
    }

    fn update(&mut self, duration: &Duration) -> Update<T> {
//        if let Ok(update) = self.update.1.try_recv() {
//            self.current = update;
//            return Update::Changed(self.current);
//        } else {
//            return Update::Idle;
//        }
        unimplemented!()
    }
}

pub struct ManualDecl {}

impl<T> BoundValueDecl<T> for ManualDecl
    where T: Copy + 'static {
    fn new(self: Box<Self>, bounds: Bounds<T>) -> Result<Box<Value<T>>, Error> {
        let current = bounds.min;

        return Ok(Box::new(Manual {
            bounds: Some(bounds),
            current,
        }));
    }
}

impl<T> UnboundValueDecl<T> for ManualDecl
    where T: Copy + Default + 'static {
    fn new(self: Box<Self>) -> Result<Box<Value<T>>, Error> {
        return Ok(Box::new(Manual {
            bounds: None,
            current: T::default(),
        }));
    }
}

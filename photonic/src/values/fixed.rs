use super::*;

struct FixedValue<T>(T);

impl<T> Value<T> for FixedValue<T>
    where T: Copy {
    fn get(&self) -> T {
        return self.0;
    }

    fn update(&mut self, _duration: &Duration) -> Update<T> {
        return Update::Idle;
    }
}

struct FixedValueDecl<T>(T);

impl<T> UnboundValueDecl<T> for FixedValueDecl<T>
    where T: Copy + 'static {
    fn new(self: Box<Self>) -> Result<Box<Value<T>>, Error> {
        return Ok(Box::new(FixedValue(self.0)));
    }
}

impl<T> From<T> for Box<UnboundValueDecl<T>>
    where T: Copy + 'static {
    fn from(value: T) -> Self {
        return Box::new(FixedValueDecl(value));
    }
}

impl<T> BoundValueDecl<T> for FixedValueDecl<T>
    where T: Copy + PartialOrd + fmt::Display + 'static {
    fn new(self: Box<Self>, bounds: Bounds<T>) -> Result<Box<Value<T>>, Error> {
        let value = bounds.ensure(self.0)?;

        return Ok(Box::new(FixedValue(value)));
    }
}

impl<T> From<T> for Box<BoundValueDecl<T>>
    where T: Copy + PartialOrd + fmt::Display + 'static {
    fn from(value: T) -> Self {
        return Box::new(FixedValueDecl(value));
    }
}
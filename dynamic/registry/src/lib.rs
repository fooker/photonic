use anyhow::Result;
use std::marker::PhantomData;

pub trait Producible {
    type Config;
}

pub trait Factory<T, B>
where
    T: Producible,
    B: ?Sized,
{
    fn produce(self: Box<Self>, config: T::Config, builder: &mut B) -> Result<T>;
}

impl<F, T, B> Factory<T, B> for F
where
    F: FnOnce(T::Config, &mut B) -> Result<T>,
    T: Producible,
{
    fn produce(self: Box<Self>, config: T::Config, builder: &mut B) -> Result<T> {
        return (self)(config, builder);
    }
}

pub trait Registry<T, B>
where
    T: Producible,
    B: ?Sized,
{
    fn lookup(kind: &str) -> Option<Box<dyn Factory<T, B>>>;
}

pub struct Combined<T, R1, R2, B>(PhantomData<(T, R1, R2, B)>)
where
    T: Producible,
    R1: Registry<T, B>,
    R2: Registry<T, B>,
    B: ?Sized;

impl<T, R1, R2, B> Registry<T, B> for Combined<T, R1, R2, B>
where
    T: Producible,
    R1: Registry<T, B>,
    R2: Registry<T, B>,
    B: ?Sized,
{
    fn lookup(kind: &str) -> Option<Box<dyn Factory<T, B>>> {
        return R1::lookup(kind).or_else(|| R2::lookup(kind));
    }
}

impl<T, B> Registry<T, B> for ()
where
    T: Producible,
    B: ?Sized,
{
    fn lookup(_kind: &str) -> Option<Box<dyn Factory<T, B>>> {
        return None;
    }
}

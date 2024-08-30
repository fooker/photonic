use anyhow::Result;
use serde::de::DeserializeOwned;
use serde::Deserialize;

use photonic_dynamic_boxed::{Boxed, BoxedBoundAttrDecl, BoxedFreeAttrDecl, BoxedNodeDecl, BoxedOutputDecl};

pub trait Factory<T, B>
where B: ?Sized
{
    fn produce(self: Box<Self>, config: serde_value::Value, builder: &mut B) -> Result<T>;
}

pub trait Producible {
    type Config: DeserializeOwned;
}

pub fn factory<P, T, F, B>(f: F) -> Box<dyn Factory<Box<T>, B>>
where
    T: ?Sized,
    P: Producible + Boxed<T> + Sized,
    F: for<'b> FnOnce(P::Config, &'b mut B) -> Result<P> + 'static,
{
    return Box::new(move |config, builder: &mut B| -> Result<Box<T>> {
        let config: P::Config = Deserialize::deserialize(config)?;
        let result = f(config, builder)?;
        return Ok(result.boxed());
    });
}

impl<F, T, B> Factory<T, B> for F
where F: for<'b> FnOnce(serde_value::Value, &'b mut B) -> Result<T> + 'static
{
    fn produce(self: Box<Self>, config: serde_value::Value, builder: &mut B) -> Result<T> {
        return self(config, builder);
    }
}

pub type NodeFactory<B> = Box<dyn Factory<BoxedNodeDecl, B>>;
pub type BoundAttrFactory<B, V> = Box<dyn Factory<BoxedBoundAttrDecl<V>, B>>;
pub type FreeAttrFactory<B, V> = Box<dyn Factory<BoxedFreeAttrDecl<V>, B>>;
pub type OutputFactory<B> = Box<dyn Factory<BoxedOutputDecl, B>>;

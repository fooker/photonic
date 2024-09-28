use std::marker::PhantomData;

use anyhow::Result;
use palette::rgb::Rgb;
use serde::de::DeserializeOwned;
use serde::Deserialize;

use photonic::boxed::{Boxed, DynBoundAttrDecl, DynFreeAttrDecl, DynNodeDecl, DynOutputDecl};

use crate::config::Anything;
use crate::registry::Registry;
use crate::{AttrBuilder, NodeBuilder, OutputBuilder};

pub trait Product {
    type Builder<'b, Reg: Registry + 'b>;
}

pub trait Factory<P, Reg: Registry>
where P: Product + ?Sized
{
    fn produce(self: Box<Self>, config: Anything, builder: P::Builder<'_, Reg>) -> Result<Box<P>>;
}

pub trait Producible<T>: DeserializeOwned
where T: Product + ?Sized
{
    type Product: Boxed<T>;

    fn produce<Reg: Registry>(config: Self, builder: T::Builder<'_, Reg>) -> Result<Self::Product>;
}

impl<P, T, Reg: Registry> Factory<T, Reg> for PhantomData<P>
where
    P: Producible<T>,
    T: Product + ?Sized,
{
    fn produce(self: Box<Self>, config: Anything, builder: T::Builder<'_, Reg>) -> Result<Box<T>> {
        let config: P = Deserialize::deserialize(config)?;
        let result = P::produce(config, builder)?;
        return Ok(result.boxed());
    }
}

pub fn factory<P>() -> Box<PhantomData<P>> {
    return Box::new(PhantomData::<P>);
}

pub type NodeFactory<Reg> = Box<dyn Factory<dyn DynNodeDecl<Rgb>, Reg>>;
pub type BoundAttrFactory<Reg, V> = Box<dyn Factory<dyn DynBoundAttrDecl<V>, Reg>>;
pub type FreeAttrFactory<Reg, V> = Box<dyn Factory<dyn DynFreeAttrDecl<V>, Reg>>;
pub type OutputFactory<Reg> = Box<dyn Factory<dyn DynOutputDecl, Reg>>;

impl Product for dyn DynNodeDecl<Rgb> {
    type Builder<'b, Reg: Registry + 'b> = NodeBuilder<'b, Reg>;
}

impl<V> Product for dyn DynFreeAttrDecl<V> {
    type Builder<'b, Reg: Registry + 'b> = AttrBuilder<'b, Reg>;
}

impl<V> Product for dyn DynBoundAttrDecl<V> {
    type Builder<'b, Reg: Registry + 'b> = AttrBuilder<'b, Reg>;
}

impl Product for dyn DynOutputDecl {
    type Builder<'b, Reg: Registry + 'b> = OutputBuilder<'b, Reg>;
}

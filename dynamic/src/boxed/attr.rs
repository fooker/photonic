use std::time::Duration;

use anyhow::Result;

use crate::config::Anything;
use photonic::attr::{Bounded, Bounds};
use photonic::{Attr, AttrBuilder, AttrValue, BoundAttrDecl, FreeAttrDecl};
use photonic_dynamic_registry::Producible;

pub trait DynFreeAttrDecl<V>
where V: AttrValue
{
    fn materialize(self: Box<Self>, builder: &mut AttrBuilder) -> Result<BoxedAttr<V>>;
}

impl<T, V> DynFreeAttrDecl<V> for T
where
    T: FreeAttrDecl<Value = V>,
    <T as FreeAttrDecl>::Attr: DynAttr<V> + Sized + 'static,
    V: AttrValue,
{
    fn materialize(self: Box<Self>, builder: &mut AttrBuilder) -> Result<BoxedAttr<V>> {
        let attr = <T as FreeAttrDecl>::materialize(*self, builder)?;
        return Ok(Box::new(attr) as Box<dyn DynAttr<V>>);
    }
}

pub type BoxedFreeAttrDecl<V> = Box<dyn DynFreeAttrDecl<V>>;

impl<V> Producible for BoxedFreeAttrDecl<V> {
    type Config = Anything;
}

impl<V> FreeAttrDecl for BoxedFreeAttrDecl<V>
where V: AttrValue
{
    type Value = V;
    type Attr = BoxedAttr<V>;

    fn materialize(self, builder: &mut AttrBuilder) -> Result<Self::Attr> {
        return DynFreeAttrDecl::materialize(self, builder);
    }
}

pub trait DynBoundAttrDecl<V>
where V: AttrValue
{
    fn materialize(self: Box<Self>, bounds: Bounds<V>, builder: &mut AttrBuilder) -> Result<BoxedAttr<V>>;
}

impl<T, V> DynBoundAttrDecl<V> for T
where
    T: BoundAttrDecl<Value = V>,
    <T as BoundAttrDecl>::Attr: DynAttr<V> + Sized + 'static,
    V: AttrValue,
{
    fn materialize(self: Box<Self>, bounds: Bounds<V>, builder: &mut AttrBuilder) -> Result<BoxedAttr<V>> {
        let attr = <T as BoundAttrDecl>::materialize(*self, bounds, builder)?;
        return Ok(Box::new(attr) as Box<dyn DynAttr<V>>);
    }
}

pub type BoxedBoundAttrDecl<V> = Box<dyn DynBoundAttrDecl<V>>;

impl<V> Producible for BoxedBoundAttrDecl<V> {
    type Config = Anything;
}

impl<V> BoundAttrDecl for BoxedBoundAttrDecl<V>
where V: AttrValue + Bounded
{
    type Value = V;
    type Attr = BoxedAttr<V>;

    fn materialize(self, bounds: Bounds<V>, builder: &mut AttrBuilder) -> Result<Self::Attr> {
        return DynBoundAttrDecl::materialize(self, bounds, builder);
    }
}

pub trait DynAttr<V>
where V: AttrValue
{
    fn update(&mut self, duration: Duration) -> V;
}

impl<T, V> DynAttr<V> for T
where
    T: Attr<Value = V>,
    V: AttrValue,
{
    fn update(&mut self, duration: Duration) -> V {
        return <T as Attr>::update(self, duration);
    }
}

pub type BoxedAttr<V> = Box<dyn DynAttr<V>>;

impl<V> Attr for BoxedAttr<V>
where V: AttrValue
{
    const KIND: &'static str = "todo!()";

    type Value = V;

    fn update(&mut self, duration: Duration) -> Self::Value {
        return DynAttr::update(self.as_mut(), duration);
    }
}

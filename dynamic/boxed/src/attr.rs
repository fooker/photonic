use anyhow::Result;

use photonic::attr::{Bounded, Bounds};
use photonic::{scene, Attr, AttrBuilder, AttrValue, BoundAttrDecl, FreeAttrDecl};

use crate::Boxed;

pub trait DynFreeAttrDecl<V>
where V: AttrValue
{
    fn materialize(self: Box<Self>, builder: &mut AttrBuilder) -> Result<BoxedAttr<V>>;
}

impl<T, V> DynFreeAttrDecl<V> for T
where
    T: FreeAttrDecl<V>,
    <T as FreeAttrDecl<V>>::Attr: DynAttr<V> + Sized + 'static,
    V: AttrValue,
{
    fn materialize(self: Box<Self>, builder: &mut AttrBuilder) -> Result<BoxedAttr<V>> {
        let attr = <T as FreeAttrDecl<V>>::materialize(*self, builder)?;
        return Ok(Box::new(attr));
    }
}

impl<T, V> Boxed<dyn DynFreeAttrDecl<V>> for T
where
    V: AttrValue,
    T: FreeAttrDecl<V> + 'static,
    T::Attr: Sized + 'static,
{
    fn boxed(self) -> Box<dyn DynFreeAttrDecl<V>> {
        return Box::new(self);
    }
}

pub type BoxedFreeAttrDecl<V> = Box<dyn DynFreeAttrDecl<V>>;

impl<V> FreeAttrDecl<V> for BoxedFreeAttrDecl<V>
where V: AttrValue
{
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
    T: BoundAttrDecl<V>,
    <T as BoundAttrDecl<V>>::Attr: DynAttr<V> + Sized + 'static,
    V: AttrValue + Bounded,
{
    fn materialize(self: Box<Self>, bounds: Bounds<V>, builder: &mut AttrBuilder) -> Result<BoxedAttr<V>> {
        let attr = <T as BoundAttrDecl<V>>::materialize(*self, bounds, builder)?;
        return Ok(Box::new(attr));
    }
}

impl<T, V> Boxed<dyn DynBoundAttrDecl<V>> for T
where
    T: BoundAttrDecl<V> + 'static,
    T::Attr: Sized + 'static,
    V: AttrValue + Bounded,
{
    fn boxed(self) -> Box<dyn DynBoundAttrDecl<V>> {
        return Box::new(self);
    }
}

pub type BoxedBoundAttrDecl<V> = Box<dyn DynBoundAttrDecl<V>>;

impl<V> BoundAttrDecl<V> for BoxedBoundAttrDecl<V>
where V: AttrValue + Bounded
{
    type Attr = BoxedAttr<V>;

    fn materialize(self, bounds: Bounds<V>, builder: &mut AttrBuilder) -> Result<Self::Attr> {
        return DynBoundAttrDecl::materialize(self, bounds, builder);
    }
}

pub trait DynAttr<V>
where V: AttrValue
{
    fn update(&mut self, ctx: &scene::RenderContext) -> V;
}

impl<T, V> DynAttr<V> for T
where
    T: Attr<V>,
    V: AttrValue,
{
    fn update(&mut self, ctx: &scene::RenderContext) -> V {
        return <T as Attr<V>>::update(self, ctx);
    }
}

pub type BoxedAttr<V> = Box<dyn DynAttr<V>>;

impl<V> Attr<V> for BoxedAttr<V>
where V: AttrValue
{
    const KIND: &'static str = "todo!()";

    fn update(&mut self, ctx: &scene::RenderContext) -> V {
        return DynAttr::update(self.as_mut(), ctx);
    }
}

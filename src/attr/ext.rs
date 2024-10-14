use anyhow::Result;
use num_traits::Num;
use std::marker::PhantomData;
use std::ops;

use crate::attr::{Bounded, Bounds};
use crate::{Attr, AttrBuilder, AttrValue, BoundAttrDecl, FreeAttrDecl, RenderContext};

pub trait FreeAttrDeclExt<V>: FreeAttrDecl<V> + Sized
where V: AttrValue
{
    fn map<F, R>(self, f: F) -> Map<Self, F, V, R>
    where
        F: Fn(V) -> R,
        V: AttrValue,
        R: AttrValue;

    fn scale(self, scale: V) -> Scale<Self, V>
    where V: Num;
}

pub trait BoundAttrDeclExt<V>: BoundAttrDecl<V> + Sized
where V: AttrValue + Bounded
{
    fn scale(self, scale: V) -> Scale<Self, V>
    where V: Num;
}

impl<V, Decl> FreeAttrDeclExt<V> for Decl
where
    V: AttrValue,
    Decl: FreeAttrDecl<V>,
{
    fn map<F, R>(self, f: F) -> Map<Self, F, V, R>
    where
        F: Fn(V) -> R,
        R: AttrValue,
    {
        return Map {
            inner: self,
            f,
            phantom: PhantomData,
        };
    }

    fn scale(self, scale: V) -> Scale<Self, V>
    where V: Num {
        return Scale {
            inner: self,
            scale,
            phantom: PhantomData,
        };
    }
}

impl<V, Decl> BoundAttrDeclExt<V> for Decl
where
    V: AttrValue + Bounded,
    Decl: BoundAttrDecl<V>,
{
    fn scale(self, scale: V) -> Scale<Self, V>
    where V: Num {
        return Scale {
            inner: self,
            scale,
            phantom: PhantomData,
        };
    }
}

#[derive(Debug)]
pub struct Map<Inner, F, V, R>
where
    F: Fn(V) -> R,
    V: AttrValue,
    R: AttrValue,
{
    inner: Inner,
    f: F,
    phantom: PhantomData<(V, R)>,
}

impl<Inner, F, V, R> FreeAttrDecl<R> for Map<Inner, F, V, R>
where
    Inner: FreeAttrDecl<V>,
    F: Fn(V) -> R,
    V: AttrValue,
    R: AttrValue,
{
    const KIND: &'static str = "map";

    type Attr = MapAttr<Inner::Attr, F, V, R>;

    fn materialize(self, builder: &mut AttrBuilder) -> Result<Self::Attr> {
        let inner = builder.unbound_attr("inner", self.inner)?;

        return Ok(Self::Attr {
            inner,
            f: self.f,
            phantom: self.phantom,
        });
    }
}

#[derive(Debug)]
pub struct MapAttr<Inner, F, V, R>
where
    Inner: Attr<V>,
    F: Fn(V) -> R,
    V: AttrValue,
    R: AttrValue,
{
    inner: Inner,
    f: F,
    phantom: PhantomData<(V, R)>,
}

impl<Inner, F, V, R> Attr<R> for MapAttr<Inner, F, V, R>
where
    Inner: Attr<V>,
    F: Fn(V) -> R,
    V: AttrValue,
    R: AttrValue,
{
    fn update(&mut self, ctx: &RenderContext) -> R {
        let value = self.inner.update(ctx);
        return (self.f)(value);
    }
}

#[derive(Debug)]
pub struct Scale<Inner, V>
where V: AttrValue
{
    inner: Inner,
    scale: V,
    phantom: PhantomData<V>,
}

impl<Inner, V> FreeAttrDecl<V> for Scale<Inner, V>
where
    Inner: FreeAttrDecl<V>,
    V: AttrValue + ops::Mul<Output = V>,
{
    const KIND: &'static str = "scale";
    type Attr = ScaleAttr<Inner::Attr, V>;

    fn materialize(self, builder: &mut AttrBuilder) -> Result<Self::Attr> {
        return Ok(Self::Attr {
            inner: builder.unbound_attr("inner", self.inner)?,
            scale: self.scale,
            phantom: self.phantom,
        });
    }
}

impl<Inner, V> BoundAttrDecl<V> for Scale<Inner, V>
where
    Inner: BoundAttrDecl<V>,
    V: AttrValue + Bounded + ops::Mul<Output = V> + ops::Div<Output = V>,
{
    const KIND: &'static str = "scale";
    type Attr = ScaleAttr<Inner::Attr, V>;

    fn materialize(self, bounds: Bounds<V>, builder: &mut AttrBuilder) -> Result<Self::Attr> {
        return Ok(Self::Attr {
            inner: builder.bound_attr("inner", self.inner, Bounds {
                min: bounds.min / self.scale,
                max: bounds.max / self.scale,
            })?,
            scale: self.scale,
            phantom: self.phantom,
        });
    }
}

pub struct ScaleAttr<Inner, V>
where
    Inner: Attr<V>,
    V: AttrValue,
{
    inner: Inner,
    scale: V,
    phantom: PhantomData<V>,
}

impl<Inner, V> Attr<V> for ScaleAttr<Inner, V>
where
    Inner: Attr<V>,
    V: AttrValue + ops::Mul<Output = V>,
{
    fn update(&mut self, ctx: &RenderContext) -> V {
        return self.inner.update(ctx) * self.scale;
    }
}

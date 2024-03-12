use std::time::Duration;

use anyhow::Result;

use crate::{Attr, AttrBuilder, AttrValue, FreeAttrDecl};

pub trait FreeAttrDeclExt: FreeAttrDecl {
    fn map<F, R>(self, f: F) -> impl FreeAttrDecl<Value=R>
        where F: Fn(Self::Value) -> R,
              R: AttrValue;
}

impl<Decl> FreeAttrDeclExt for Decl
    where Decl: FreeAttrDecl,
{
    fn map<F, R>(self, f: F) -> impl FreeAttrDecl<Value=R>
        where F: Fn(Decl::Value) -> R,
              R: AttrValue,
    {
        return FreeMapDecl { inner: self, f };
    }
}

pub struct FreeMapDecl<Inner, F, R>
    where Inner: FreeAttrDecl,
          F: Fn(Inner::Value) -> R,
          R: AttrValue,
{
    inner: Inner,
    f: F,
}

impl<Inner, F, R> FreeAttrDecl for FreeMapDecl<Inner, F, R>
    where Inner: FreeAttrDecl,
          F: Fn(Inner::Value) -> R,
          R: AttrValue,
{
    type Value = R;
    type Attr = MapAttr<Inner::Attr, F, R>;

    fn materialize(self, builder: &mut AttrBuilder) -> Result<Self::Attr> {
        let inner = builder.unbound_attr("inner", self.inner)?;

        return Ok(Self::Attr {
            inner,
            f: self.f,
        });
    }
}

pub struct MapAttr<Inner, F, R>
    where Inner: Attr,
          F: Fn(Inner::Value) -> R,
          R: AttrValue,
{
    inner: Inner,
    f: F,
}

impl<Inner, F, R> Attr for MapAttr<Inner, F, R>
    where Inner: Attr,
          F: Fn(Inner::Value) -> R,
          R: AttrValue,
{
    const KIND: &'static str = "map";

    type Value = R;

    fn update(&mut self, duration: Duration) -> Self::Value {
        let value = self.inner.update(duration);
        return (self.f)(value);
    }
}
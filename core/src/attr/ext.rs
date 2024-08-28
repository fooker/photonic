use std::time::Duration;

use anyhow::Result;

use crate::{Attr, AttrBuilder, AttrValue, FreeAttrDecl};

pub trait FreeAttrDeclExt: FreeAttrDecl + Sized {
    fn map<F, R>(self, f: F) -> FreeMapAttrDecl<Self, F, R>
        where
            F: Fn(Self::Value) -> R,
            R: AttrValue;
}

// pub trait BoundAttrDeclExt: BoundAttrDecl + Sized {
//     fn map<F, R>(self, f: F) -> BoundMapAttrDecl<Self, F, R>
//         where
//             F: Fn(Self::Value) -> R,
//             R: AttrValue;
// }

impl<Decl> FreeAttrDeclExt for Decl
    where Decl: FreeAttrDecl
{
    fn map<F, R>(self, f: F) -> FreeMapAttrDecl<Self, F, R>
        where
            F: Fn(Decl::Value) -> R,
            R: AttrValue,
    {
        return FreeMapAttrDecl {
            inner: self,
            f,
        };
    }
}

// impl<Decl> BoundAttrDeclExt for Decl
//     where Decl: BoundAttrDecl
// {
//     fn map<F, R>(self, f: F) -> BoundMapAttrDecl<Self, F, R>
//         where
//             F: Fn(Decl::Value) -> R,
//             R: AttrValue,
//     {
//         return BoundMapAttrDecl {
//             inner: self,
//             f,
//         };
//     }
// }

#[derive(Debug)]
pub struct FreeMapAttrDecl<Inner, F, R>
    where
        Inner: FreeAttrDecl,
        F: Fn(Inner::Value) -> R,
        R: AttrValue,
{
    inner: Inner,
    f: F,
}

// #[derive(Debug)]
// pub struct BoundMapAttrDecl<Inner, F, R>
//     where
//         Inner: BoundAttrDecl,
//         F: Fn(Inner::Value) -> R,
//         R: AttrValue,
// {
//     inner: Inner,
//     f: F,
// }

impl<Inner, F, R> FreeAttrDecl for FreeMapAttrDecl<Inner, F, R>
    where
        Inner: FreeAttrDecl,
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

// impl<Inner, F, R> BoundAttrDecl for BoundMapAttrDecl<Inner, F, R>
//     where
//         Inner: BoundAttrDecl,
//         F: Fn(Inner::Value) -> R,
//         R: AttrValue + Bounded,
// {
//     type Value = R;
//     type Attr = MapAttr<Inner::Attr, F, R>;
//
//     fn materialize(self, bounds: Bounds<R>, builder: &mut AttrBuilder) -> Result<Self::Attr> {
//         // let bounds = Bounds::from((
//         //     (self.f)(bounds.min),
//         //     (self.f)(bounds.max),
//         // ));
//
//         let inner = builder.bound_attr("inner", self.inner, todo!())?;
//
//         return Ok(Self::Attr {
//             inner,
//             f: self.f,
//         });
//     }
// }

#[derive(Debug)]
pub struct MapAttr<Inner, F, R>
    where
        Inner: Attr,
        F: Fn(Inner::Value) -> R,
        R: AttrValue,
{
    inner: Inner,
    f: F,
}

impl<Inner, F, R> Attr for MapAttr<Inner, F, R>
    where
        Inner: Attr,
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

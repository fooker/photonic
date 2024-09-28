use anyhow::Result;
use std::marker::PhantomData;

use crate::{scene, Attr, AttrBuilder, AttrValue, FreeAttrDecl};

pub trait FreeAttrDeclExt<V: AttrValue>: FreeAttrDecl<V> + Sized {
    fn map<F, R>(self, f: F) -> FreeMapAttrDecl<Self, F, V, R>
    where
        F: Fn(V) -> R,
        V: AttrValue,
        R: AttrValue;
}

// pub trait BoundAttrDeclExt: BoundAttrDecl + Sized {
//     fn map<F, R>(self, f: F) -> BoundMapAttrDecl<Self, F, R>
//         where
//             F: Fn(Self::Value) -> R,
//             R: AttrValue;
// }

impl<V, Decl> FreeAttrDeclExt<V> for Decl
where
    V: AttrValue,
    Decl: FreeAttrDecl<V>,
{
    fn map<F, R>(self, f: F) -> FreeMapAttrDecl<Self, F, V, R>
    where
        F: Fn(V) -> R,
        R: AttrValue,
    {
        return FreeMapAttrDecl {
            inner: self,
            f,
            phantom: PhantomData,
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
pub struct FreeMapAttrDecl<Inner, F, V, R>
where
    F: Fn(V) -> R,
    V: AttrValue,
    R: AttrValue,
{
    inner: Inner,
    f: F,
    phantom: PhantomData<(V, R)>,
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

impl<Inner, F, V, R> FreeAttrDecl<R> for FreeMapAttrDecl<Inner, F, V, R>
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
    fn update(&mut self, ctx: &scene::RenderContext) -> R {
        let value = self.inner.update(ctx);
        return (self.f)(value);
    }
}

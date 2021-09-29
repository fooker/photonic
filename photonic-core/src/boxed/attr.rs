use std::time::Duration;

use anyhow::Error;

use crate::attr::{Attr, AttrValue, BoundAttrDecl, Bounded, Bounds, UnboundAttrDecl, Update};
use crate::scene::AttrBuilder;
use crate::boxed::Wrap;

trait AsBoxedUnboundAttrDecl<V> {
    fn materialize(self: Box<Self>, builder: &mut AttrBuilder) -> Result<BoxedAttr<V>, Error>;
}

trait AsBoxedBoundAttrDecl<V> {
    fn materialize(
        self: Box<Self>,
        bounds: Bounds<V>,
        builder: &mut AttrBuilder,
    ) -> Result<BoxedAttr<V>, Error>;
}

impl<T, V> AsBoxedUnboundAttrDecl<V> for T
where
    T: UnboundAttrDecl<Element=V>,
    T::Target: 'static,
    V: AttrValue,
{
    fn materialize(self: Box<Self>, builder: &mut AttrBuilder) -> Result<BoxedAttr<V>, Error> {
        return T::materialize(*self, builder).map(BoxedAttr::wrap);
    }
}

impl<T, V> AsBoxedBoundAttrDecl<V> for T
where
    T: BoundAttrDecl<Element=V>,
    T::Target: 'static,
    V: AttrValue + Bounded,
{
    fn materialize(
        self: Box<Self>,
        bounds: Bounds<V>,
        builder: &mut AttrBuilder,
    ) -> Result<BoxedAttr<V>, Error> {
        return T::materialize(*self, bounds, builder).map(BoxedAttr::wrap);
    }
}

pub struct BoxedUnboundAttrDecl<V> {
    decl: Box<dyn AsBoxedUnboundAttrDecl<V>>,
}

pub struct BoxedBoundAttrDecl<V> {
    decl: Box<dyn AsBoxedBoundAttrDecl<V>>,
}

impl<V, Decl> Wrap<Decl> for BoxedUnboundAttrDecl<V>
where
    V: AttrValue,
    Decl: UnboundAttrDecl<Element=V> + 'static,
{
    fn wrap(decl: Decl) -> Self {
        return Self {
            decl: Box::new(decl),
        };
    }
}

impl<V, Decl> Wrap<Decl> for  BoxedBoundAttrDecl<V>
where
    V: AttrValue + Bounded,
    Decl: BoundAttrDecl<Element=V> + 'static,
{
    fn wrap(decl: Decl) -> Self {
        return Self {
            decl: Box::new(decl),
        };
    }
}

impl<V> UnboundAttrDecl for BoxedUnboundAttrDecl<V>
where
    V: AttrValue,
{
    type Element = V;
    type Target = BoxedAttr<V>;

    fn materialize(self, builder: &mut AttrBuilder) -> Result<Self::Target, Error> {
        return self.decl.materialize(builder);
    }
}

impl<V> BoundAttrDecl for BoxedBoundAttrDecl<V>
where
    V: AttrValue + Bounded,
{
    type Element = V;
    type Target = BoxedAttr<V>;

    fn materialize(
        self,
        bounds: Bounds<V>,
        builder: &mut AttrBuilder,
    ) -> Result<Self::Target, Error> {
        return self.decl.materialize(bounds, builder);
    }
}

trait AsBoxedAttr<V>
where
    V: AttrValue,
{
    fn get(&self) -> V;
    fn update(&mut self, duration: Duration) -> Update<V>;
}

impl<T, V> AsBoxedAttr<V> for T
where
    T: Attr<Element=V>,
    V: AttrValue,
{
    fn get(&self) -> V {
        return T::get(self);
    }

    fn update(&mut self, duration: Duration) -> Update<V> {
        return T::update(self, duration);
    }
}

pub struct BoxedAttr<V> {
    attr: Box<dyn AsBoxedAttr<V>>,
}

impl<V, Attr> Wrap<Attr> for BoxedAttr<V>
where
    V: AttrValue,
    Attr: self::Attr<Element=V> + 'static,
{
    fn wrap(attr: Attr) -> Self {
        return Self {
            attr: Box::new(attr),
        };
    }
}

impl<V> Attr for BoxedAttr<V>
where
    V: AttrValue,
{
    type Element = V;

    const KIND: &'static str = "boxed";

    fn get(&self) -> V {
        return self.attr.get();
    }

    fn update(&mut self, duration: Duration) -> Update<V> {
        return self.attr.update(duration);
    }
}

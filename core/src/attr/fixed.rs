use std::ops::Deref;
use std::time::Duration;

use anyhow::Result;

use crate::{Attr, AttrBuilder, AttrValue};
use crate::attr::{BoundAttrDecl, Bounded, Bounds, FreeAttrDecl};

pub struct FixedAttr<V>(V)
    where V: AttrValue;

impl<V> Attr for FixedAttr<V>
    where V: AttrValue,
{
    type Value = V;

    const KIND: &'static str = "fixed";

    fn update(&mut self, _duration: Duration) -> Self::Value {
        return self.0;
    }
}

impl<V> Deref for FixedAttr<V>
    where V: AttrValue,
{
    type Target = V;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

pub struct FixedAttrDecl<V>(V)
    where V: AttrValue;

impl<V> FreeAttrDecl for FixedAttrDecl<V>
    where V: AttrValue,
{
    type Value = V;
    type Target = FixedAttr<Self::Value>;

    fn materialize(self, _builder: &mut AttrBuilder) -> Result<Self::Target> {
        return Ok(FixedAttr(self.0));
    }
}

impl<V> BoundAttrDecl for FixedAttrDecl<V>
    where V: AttrValue + Bounded,
{
    type Value = V;
    type Target = FixedAttr<Self::Value>;

    fn materialize(self, bounds: Bounds<Self::Value>, _builder: &mut AttrBuilder) -> Result<Self::Target> {
        let value = bounds.ensure(self.0)?;
        return Ok(FixedAttr(value));
    }
}

pub trait AsFixedAttr<V>
    where V: AttrValue,
{
    fn fixed(self) -> FixedAttrDecl<V>;
}

impl<V, T> AsFixedAttr<V> for T
    where V: AttrValue + From<Self>,
{
    fn fixed(self) -> FixedAttrDecl<V> {
        return FixedAttrDecl(self.into());
    }
}
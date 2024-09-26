use std::ops::Deref;

use anyhow::Result;

use crate::attr::{Attr, AttrValue, Bounded, Bounds};
use crate::decl::{BoundAttrDecl, FreeAttrDecl};
use crate::{scene, AttrBuilder};

#[derive(Debug)]
pub struct FixedAttr<V>(V)
where V: AttrValue;

impl<V> Attr<V> for FixedAttr<V>
where V: AttrValue
{
    const KIND: &'static str = "fixed";

    fn update(&mut self, _ctx: &scene::RenderContext) -> V {
        return self.0;
    }
}

impl<V> Deref for FixedAttr<V>
where V: AttrValue
{
    type Target = V;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

#[derive(Debug)]
pub struct FixedAttrDecl<V>(V)
where V: AttrValue;

impl<V> FreeAttrDecl<V> for FixedAttrDecl<V>
where V: AttrValue
{
    type Attr = FixedAttr<V>;

    fn materialize(self, _builder: &mut AttrBuilder) -> Result<Self::Attr> {
        return Ok(FixedAttr(self.0));
    }
}

impl<V> BoundAttrDecl<V> for FixedAttrDecl<V>
where V: AttrValue + Bounded
{
    type Attr = FixedAttr<V>;

    fn materialize(self, bounds: Bounds<V>, _builder: &mut AttrBuilder) -> Result<Self::Attr> {
        let value = bounds.ensure(self.0)?;
        return Ok(FixedAttr(value));
    }
}

pub trait AsFixedAttr<V>
where V: AttrValue
{
    fn fixed(self) -> FixedAttrDecl<V>;
}

impl<V, T> AsFixedAttr<V> for T
where V: AttrValue + From<Self>
{
    fn fixed(self) -> FixedAttrDecl<V> {
        return FixedAttrDecl(self.into());
    }
}

use anyhow::{Error, format_err};
use serde::de::DeserializeOwned;
use serde_json::Value;

use photonic_core::color;
use photonic_core::attr::{AsFixedAttr, AttrValue, Bounded};
use photonic_core::boxed;
use photonic_core::input::InputValue;
use photonic_core::node::NodeDecl;
use photonic_core::output::OutputDecl;

use crate::config;
use crate::builder::Builder;

pub trait OutputModel: DeserializeOwned {
    fn assemble(self, builder: &mut Builder) -> Result<boxed::BoxedOutputDecl<color::RGBColor>, Error>;
}

impl<T> OutputModel for T
    where T: OutputDecl<Element=color::RGBColor> + DeserializeOwned + 'static {
    fn assemble(self, _builder: &mut Builder) -> Result<boxed::BoxedOutputDecl<color::RGBColor>, Error> {
        return Ok(boxed::BoxedOutputDecl::wrap(self));
    }
}

pub trait NodeModel: DeserializeOwned {
    fn assemble(self, builder: &mut Builder) -> Result<boxed::BoxedNodeDecl<color::RGBColor>, Error>;
}

impl<T> NodeModel for T
    where T: NodeDecl + DeserializeOwned + 'static,
          T::Element: Into<color::RGBColor> {
    fn assemble(self, _builder: &mut Builder) -> Result<boxed::BoxedNodeDecl<color::RGBColor>, Error> {
        return Ok(boxed::BoxedNodeDecl::wrap(self.map(Into::into)));
    }
}

pub trait UnboundAttrModel<V>: DeserializeOwned
    where V: AttrValueFactory {
    fn assemble(self, builder: &mut Builder) -> Result<boxed::BoxedUnboundAttrDecl<V>, Error>;
}

pub trait BoundAttrModel<V>: DeserializeOwned
    where V: AttrValueFactory + Bounded {
    fn assemble(self, builder: &mut Builder) -> Result<boxed::BoxedBoundAttrDecl<V>, Error>;
}

pub trait UnboundAttrFactory: AttrValueFactory {
    fn make_input(builder: &mut Builder, input: config::Input, initial: Value) -> Result<boxed::BoxedUnboundAttrDecl<Self>, Error>;
    fn make_fixed(builder: &mut Builder, value: Value) -> Result<boxed::BoxedUnboundAttrDecl<Self>, Error>;
}

pub trait BoundAttrFactory: AttrValueFactory + Bounded {
    fn make_input(builder: &mut Builder, input: config::Input, initial: Value) -> Result<boxed::BoxedBoundAttrDecl<Self>, Error>;
    fn make_fixed(builder: &mut Builder, value: Value) -> Result<boxed::BoxedBoundAttrDecl<Self>, Error>;
}

pub trait AttrValueFactory: AttrValue + Sized {
    type Model: DeserializeOwned;

    fn assemble(model: Self::Model) -> Result<Self, Error>;
}

impl AttrValueFactory for bool {
    type Model = Self;

    fn assemble(model: Self::Model) -> Result<Self, Error> {
        return Ok(model);
    }
}

impl AttrValueFactory for i64 {
    type Model = Self;

    fn assemble(model: Self::Model) -> Result<Self, Error> {
        return Ok(model);
    }
}

impl AttrValueFactory for f64 {
    type Model = Self;

    fn assemble(model: Self::Model) -> Result<Self, Error> {
        return Ok(model);
    }
}

impl<T> UnboundAttrFactory for T where T: AttrValueFactory {
    default fn make_input(_builder: &mut Builder, _input: config::Input, _initial: Value) -> Result<boxed::BoxedUnboundAttrDecl<Self>, Error> {
        return Err(format_err!("Input not supported for attributes of type {}", std::any::type_name::<Self>()));
    }

    default fn make_fixed(_builder: &mut Builder, value: Value) -> Result<boxed::BoxedUnboundAttrDecl<Self>, Error> {
        let value: Self::Model = serde_json::from_value(value)?;
        let value = Self::assemble(value)?;
        return Ok(boxed::BoxedUnboundAttrDecl::wrap(value.fixed()));
    }
}

impl<T> BoundAttrFactory for T where T: AttrValueFactory + Bounded {
    default fn make_input(_builder: &mut Builder, _input: config::Input, _initial: Value) -> Result<boxed::BoxedBoundAttrDecl<Self>, Error> {
        return Err(format_err!("Input not supported for attributes of type {}", std::any::type_name::<Self>()));
    }

    default fn make_fixed(_builder: &mut Builder, value: Value) -> Result<boxed::BoxedBoundAttrDecl<Self>, Error> {
        let value: Self::Model = serde_json::from_value(value)?;
        let value = Self::assemble(value)?;
        return Ok(boxed::BoxedBoundAttrDecl::wrap(value.fixed()));
    }
}

impl<T> UnboundAttrFactory for T where T: AttrValueFactory + InputValue {
    fn make_input(builder: &mut Builder, input: config::Input, initial: Value) -> Result<boxed::BoxedUnboundAttrDecl<Self>, Error> {
        let input = builder.input(input)?;

        let initial: Self::Model = serde_json::from_value(initial)?;
        let initial = Self::assemble(initial)?;

        return Ok(boxed::BoxedUnboundAttrDecl::wrap(input.attr(initial)));
    }
}

impl<T> BoundAttrFactory for T where T: AttrValueFactory + Bounded + InputValue {
    fn make_input(builder: &mut Builder, input: config::Input, initial: Value) -> Result<boxed::BoxedBoundAttrDecl<Self>, Error> {
        let input = builder.input(input)?;

        let initial: Self::Model = serde_json::from_value(initial)?;
        let initial = Self::assemble(initial)?;

        return Ok(boxed::BoxedBoundAttrDecl::wrap(input.attr(initial)));
    }
}

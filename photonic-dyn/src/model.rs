use anyhow::{format_err, Result};
use serde::{Deserialize, de::DeserializeOwned};
use serde_json::Value;

use photonic_core::attr::{AsFixedAttr, AttrValue, Bounded, Range};
use photonic_core::boxed::{BoxedBoundAttrDecl, BoxedNodeDecl, BoxedOutputDecl, BoxedUnboundAttrDecl, Wrap};
use photonic_core::color;
use photonic_core::input::InputValue;
use photonic_core::node::NodeDecl;
use photonic_core::output::OutputDecl;

use crate::builder::{AttrBuilder, InputBuilder, NodeBuilder, OutputBuilder};
use crate::config;
use photonic_core::color::RGBColor;

pub trait OutputModel: DeserializeOwned {
    fn assemble(self, builder: &mut dyn OutputBuilder) -> Result<BoxedOutputDecl<BoxedNodeDecl<RGBColor>>>;
}

impl<T> OutputModel for T
    where
        T: OutputDecl<BoxedNodeDecl<RGBColor>> + DeserializeOwned + 'static,
{
    fn assemble(self, _builder: &mut dyn OutputBuilder) -> Result<BoxedOutputDecl<BoxedNodeDecl<RGBColor>>> {
        return Ok(BoxedOutputDecl::wrap(self));
    }
}

pub trait NodeModel: DeserializeOwned {
    fn assemble(self, builder: &mut impl NodeBuilder) -> Result<BoxedNodeDecl<color::RGBColor>>;
}

impl<T> NodeModel for T
    where
        T: NodeDecl + DeserializeOwned + 'static,
        T::Element: Into<color::RGBColor>,
{
    fn assemble(self, _builder: &mut impl NodeBuilder) -> Result<BoxedNodeDecl<color::RGBColor>> {
        let decl = self.map(Into::into);
        return Ok(BoxedNodeDecl::wrap(decl));
    }
}

pub trait UnboundAttrModel<V>: DeserializeOwned
    where
        V: AttrValueFactory,
{
    fn assemble(self, builder: &mut impl AttrBuilder) -> Result<BoxedUnboundAttrDecl<V>>;
}

pub trait BoundAttrModel<V>: DeserializeOwned
    where
        V: AttrValueFactory + Bounded,
{
    fn assemble(self, builder: &mut impl AttrBuilder) -> Result<BoxedBoundAttrDecl<V>>;
}

pub trait UnboundAttrFactory: AttrValueFactory {
    fn make_input(
        builder: &mut impl InputBuilder,
        input: config::Input,
        initial: Value,
    ) -> Result<BoxedUnboundAttrDecl<Self>>;
    fn make_fixed(builder: &mut impl InputBuilder, value: Value) -> Result<BoxedUnboundAttrDecl<Self>>;
}

pub trait BoundAttrFactory: AttrValueFactory + Bounded {
    fn make_input(
        builder: &mut impl InputBuilder,
        input: config::Input,
        initial: Value,
    ) -> Result<BoxedBoundAttrDecl<Self>>;
    fn make_fixed(builder: &mut impl InputBuilder, value: Value) -> Result<BoxedBoundAttrDecl<Self>>;
}

pub trait AttrValueFactory: AttrValue + Sized {
    type Model: DeserializeOwned;

    fn assemble(model: Self::Model) -> Result<Self>;
}

impl AttrValueFactory for bool {
    type Model = Self;

    fn assemble(model: Self::Model) -> Result<Self> {
        return Ok(model);
    }
}

impl AttrValueFactory for i64 {
    type Model = Self;

    fn assemble(model: Self::Model) -> Result<Self> {
        return Ok(model);
    }
}

impl AttrValueFactory for f64 {
    type Model = Self;

    fn assemble(model: Self::Model) -> Result<Self> {
        return Ok(model);
    }
}

impl<T> UnboundAttrFactory for T
    where
        T: AttrValueFactory,
{
    default fn make_input(
        _builder: &mut impl InputBuilder,
        _input: config::Input,
        _initial: Value,
    ) -> Result<BoxedUnboundAttrDecl<Self>> {
        return Err(format_err!(
            "Input not supported for attributes of type {}",
            std::any::type_name::<Self>()
        ));
    }

    default fn make_fixed(
        _builder: &mut impl InputBuilder,
        value: Value,
    ) -> Result<BoxedUnboundAttrDecl<Self>> {
        let value: Self::Model = serde_json::from_value(value)?;
        let value = Self::assemble(value)?;
        return Ok(BoxedUnboundAttrDecl::wrap(value.fixed()));
    }
}

impl<T> BoundAttrFactory for T
    where
        T: AttrValueFactory + Bounded,
{
    default fn make_input(
        _builder: &mut impl InputBuilder,
        _input: config::Input,
        _initial: Value,
    ) -> Result<BoxedBoundAttrDecl<Self>> {
        return Err(format_err!(
            "Input not supported for attributes of type {}",
            std::any::type_name::<Self>()
        ));
    }

    default fn make_fixed(
        _builder: &mut impl InputBuilder,
        value: Value,
    ) -> Result<BoxedBoundAttrDecl<Self>> {
        let value: Self::Model = serde_json::from_value(value)?;
        let value = Self::assemble(value)?;
        return Ok(BoxedBoundAttrDecl::wrap(value.fixed()));
    }
}

impl<T> UnboundAttrFactory for T
    where
        T: AttrValueFactory + InputValue,
{
    fn make_input(
        builder: &mut impl InputBuilder,
        input: config::Input,
        initial: Value,
    ) -> Result<BoxedUnboundAttrDecl<Self>> {
        let input = builder.input(input)?;

        let initial: Self::Model = serde_json::from_value(initial)?;
        let initial = Self::assemble(initial)?;

        return Ok(BoxedUnboundAttrDecl::wrap(input.attr(initial)));
    }
}

impl<T> BoundAttrFactory for T
    where
        T: AttrValueFactory + Bounded + InputValue,
{
    fn make_input(
        builder: &mut impl InputBuilder,
        input: config::Input,
        initial: Value,
    ) -> Result<BoxedBoundAttrDecl<Self>> {
        let input = builder.input(input)?;

        let initial: Self::Model = serde_json::from_value(initial)?;
        let initial = Self::assemble(initial)?;

        return Ok(BoxedBoundAttrDecl::wrap(input.attr(initial)));
    }
}

#[derive(Deserialize)]
pub struct RangeModel<V>(pub V::Model, pub V::Model)
    where
        V: AttrValueFactory;

impl<V> AttrValueFactory for Range<V>
    where
        V: AttrValueFactory,
{
    type Model = RangeModel<V>;

    fn assemble(model: Self::Model) -> Result<Self> {
        return Ok(Range(V::assemble(model.0)?, V::assemble(model.1)?));
    }
}

impl AttrValueFactory for color::RGBColor {
    type Model = String;

    fn assemble(model: Self::Model) -> Result<Self> {
        let color = csscolorparser::parse(&model)?.to_lrgba();
        return Ok(color::RGBColor::new(color.0, color.1, color.2));
    }
}

impl AttrValueFactory for color::HSVColor {
    type Model = String;

    fn assemble(model: Self::Model) -> Result<Self> {
        let color = csscolorparser::parse(&model)?.to_hsva();
        return Ok(color::HSVColor::new(color.0, color.1, color.2));
    }
}

impl AttrValueFactory for color::HSLColor {
    type Model = String;

    fn assemble(model: Self::Model) -> Result<Self> {
        let color = csscolorparser::parse(&model)?.to_hsla();
        return Ok(color::HSLColor::new(color.0, color.1, color.2));
    }
}
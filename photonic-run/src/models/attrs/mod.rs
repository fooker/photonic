use std::time::Duration;

use anyhow::{format_err, Result};
use num::Num;
use rand::distributions::uniform::SampleUniform;
use serde::Deserialize;

use photonic_core::animation;
use photonic_core::attr::{Bounded, Range};
use photonic_core::boxed::{BoxedBoundAttrDecl, BoxedUnboundAttrDecl};
use photonic_core::color;
use photonic_core::math::Lerp;

use crate::builder::Builder;
use crate::config;
use crate::model::{AttrValueFactory, BoundAttrModel, UnboundAttrModel};

#[derive(Deserialize)]
pub struct ButtonModel<V>
where
    V: AttrValueFactory,
{
    pub value: (V::Model, V::Model),
    pub hold_time: Duration,
    pub trigger: config::Input,
}

impl<V> UnboundAttrModel<V> for ButtonModel<V>
where
    V: AttrValueFactory,
{
    fn assemble(self, builder: &mut Builder) -> Result<BoxedUnboundAttrDecl<V>> {
        return Ok(BoxedUnboundAttrDecl::wrap(
            photonic_effects::attrs::button::ButtonDecl {
                value: (V::assemble(self.value.0)?, V::assemble(self.value.1)?),
                hold_time: self.hold_time,
                trigger: builder.input(self.trigger)?,
            },
        ));
    }
}

impl<V> BoundAttrModel<V> for ButtonModel<V>
where
    V: AttrValueFactory + Bounded,
{
    fn assemble(self, builder: &mut Builder) -> Result<BoxedBoundAttrDecl<V>> {
        return Ok(BoxedBoundAttrDecl::wrap(
            photonic_effects::attrs::button::ButtonDecl {
                value: (V::assemble(self.value.0)?, V::assemble(self.value.1)?),
                hold_time: self.hold_time,
                trigger: builder.input(self.trigger)?,
            },
        ));
    }
}

#[derive(Deserialize)]
pub struct EasingModel {
    pub func: String,

    #[serde(with = "humantime_serde")]
    pub speed: Duration,
}

impl EasingModel {
    fn func<F: animation::Float>(func: &str) -> Option<fn(F) -> F> {
        return match func {
            "linear" => Some(animation::linear),
            "quad_in" => Some(animation::quad_in),
            "quad_out" => Some(animation::quad_out),
            "quad_inout" => Some(animation::quad_inout),
            "cubic_in" => Some(animation::cubic_in),
            "cubic_out" => Some(animation::cubic_out),
            "cubic_inout" => Some(animation::cubic_inout),
            "quart_in" => Some(animation::quart_in),
            "quart_out" => Some(animation::quart_out),
            "quart_inout" => Some(animation::quart_inout),
            "quint_in" => Some(animation::quint_in),
            "quint_out" => Some(animation::quint_out),
            "quint_inout" => Some(animation::quint_inout),
            "sine_in" => Some(animation::sine_in),
            "sine_out" => Some(animation::sine_out),
            "sine_inout" => Some(animation::sine_inout),
            "circ_in" => Some(animation::circ_in),
            "circ_out" => Some(animation::circ_out),
            "circ_inout" => Some(animation::circ_inout),
            "expo_in" => Some(animation::expo_in),
            "expo_out" => Some(animation::expo_out),
            "expo_inout" => Some(animation::expo_inout),
            "elastic_in" => Some(animation::elastic_in),
            "elastic_out" => Some(animation::elastic_out),
            "elastic_inout" => Some(animation::elastic_inout),
            "back_in" => Some(animation::back_in),
            "back_out" => Some(animation::back_out),
            "back_inout" => Some(animation::back_inout),
            "bounce_in" => Some(animation::bounce_in),
            "bounce_out" => Some(animation::bounce_out),
            "bounce_inout" => Some(animation::bounce_inout),
            _ => None,
        };
    }

    pub fn resemble<F>(self) -> Result<animation::Easing<F>>
    where
        F: animation::Float,
    {
        return Ok(animation::Easing {
            func: Self::func(&self.func)
                .ok_or_else(|| format_err!("Unknown easing function: {}", self.func))?,
            speed: self.speed,
        });
    }
}

#[derive(Deserialize)]
pub struct FaderModel {
    pub input: config::Attr,
    pub easing: EasingModel,
}

impl<V> UnboundAttrModel<V> for FaderModel
where
    V: AttrValueFactory,
{
    default fn assemble(
        self,
        _builder: &mut Builder,
    ) -> Result<BoxedUnboundAttrDecl<V>> {
        return Err(format_err!(
            "Fader is not supported for Attributes of Type {}",
            std::any::type_name::<V>()
        ));
    }
}

impl<V> UnboundAttrModel<V> for FaderModel
where
    V: AttrValueFactory + Lerp,
{
    fn assemble(self, builder: &mut Builder) -> Result<BoxedUnboundAttrDecl<V>> {
        return Ok(BoxedUnboundAttrDecl::wrap(
            photonic_effects::attrs::fader::FaderDecl {
                input: builder.unbound_attr("input", self.input)?,
                easing: self.easing.resemble()?,
            },
        ));
    }
}

impl<V> BoundAttrModel<V> for FaderModel
where
    V: AttrValueFactory + Bounded,
{
    default fn assemble(
        self,
        _builder: &mut Builder,
    ) -> Result<BoxedBoundAttrDecl<V>> {
        return Err(format_err!(
            "Fader is not supported for Attributes of Type {}",
            std::any::type_name::<V>()
        ));
    }
}

impl<V> BoundAttrModel<V> for FaderModel
where
    V: AttrValueFactory + Bounded + Lerp,
{
    fn assemble(self, builder: &mut Builder) -> Result<BoxedBoundAttrDecl<V>> {
        return Ok(BoxedBoundAttrDecl::wrap(
            photonic_effects::attrs::fader::FaderDecl {
                input: builder.bound_attr("input", self.input)?,
                easing: self.easing.resemble()?,
            },
        ));
    }
}

#[derive(Deserialize)]
pub struct LooperModel<V>
where
    V: AttrValueFactory,
{
    pub step: V::Model,
    pub trigger: config::Input,
}

impl<V> BoundAttrModel<V> for LooperModel<V>
where
    V: AttrValueFactory + Bounded,
{
    default fn assemble(
        self,
        _builder: &mut Builder,
    ) -> Result<BoxedBoundAttrDecl<V>> {
        return Err(format_err!(
            "Looper is not supported for Attributes of Type {}",
            std::any::type_name::<V>()
        ));
    }
}

impl<V> BoundAttrModel<V> for LooperModel<V>
where
    V: AttrValueFactory + Bounded + Num + PartialOrd,
{
    fn assemble(self, builder: &mut Builder) -> Result<BoxedBoundAttrDecl<V>> {
        return Ok(BoxedBoundAttrDecl::wrap(
            photonic_effects::attrs::looper::LooperDecl {
                step: V::assemble(self.step)?,
                trigger: builder.input(self.trigger)?,
            },
        ));
    }
}

#[derive(Deserialize)]
pub struct RandomModel {
    pub trigger: config::Input,
}

impl<V> BoundAttrModel<V> for RandomModel
where
    V: AttrValueFactory + Bounded,
{
    default fn assemble(
        self,
        _builder: &mut Builder,
    ) -> Result<BoxedBoundAttrDecl<V>> {
        return Err(format_err!(
            "Random is not supported for Attributes of Type {}",
            std::any::type_name::<V>()
        ));
    }
}

impl<V> BoundAttrModel<V> for RandomModel
where
    V: AttrValueFactory + Bounded + SampleUniform,
{
    fn assemble(self, builder: &mut Builder) -> Result<BoxedBoundAttrDecl<V>> {
        return Ok(BoxedBoundAttrDecl::wrap(
            photonic_effects::attrs::random::RandomDecl {
                trigger: builder.input(self.trigger)?,
            },
        ));
    }
}

#[derive(Deserialize)]
pub struct SequenceModel<V>
where
    V: AttrValueFactory,
{
    pub values: Vec<V::Model>,
    pub next: Option<config::Input>,
    pub prev: Option<config::Input>,
}

impl<V> UnboundAttrModel<V> for SequenceModel<V>
where
    V: AttrValueFactory,
{
    fn assemble(self, builder: &mut Builder) -> Result<BoxedUnboundAttrDecl<V>> {
        return Ok(BoxedUnboundAttrDecl::wrap(
            photonic_effects::attrs::sequence::SequenceDecl {
                values: self
                    .values
                    .into_iter()
                    .map(V::assemble)
                    .collect::<Result<Vec<_>>>()?,
                next: self.next.map(|i| builder.input(i)).transpose()?,
                prev: self.prev.map(|i| builder.input(i)).transpose()?,
            },
        ));
    }
}

impl<V> BoundAttrModel<V> for SequenceModel<V>
where
    V: AttrValueFactory + Bounded,
{
    fn assemble(self, builder: &mut Builder) -> Result<BoxedBoundAttrDecl<V>> {
        return Ok(BoxedBoundAttrDecl::wrap(
            photonic_effects::attrs::sequence::SequenceDecl {
                values: self
                    .values
                    .into_iter()
                    .map(V::assemble)
                    .collect::<Result<Vec<_>>>()?,
                next: self.next.map(|i| builder.input(i)).transpose()?,
                prev: self.prev.map(|i| builder.input(i)).transpose()?,
            },
        ));
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

use anyhow::Error;
use serde::Deserialize;

use photonic_core::{boxed, NodeDecl};
use photonic_core::boxed::BoxedNodeDecl;
use photonic_core::color;

use crate::config;
use crate::model::NodeModel;
use crate::builder::Builder;

#[derive(Deserialize)]
pub struct AfterglowConfig {
    pub source: config::Node,
    pub decay: config::Attr,
}

impl NodeModel for AfterglowConfig {
    fn assemble(self, builder: &mut Builder) -> Result<boxed::BoxedNodeDecl<color::RGBColor>, Error> {
        return Ok(BoxedNodeDecl::wrap(
            photonic_effects::nodes::afterglow::AfterglowNodeDecl {
                source: builder.node("source", self.source)?,
                decay: builder.bound_attr("decay", self.decay)?,
            }));
    }
}

#[derive(Deserialize)]
pub struct AlertConfig {
    pub hue: config::Attr,
    pub block: config::Attr,
    pub speed: config::Attr,
}

impl NodeModel for AlertConfig {
    fn assemble(self, builder: &mut Builder) -> Result<boxed::BoxedNodeDecl<color::RGBColor>, Error> {
        return Ok(BoxedNodeDecl::wrap(
            photonic_effects::nodes::alert::AlertNodeDecl {
                hue: builder.bound_attr("hue", self.hue)?,
                block: builder.bound_attr("block", self.block)?,
                speed: builder.unbound_attr("speed", self.speed)?,
            }.map(Into::into)));
    }
}

#[derive(Deserialize)]
pub struct BlackoutConfig {
    pub source: config::Node,
    pub active: config::Attr,
    pub range: Option<(usize, usize)>,
}

impl NodeModel for BlackoutConfig {
    fn assemble(self, builder: &mut Builder) -> Result<boxed::BoxedNodeDecl<color::RGBColor>, Error> {
        return Ok(BoxedNodeDecl::wrap(
            photonic_effects::nodes::blackout::BlackoutNodeDecl {
                source: builder.node("source", self.source)?,
                active: builder.unbound_attr("active", self.active)?,
                range: self.range,
            }));
    }
}

#[derive(Deserialize)]
pub struct BrightnessConfig {
    pub source: config::Node,
    pub brightness: config::Attr,
    pub range: Option<(usize, usize)>,
}

impl NodeModel for BrightnessConfig {
    fn assemble(self, builder: &mut Builder) -> Result<boxed::BoxedNodeDecl<color::RGBColor>, Error> {
        return Ok(BoxedNodeDecl::wrap(
            photonic_effects::nodes::brightness::BrightnessNodeDecl {
                source: builder.node("source", self.source)?,
                brightness: builder.bound_attr("brightness", self.brightness)?,
                range: self.range,
            }));
    }
}

#[derive(Deserialize)]
pub struct RaindropsConfig {
    pub rate: config::Attr,
    pub color: config::Attr,
    pub decay: config::Attr,
}

impl NodeModel for RaindropsConfig {
    fn assemble(self, builder: &mut Builder) -> Result<boxed::BoxedNodeDecl<color::RGBColor>, Error> {
        return Ok(BoxedNodeDecl::wrap(
            photonic_effects::nodes::raindrops::RaindropsNodeDecl {
                rate: builder.bound_attr("rate", self.rate)?,
                color: builder.unbound_attr("color", self.color)?,
                decay: builder.bound_attr("decay", self.decay)?,
            }.map(Into::into)));
    }
}

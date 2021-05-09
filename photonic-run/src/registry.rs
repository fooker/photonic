use std::marker::PhantomData;

use anyhow::{format_err, Result};
use serde_json::Value;

use photonic_core::attr::Bounded;
use photonic_core::boxed::{
    BoxedBoundAttrDecl, BoxedNodeDecl, BoxedOutputDecl, BoxedUnboundAttrDecl,
};
use photonic_core::{color, NodeDecl};

use crate::builder::Builder;
use crate::model::{AttrValueFactory, BoundAttrModel, NodeModel, OutputModel, UnboundAttrModel};
use crate::models;

pub struct Registry<T> {
    phantom: PhantomData<T>,
}

pub type OutputRegistry = Registry<BoxedOutputDecl<color::RGBColor>>;

impl OutputRegistry {
    pub fn manufacture(
        kind: &str,
        config: Value,
        builder: &mut Builder,
    ) -> Result<BoxedOutputDecl<color::RGBColor>> {
        fn factory<T>(
            config: Value,
            builder: &mut Builder,
        ) -> Result<BoxedOutputDecl<color::RGBColor>>
        where
            T: OutputModel,
        {
            let model: T = serde_json::from_value(config)?;
            let decl = T::assemble(model, builder)?;
            return Ok(BoxedOutputDecl::wrap(decl));
        }

        return (match kind {
            "console" => factory::<photonic_console::ConsoleOutputDecl>,
            "led-strip" => factory::<photonic_ledstrip::LedStripOutputDecl>,

            _ => return Err(format_err!("Unknown output type: {}", kind)),
        })(config, builder);
    }
}

pub type NodeRegistry = Registry<BoxedNodeDecl<color::RGBColor>>;

impl NodeRegistry {
    pub fn manufacture(
        kind: &str,
        config: Value,
        builder: &mut Builder,
    ) -> Result<BoxedNodeDecl<color::RGBColor>> {
        fn factory<T>(
            config: Value,
            builder: &mut Builder,
        ) -> Result<BoxedNodeDecl<color::RGBColor>>
        where
            T: NodeModel + 'static,
        {
            let model: T = serde_json::from_value(config)?;
            let decl = T::assemble(model, builder)?;
            return Ok(BoxedNodeDecl::wrap(decl.map(Into::into)));
        }

        return (match kind {
            "afterglow" => factory::<models::nodes::AfterglowConfig>,
            "alert" => factory::<models::nodes::AlertConfig>,
            "blackout" => factory::<models::nodes::BlackoutConfig>,
            "brightness" => factory::<models::nodes::BrightnessConfig>,
            "raindrops" => factory::<models::nodes::RaindropsConfig>,

            _ => return Err(format_err!("Unknown node type: {}", kind)),
        })(config, builder);
    }
}

pub type UnboundAttrRegistry<V> = Registry<BoxedUnboundAttrDecl<V>>;

impl<V> UnboundAttrRegistry<V>
where
    V: AttrValueFactory,
{
    pub fn manufacture(
        kind: &str,
        config: Value,
        builder: &mut Builder,
    ) -> Result<BoxedUnboundAttrDecl<V>> {
        fn factory<T, V>(config: Value, builder: &mut Builder) -> Result<BoxedUnboundAttrDecl<V>>
        where
            T: UnboundAttrModel<V> + 'static,
            V: AttrValueFactory,
        {
            let model: T = serde_json::from_value(config)?;
            let decl = T::assemble(model, builder)?;
            return Ok(BoxedUnboundAttrDecl::wrap(decl));
        }

        return (match kind {
            "button" => factory::<models::attrs::ButtonModel<V>, V>,
            "fader" => factory::<models::attrs::FaderModel, V>,
            "sequence" => factory::<models::attrs::SequenceModel<V>, V>,

            _ => return Err(format_err!("Unknown unbound attribute type: {}", kind)),
        })(config, builder);
    }
}

pub type BoundAttrRegistry<V> = Registry<BoxedBoundAttrDecl<V>>;

impl<V> BoundAttrRegistry<V>
where
    V: AttrValueFactory + Bounded,
{
    pub fn manufacture(
        kind: &str,
        config: Value,
        builder: &mut Builder,
    ) -> Result<BoxedBoundAttrDecl<V>> {
        fn factory<T, V>(config: Value, builder: &mut Builder) -> Result<BoxedBoundAttrDecl<V>>
        where
            T: BoundAttrModel<V> + 'static,
            V: AttrValueFactory + Bounded,
        {
            let model: T = serde_json::from_value(config)?;
            let decl = T::assemble(model, builder)?;
            return Ok(BoxedBoundAttrDecl::wrap(decl));
        }

        return (match kind {
            "button" => factory::<models::attrs::ButtonModel<V>, V>,
            "fader" => factory::<models::attrs::FaderModel, V>,
            "looper" => factory::<models::attrs::LooperModel<V>, V>,
            "random" => factory::<models::attrs::RandomModel, V>,
            "sequence" => factory::<models::attrs::SequenceModel<V>, V>,
            _ => return Err(format_err!("Unknown bound attribute type: {}", kind)),
        })(config, builder);
    }
}

use std::marker::PhantomData;

use anyhow::{Error, format_err};
use serde_json::Value;

use photonic_core::{boxed, color, NodeDecl};
use photonic_core::attr::Bounded;

use crate::model::{AttrValueFactory, BoundAttrModel, NodeModel, OutputModel, UnboundAttrModel};
use crate::models;
use crate::builder::Builder;

pub struct Registry<T> {
    phantom: PhantomData<T>,
}

pub type OutputRegistry = Registry<boxed::BoxedOutputDecl<color::RGBColor>>;

impl OutputRegistry {
    pub fn manufacture(kind: &str, config: Value, builder: &mut Builder) -> Result<boxed::BoxedOutputDecl<color::RGBColor>, Error> {
        fn factory<T>(config: Value, builder: &mut Builder) -> Result<boxed::BoxedOutputDecl<color::RGBColor>, Error>
            where T: OutputModel {
            let model: T = serde_json::from_value(config)?;
            let decl = T::assemble(model, builder)?;
            return Ok(boxed::BoxedOutputDecl::wrap(decl));
        }

        return (match kind {
            "console" => factory::<photonic_console::ConsoleOutputDecl>,
            _ => return Err(format_err!("Unknown output type: {}", kind)),
        })(config, builder);
    }
}

pub type NodeRegistry = Registry<boxed::BoxedNodeDecl<color::RGBColor>>;

impl NodeRegistry {
    pub fn manufacture(kind: &str, config: Value, builder: &mut Builder) -> Result<boxed::BoxedNodeDecl<color::RGBColor>, Error> {
        fn factory<T>(config: Value, builder: &mut Builder) -> Result<boxed::BoxedNodeDecl<color::RGBColor>, Error>
            where T: NodeModel + 'static {
            let model: T = serde_json::from_value(config)?;
            let decl = T::assemble(model, builder)?;
            return Ok(boxed::BoxedNodeDecl::wrap(decl.map(Into::into)));
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

pub type UnboundAttrRegistry<V> = Registry<boxed::BoxedUnboundAttrDecl<V>>;


impl<V> UnboundAttrRegistry<V>
    where V: AttrValueFactory {
    pub fn manufacture(kind: &str, config: Value, builder: &mut Builder) -> Result<boxed::BoxedUnboundAttrDecl<V>, Error> {
        fn factory<T, V>(config: Value, builder: &mut Builder) -> Result<boxed::BoxedUnboundAttrDecl<V>, Error>
            where T: UnboundAttrModel<V> + 'static,
                  V: AttrValueFactory {
            let model: T = serde_json::from_value(config)?;
            let decl = T::assemble(model, builder)?;
            return Ok(boxed::BoxedUnboundAttrDecl::wrap(decl));
        }

        return (match kind {
            "button" => factory::<models::attrs::ButtonModel<V>, V>,
            "fader" => factory::<models::attrs::FaderModel, V>,
            "sequence" => factory::<models::attrs::SequenceModel<V>, V>,

            _ => return Err(format_err!("Unknown unbound attribute type: {}", kind))
        })(config, builder);
    }
}

pub type BoundAttrRegistry<V> = Registry<boxed::BoxedBoundAttrDecl<V>>;

impl<V> BoundAttrRegistry<V>
    where V: AttrValueFactory + Bounded {
    pub fn manufacture(kind: &str, config: Value, builder: &mut Builder) -> Result<boxed::BoxedBoundAttrDecl<V>, Error> {
        fn factory<T, V>(config: Value, builder: &mut Builder) -> Result<boxed::BoxedBoundAttrDecl<V>, Error>
            where T: BoundAttrModel<V> + 'static,
                  V: AttrValueFactory + Bounded {
            let model: T = serde_json::from_value(config)?;
            let decl = T::assemble(model, builder)?;
            return Ok(boxed::BoxedBoundAttrDecl::wrap(decl));
        }

        return (match kind {
            "button" => factory::<models::attrs::ButtonModel<V>, V>,
            "fader" => factory::<models::attrs::FaderModel, V>,
            "looper" => factory::<models::attrs::LooperModel<V>, V>,
            "random" => factory::<models::attrs::RandomModel, V>,
            "sequence" => factory::<models::attrs::SequenceModel<V>, V>,
            _ => return Err(format_err!("Unknown bound attribute type: {}", kind))
        })(config, builder);
    }
}

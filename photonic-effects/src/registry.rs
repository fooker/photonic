#![cfg(feature = "dyn")]

use photonic_core::attr::Bounded;
use photonic_core::boxed::{BoxedBoundAttrDecl, BoxedNodeDecl, BoxedUnboundAttrDecl};
use photonic_core::color::RGBColor;
use photonic_dyn::builder::{AttrBuilder, NodeBuilder};
use photonic_dyn::model::AttrValueFactory;
use photonic_dyn::registry::{BoundAttrRegistry, Factory, NodeRegistry, UnboundAttrRegistry};

use super::attrs;
use super::nodes;

pub struct Registry;

impl NodeRegistry for Registry {
    fn manufacture<Builder: NodeBuilder>(kind: &str) -> Option<Box<dyn Factory<BoxedNodeDecl<RGBColor>, Builder>>> {
        return Some(match kind {
            "afterglow" => Factory::node::<nodes::afterglow::model::AfterglowConfig>(),
            "alert" => Factory::node::<nodes::alert::model::AlertConfig>(),
            "blackout" => Factory::node::<nodes::blackout::model::BlackoutConfig>(),
            "brightness" => Factory::node::<nodes::brightness::model::BrightnessConfig>(),
            "colorwheel" => Factory::node::<nodes::colorwheel::ColorwheelNodeDecl>(),
            "distortion" => Factory::node::<nodes::distortion::model::DistortionConfig>(),
            "larson" => Factory::node::<nodes::larson::model::LarsonConfig>(),
            "overlay" => Factory::node::<nodes::overlay::model::OverlayConfig>(),
            "plasma" => Factory::node::<nodes::plasma::model::PlasmaConfig>(),
            "raindrops" => Factory::node::<nodes::raindrops::model::RaindropsConfig>(),
            "rotation" => Factory::node::<nodes::rotation::model::RotationConfig>(),
            "solid" => Factory::node::<nodes::solid::model::SolidConfig>(),
            "switch" => Factory::node::<nodes::switch::model::SwitchConfig>(),
            _ => return None,
        });
    }
}

impl UnboundAttrRegistry for Registry {
    fn manufacture<V: AttrValueFactory, Builder: AttrBuilder>(kind: &str) -> Option<Box<dyn Factory<BoxedUnboundAttrDecl<V>, Builder>>> {
        return Some(match kind {
            "button" => Factory::unbound_attr::<V, attrs::button::model::ButtonModel<V>>(),
            "fader" => Factory::unbound_attr::<V, attrs::fader::model::FaderModel>(),
            "sequence" => Factory::unbound_attr::<V, attrs::sequence::model::SequenceModel<V>>(),
            _ => return None,
        });
    }
}

impl BoundAttrRegistry for Registry {
    fn manufacture<V: AttrValueFactory + Bounded, Builder: AttrBuilder>(kind: &str) -> Option<Box<dyn Factory<BoxedBoundAttrDecl<V>, Builder>>> {
        return Some(match kind {
            "button" => Factory::bound_attr::<V, attrs::button::model::ButtonModel<V>>(),
            "fader" => Factory::bound_attr::<V, attrs::fader::model::FaderModel>(),
            "looper" => Factory::bound_attr::<V, attrs::looper::model::LooperModel<V>>(),
            "random" => Factory::bound_attr::<V, attrs::random::model::RandomModel>(),
            "sequence" => Factory::bound_attr::<V, attrs::sequence::model::SequenceModel<V>>(),
            _ => return None,
        });
    }
}
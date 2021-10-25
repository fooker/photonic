#![cfg(feature = "dyn")]

use photonic_core::attr::Bounded;
use photonic_core::boxed::{BoxedBoundAttrDecl, BoxedNodeDecl, BoxedUnboundAttrDecl};
use photonic_core::color::RGBColor;
use photonic_dyn::builder::{AttrBuilder, NodeBuilder};
use photonic_dyn::model::AttrValueFactory;
use photonic_dyn::registry::{self, BoundAttrRegistry, Factory, NodeRegistry, UnboundAttrRegistry};

use super::{attrs, nodes};

pub struct Registry;

impl NodeRegistry for Registry {
    fn manufacture<Builder: NodeBuilder>(
        kind: &str,
    ) -> Option<Box<dyn Factory<BoxedNodeDecl<RGBColor>, Builder>>> {
        return Some(match kind {
            "afterglow" => registry::node::<Builder, nodes::afterglow::model::AfterglowConfig>(),
            "alert" => registry::node::<Builder, nodes::alert::model::AlertConfig>(),
            "blackout" => registry::node::<Builder, nodes::blackout::model::BlackoutConfig>(),
            "brightness" => registry::node::<Builder, nodes::brightness::model::BrightnessConfig>(),
            "colorwheel" => registry::node::<Builder, nodes::colorwheel::ColorwheelNodeDecl>(),
            "distortion" => registry::node::<Builder, nodes::distortion::model::DistortionConfig>(),
            "larson" => registry::node::<Builder, nodes::larson::model::LarsonConfig>(),
            "overlay" => registry::node::<Builder, nodes::overlay::model::OverlayConfig>(),
            "plasma" => registry::node::<Builder, nodes::plasma::model::PlasmaConfig>(),
            "raindrops" => registry::node::<Builder, nodes::raindrops::model::RaindropsConfig>(),
            "rotation" => registry::node::<Builder, nodes::rotation::model::RotationConfig>(),
            "solid" => registry::node::<Builder, nodes::solid::model::SolidConfig>(),
            "switch" => registry::node::<Builder, nodes::switch::model::SwitchConfig>(),
            _ => return None,
        });
    }
}

impl UnboundAttrRegistry for Registry {
    fn manufacture<V: AttrValueFactory, Builder: AttrBuilder>(
        kind: &str,
    ) -> Option<Box<dyn Factory<BoxedUnboundAttrDecl<V>, Builder>>> {
        return Some(match kind {
            "button" => {
                registry::unbound_attr::<Builder, V, attrs::button::model::ButtonModel<V>>()
            }
            "fader" => registry::unbound_attr::<Builder, V, attrs::fader::model::FaderModel>(),
            "sequence" => {
                registry::unbound_attr::<Builder, V, attrs::sequence::model::SequenceModel<V>>()
            }
            _ => return None,
        });
    }
}

impl BoundAttrRegistry for Registry {
    fn manufacture<V: AttrValueFactory + Bounded, Builder: AttrBuilder>(
        kind: &str,
    ) -> Option<Box<dyn Factory<BoxedBoundAttrDecl<V>, Builder>>> {
        return Some(match kind {
            "button" => registry::bound_attr::<Builder, V, attrs::button::model::ButtonModel<V>>(),
            "fader" => registry::bound_attr::<Builder, V, attrs::fader::model::FaderModel>(),
            "looper" => registry::bound_attr::<Builder, V, attrs::looper::model::LooperModel<V>>(),
            "random" => registry::bound_attr::<Builder, V, attrs::random::model::RandomModel>(),
            "sequence" => {
                registry::bound_attr::<Builder, V, attrs::sequence::model::SequenceModel<V>>()
            }
            _ => return None,
        });
    }
}

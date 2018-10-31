use photonic::attributes::*;
use photonic::attributes::animation::*;
use photonic::attributes::dynamic::*;
use photonic::core::*;
use std::sync::Arc;
use std::time::Duration;
use super::model::*;
use ezing;


pub struct Builder {
    size: usize,
}

impl Builder {
    pub fn with_size(size: usize) -> Self {
        return Self {
            size,
        };
    }

    pub fn build(&mut self, config: &Config) -> Box<Node> {
        // FIXME: Make config more flat / DSL style
        return self.node(&config.node);
    }

    fn easing(&mut self, config: &Option<EasingConfig>) -> Option<Easing> {
        fn linear(t: f64) -> f64 { t } // TODO: https://github.com/michaelfairley/ezing/pull/1

        if let Some(config) = config {
            let func = match config.func {
                EasingFuncConfig::Linear => linear,
                EasingFuncConfig::InQuad => ezing::quad_in,
                EasingFuncConfig::OutQuad => ezing::quad_out,
                EasingFuncConfig::Quad => ezing::quad_inout,
                EasingFuncConfig::InCubic => ezing::cubic_in,
                EasingFuncConfig::OutCubic => ezing::cubic_out,
                EasingFuncConfig::Cubic => ezing::cubic_inout,
                EasingFuncConfig::InQuart => ezing::quart_in,
                EasingFuncConfig::OutQuart => ezing::quart_out,
                EasingFuncConfig::Quart => ezing::quart_inout,
                EasingFuncConfig::InQuint => ezing::quint_in,
                EasingFuncConfig::OutQuint => ezing::quint_out,
                EasingFuncConfig::Quint => ezing::quint_inout,
                EasingFuncConfig::InSine => ezing::sine_in,
                EasingFuncConfig::OutSine => ezing::sine_out,
                EasingFuncConfig::Sine => ezing::sine_inout,
                EasingFuncConfig::InExpo => ezing::expo_in,
                EasingFuncConfig::OutExpo => ezing::expo_out,
                EasingFuncConfig::Expo => ezing::expo_inout,
                EasingFuncConfig::InElastic => ezing::elastic_in,
                EasingFuncConfig::OutElastic => ezing::elastic_out,
                EasingFuncConfig::Elastic => ezing::elastic_inout,
                EasingFuncConfig::InBack => ezing::back_in,
                EasingFuncConfig::OutBack => ezing::back_out,
                EasingFuncConfig::Back => ezing::back_inout,
                EasingFuncConfig::InBounce => ezing::bounce_in,
                EasingFuncConfig::OutBounce => ezing::bounce_out,
                EasingFuncConfig::Bounce => ezing::bounce_inout,
            };

            return Some(Easing {
                func,
                speed: Duration::from_float_secs(config.speed),
            });
        } else {
            return None;
        }
    }

    fn value(&mut self, config: &ValueConfig) -> Attribute {
        let value: Attribute = match config {
            ValueConfig::Fixed(value) => {
                Attribute::new_fixed(*value)
            }

            ValueConfig::Dynamic(config) => {
                let value: DynamicAttribute = match &config.behavior {
                    BehaviorConfig::Fader(behavior) =>
                        DynamicAttribute::Fader(Fader::new(behavior.initial_value,
                                                           self.easing(&behavior.easing))),

                    BehaviorConfig::Button(behavior) =>
                        DynamicAttribute::Button(Button::new(behavior.value_released,
                                                             behavior.value_pressed,
                                                             Duration::from_float_secs(behavior.hold_time),
                                                             self.easing(&behavior.easing_pressed),
                                                             self.easing(&behavior.easing_released))),

                    BehaviorConfig::Sequence(behavior) =>
                        DynamicAttribute::Sequence(Sequence::new(behavior.values.clone(),
                                                                 Duration::from_float_secs(behavior.duration),
                                                                 self.easing(&behavior.easing))),
                };

                Attribute::new_dynamic(&config.name, value)
            }
        };

        return value;
    }

    fn node(&mut self, config: &NodeConfig) -> Box<Node> {
        let node: Box<Node> = match config.config {
            NodeImplConfig::Blackout(ref config) =>
                Box::new(crate::nodes::blackout::BlackoutNode::new(
                    self.node(&config.source),
                    self.value(&config.value),
                    config.range,
                )),

            NodeImplConfig::Colorwheel(ref config) =>
                if let Some(delta) = config.delta {
                    Box::new(crate::nodes::colorwheel::ColorwheelNode::new_delta(
                        config.offset,
                        delta,
                    ))
                } else {
                    Box::new(crate::nodes::colorwheel::ColorwheelNode::new_full(
                        self.size,
                        config.offset,
                    ))
                },

            NodeImplConfig::Rotation(ref config) =>
                Box::new(crate::nodes::rotation::RotationNode::new(
                    self.node(&config.source),
                    self.value(&config.speed),
                    self.size,
                )),

            NodeImplConfig::Raindrops(ref config) =>
                Box::new(crate::nodes::raindrops::RaindropsNode::new(
                    self.size,
                    self.value(&config.rate),
                    self.value(&config.hue.min), self.value(&config.hue.max),
                    self.value(&config.saturation.min), self.value(&config.saturation.max),
                    self.value(&config.lightness.min), self.value(&config.lightness.max),
                    self.value(&config.decay.min), self.value(&config.decay.max),
                )),

            NodeImplConfig::Larson(ref config) =>
                Box::new(crate::nodes::larson::LarsonNode::new(
                    self.size,
                    self.value(&config.hue),
                    self.value(&config.speed),
                    self.value(&config.width),
                )),

            NodeImplConfig::Overlay(ref config) =>
                Box::new(crate::nodes::overlay::OverlayNode::new(
                    self.node(&config.base),
                    self.node(&config.overlay),
                    self.value(&config.blend),
                )),

            NodeImplConfig::Switch(ref config) =>
                Box::new(crate::nodes::switch::SwitchNode::new(
                    config.sources.iter()
                          .map(|config| self.node(&config))
                          .collect(),
                    self.value(&config.position),
                )),
        };

        return node;
    }
}

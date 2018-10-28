use photonic::attributes::*;
use photonic::attributes::animation::*;
use photonic::attributes::dynamic::*;
use photonic::core::*;
use std::sync::Arc;
use std::time::Duration;
use super::model::*;


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
        if let Some(config) = config {
            let func = match config.func {
                EasingFuncConfig::Linear => easing::linear,
                EasingFuncConfig::InQuad => easing::in_quad,
                EasingFuncConfig::OutQuad => easing::out_quad,
                EasingFuncConfig::Quad => easing::quad,
                EasingFuncConfig::InCubic => easing::in_cubic,
                EasingFuncConfig::OutCubic => easing::out_cubic,
                EasingFuncConfig::Cubic => easing::cubic,
                EasingFuncConfig::InQuart => easing::in_quart,
                EasingFuncConfig::OutQuart => easing::out_quart,
                EasingFuncConfig::Quart => easing::quart,
                EasingFuncConfig::InQuint => easing::in_quint,
                EasingFuncConfig::OutQuint => easing::out_quint,
                EasingFuncConfig::Quint => easing::quint,
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

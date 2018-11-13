use ezing;
use photonic::animation::*;
use photonic::core::*;
use photonic::values::*;
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

    pub fn build(&mut self, config: &Config) -> Result<Box<Node>, String> {
        // FIXME: Make config more flat / DSL style
        // Maybe a file per node is a starting point?
        return self.node(&config.node);
    }

    fn easing(&mut self, config: &Option<EasingConfig>) -> Option<Easing> {
        if let Some(config) = config {
            let func = match config.func {
                EasingFuncConfig::Linear => ezing::linear,
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

    fn float_value(&mut self, config: &FloatValueConfig) -> FloatValueFactory {
        match config {
            FloatValueConfig::Fixed(value) => {
                return FloatValue::new_fixed(*value);
            }

            FloatValueConfig::Dynamic(config) => {
                match &config.details {
                    DynamicFloatValueDetailsConfig::Fader(details) => {
                        return FloatValue::new_fader(config.name.to_owned(),
                                                     details.min_value,
                                                     details.max_value,
                                                     self.easing(&details.easing));
                    }

                    DynamicFloatValueDetailsConfig::Button(details) => {
                        return FloatValue::new_button(config.name.to_owned(),
                                                      details.value_released,
                                                      details.value_pressed,
                                                      Duration::from_float_secs(details.hold_time),
                                                      details.auto_trigger.map(Duration::from_float_secs),
                                                      self.easing(&details.easing_pressed),
                                                      self.easing(&details.easing_released));
                    }

                    DynamicFloatValueDetailsConfig::Sequence(details) => {
                        return FloatValue::new_sequence(config.name.to_owned(),
                                                        details.values.clone(),
                                                        details.auto_trigger.map(Duration::from_float_secs),
                                                        self.easing(&details.easing));
                    }

                    DynamicFloatValueDetailsConfig::Random(details) => {
                        return FloatValue::new_random(config.name.to_owned(),
                                                      details.min_value,
                                                      details.max_value,
                                                      details.auto_trigger.map(Duration::from_float_secs),
                                                      self.easing(&details.easing));
                    }
                }
            }
        };
    }

    fn int_value(&mut self, config: &IntValueConfig) -> IntValueFactory {
        match config {
            IntValueConfig::Fixed(value) => {
                return IntValue::new_fixed(*value);
            }

            IntValueConfig::Dynamic(config) => {
                match &config.details {
                    DynamicIntValueDetailsConfig::Manual(details) => {
                        return IntValue::new_manual(config.name.to_owned(),
                                                    details.min_value,
                                                    details.max_value);
                    }

                    DynamicIntValueDetailsConfig::Loop(details) => {
                        return IntValue::new_loop(config.name.to_owned(),
                                                  details.min_value,
                                                  details.max_value,
                                                  details.step.unwrap_or(1),
                                                  details.auto_trigger.map(Duration::from_float_secs));
                    }

                    DynamicIntValueDetailsConfig::Sequence(details) => {
                        return IntValue::new_sequence(config.name.to_owned(),
                                                      details.values.clone(),
                                                      details.auto_trigger.map(Duration::from_float_secs));
                    }

                    DynamicIntValueDetailsConfig::Random(details) => {
                        return IntValue::new_random(config.name.to_owned(),
                                                    details.min_value,
                                                    details.max_value,
                                                    details.auto_trigger.map(Duration::from_float_secs));
                    }
                }
            }
        };
    }

    fn node(&mut self, config: &NodeConfig) -> Result<Box<Node>, String> {
        match config.config {
            NodeImplConfig::Blackout(ref config) => {
                return Ok(Box::new(crate::nodes::blackout::BlackoutNode::new(
                    self.node(&config.source)?,
                    config.range,
                    self.float_value(&config.value),
                )?));
            }

            NodeImplConfig::Colorwheel(ref config) => {
                return Ok(if let Some(delta) = config.delta {
                    Box::new(crate::nodes::colorwheel::ColorwheelNode::new_delta(
                        config.offset,
                        delta,
                    )?)
                } else {
                    Box::new(crate::nodes::colorwheel::ColorwheelNode::new_full(
                        self.size,
                        config.offset,
                    )?)
                });
            }

            NodeImplConfig::Rotation(ref config) => {
                return Ok(Box::new(crate::nodes::rotation::RotationNode::new(
                    self.size,
                    self.node(&config.source)?,
                    self.float_value(&config.speed),
                )?));
            }

            NodeImplConfig::Raindrops(ref config) => {
                return Ok(Box::new(crate::nodes::raindrops::RaindropsNode::new(
                    self.size,
                    self.float_value(&config.rate),
                    self.float_value(&config.hue.min), self.float_value(&config.hue.max),
                    self.float_value(&config.saturation.min), self.float_value(&config.saturation.max),
                    self.float_value(&config.lightness.min), self.float_value(&config.lightness.max),
                    self.float_value(&config.decay.min), self.float_value(&config.decay.max),
                )?));
            }

            NodeImplConfig::Larson(ref config) => {
                return Ok(Box::new(crate::nodes::larson::LarsonNode::new(
                    self.size,
                    self.float_value(&config.hue),
                    self.float_value(&config.speed),
                    self.float_value(&config.width),
                )?));
            }

            NodeImplConfig::Overlay(ref config) => {
                return Ok(Box::new(crate::nodes::overlay::OverlayNode::new(
                    self.node(&config.base)?,
                    self.node(&config.overlay)?,
                    self.float_value(&config.blend),
                )?));
            }

            NodeImplConfig::Switch(ref config) => {
                let sources: Result<Vec<_>, _> = config.sources.iter()
                                                       .map(|config| self.node(&config))
                                                       .collect();

                return Ok(Box::new(crate::nodes::switch::SwitchNode::new(
                    sources?,
                    self.easing(&config.easing),
                    self.int_value(&config.position),
                )?));
            }
        };
    }
}

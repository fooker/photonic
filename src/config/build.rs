use core::*;
use super::model::*;


pub struct Builder {
    size: usize,
}

impl Builder {
    pub fn from_config(config: &Config) -> Box<Node> {
        let mut builder = Builder {
            size: config.size,
        };

        return builder.node(&config.node);
    }

    fn value(&mut self, config: &ValueConfig) -> Box<Value> {
        let value: Box<Value> = match config {
            ValueConfig::Fixed(value) => {
                Box::new(::values::fixed::FixedValue::from(*value))
            }
            ValueConfig::Dynamic(config) => {
                unimplemented!()
            }
        };

        return value;
    }

    fn node(&mut self, config: &NodeConfig) -> Box<Node> {
        let node: Box<Node> = match config.config {
            NodeImplConfig::Blackout(ref config) => {
                Box::new(::nodes::blackout::BlackoutNode::new(
                    self.node(&config.source),
                    self.value(&config.value),
                    config.range,
                ))
            }
            NodeImplConfig::Colorwheel(ref config) => {
                if let Some(delta) = config.delta {
                    Box::new(::nodes::colorwheel::ColorwheelNode::new_delta(
                        config.offset,
                        delta,
                    ))
                } else {
                    Box::new(::nodes::colorwheel::ColorwheelNode::new_full(
                        self.size,
                        config.offset,
                    ))
                }
            }
            NodeImplConfig::Rotation(ref config) => {
                Box::new(::nodes::rotation::RotationNode::new(
                    self.node(&config.source),
                    self.value(&config.speed),
                ))
            }
            NodeImplConfig::Raindrops(ref config) => {
                Box::new(::nodes::raindrops::RaindropsNode::new(
                    self.size,
                    self.value(&config.rate),
                    (self.value(&config.hue.min), self.value(&config.hue.max)),
                    (self.value(&config.saturation.min), self.value(&config.saturation.max)),
                    (self.value(&config.lightness.min), self.value(&config.lightness.max)),
                    (self.value(&config.decay.min), self.value(&config.decay.max)),
                ))
            }
            NodeImplConfig::Larson(ref config) => {
                Box::new(::nodes::larson::LarsonNode::new(
                    self.size,
                    self.value(&config.hue),
                    self.value(&config.speed),
                    self.value(&config.width),
                ))
            }
        };

        return node;
    }
}

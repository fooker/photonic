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
        };

        return node;
    }
}

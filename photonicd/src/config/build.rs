use photonic::attributes::*;
use photonic::attributes::dynamic::*;
use photonic::core::*;
use super::model::*;


pub struct Builder {
    size: usize,
}

impl Builder {
    pub fn from_config(config: &Config) -> Box<Node> {
        let mut builder = Self {
            size: config.size,
        };

        return builder.node(&config.node);
    }

    fn value(&mut self, config: &ValueConfig) -> Attribute {
        let value: Attribute = match config {
            ValueConfig::Fixed(value) => {
                Attribute::from(*value)
            }

            ValueConfig::Dynamic(config) => {
                let attribute: Box<DynamicAttribute> = match &config.behavior {
                    BehaviorConfig::Fader(behavior) =>
                        Box::new(Fader::new(&config.name,
                                            behavior.default_value,
                                            (behavior.min_value, behavior.max_value))),

                    BehaviorConfig::Pushbutton(behavior) =>
                        Box::new(Pushbutton::new(&config.name,
                                                 behavior.released_value,
                                                 behavior.pressed_value,
                                                 behavior.hold_time)),

                    BehaviorConfig::Timer(config) => unimplemented!(),
                };

                Attribute::from(attribute)
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
        };

        return node;
    }
}

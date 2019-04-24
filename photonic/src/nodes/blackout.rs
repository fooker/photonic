use std::time::Duration;

use crate::animation::Animation;
use crate::animation::Easing;
use crate::color::Black;
use crate::core::*;
use crate::math::Lerp;
use crate::values::*;
use failure::Error;

struct PartialBlackoutRenderer<'a> {
    source: Box<Renderer + 'a>,

    range: (usize, usize),
    value: f64,
}

impl<'a> Renderer for PartialBlackoutRenderer<'a> {
    fn get(&self, index: usize) -> MainColor {
        let value = self.source.get(index);

        if self.range.0 < index && index < self.range.1 {
            return MainColor::lerp(value,
                                   MainColor::black(),
                                   self.value);
        } else {
            return value;
        }
    }
}

struct FullBlackoutRenderer<'a> {
    source: Box<Renderer + 'a>,

    value: f64,
}

impl<'a> Renderer for FullBlackoutRenderer<'a> {
    fn get(&self, index: usize) -> MainColor {
        return MainColor::lerp(self.source.get(index),
                               MainColor::black(),
                               self.value);
    }
}

//enum AnimatedFloatValue {
//    Hard(FloatValue),
//    Soft {
//        value: FloatValue,
//        easing: Easing,
//        transition: Animation,
//    },
//}
//
//impl AnimatedFloatValue {
//    pub fn wrap(value: FloatValue, easing: Option<Easing>) -> Self {
//        return match easing {
//            None => AnimatedFloatValue::Hard(value),
//            Some(easing) => AnimatedFloatValue::Soft {
//                value,
//                easing,
//                transition: Animation::Idle,
//
//            }
//        };
//    }
//
//    #[inline]
//    pub fn value(&self) -> &FloatValue {
//        return match self {
//            AnimatedFloatValue::Hard(value) => value,
//            AnimatedFloatValue::Soft{value, easing, transition} => value,
//        };
//    }
//
//    #[inline]
//    pub fn get(&self) -> f64 {
//        self.value().get()
//    }
//
//    #[inline]
//    pub fn update(&mut self, duration: &Duration) -> Option<f64> {
//        match self {
//            AnimatedFloatValue::Hard(value) => {
//                return value.update(duration)
//            }
//            AnimatedFloatValue::Soft{value, easing, transition} => {
//                let new = value.update(duration);
//
//                // TODO: Start new animation?
//
//                return new;
//            }
//        }
//    }
//}

pub struct BlackoutNodeDecl<Source: Node> {
    pub source: Source,
    pub value: Box<BoundValueDecl<f64>>,
    pub range: Option<(usize, usize)>,
}

pub struct BlackoutNode<Source: Node> {
    source: Source,

    value: Box<Value<f64>>,

    range: Option<(usize, usize)>,

//    easing: Option<Easing>,
//    transition: Animation,
}

impl <Source: Node> NodeDecl for BlackoutNodeDecl<Source> {
    type Node = BlackoutNode<Source>;

    fn new(self, size: usize) -> Result<Self::Node, Error> {
        return Ok(Self::Node {
            source: self.source,
            value: self.value.new(Bounds::norm())?,
            range: self.range,
        });
    }
}

impl <Source: Node> Node for BlackoutNode<Source> {
    const TYPE: &'static str = "blackout";

    fn update(&mut self, duration: &Duration) {
        self.source.update(duration);

//        if Some(value) = self.value.update(duration) {
//            self.transition = Animation::start(self.easing, 0.0, value);
//        }
    }

    fn render<'a>(&'a self) -> Box<Renderer + 'a> {
        if let Some(range) = self.range {
            return Box::new(PartialBlackoutRenderer {
                source: self.source.render(),
                value: self.value.get(),
                range,
            });
        } else {
            return Box::new(FullBlackoutRenderer {
                source: self.source.render(),
                value: self.value.get(),
            });
        }
    }
}

//impl Inspection for BlackoutNode {
//    fn children(&self) -> Vec<NodeRef> {
//        vec![
//            NodeRef { name: "source", ptr: self.source.as_ref() },
//        ]
//    }
//
//    fn values(&self) -> Vec<ValueRef> {
//        vec![
//            ValueRef { name: "value", ptr: ValuePtr::Float(self.value.value()), }
//        ]
//    }
//}

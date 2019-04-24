use crate::animation::*;
use crate::core::*;
use crate::math;
use crate::trigger::*;
use crate::utils;
use rand::prelude::{FromEntropy, Rng, SmallRng};
use std::boxed::FnBox;
use std::cell::Cell;
use std::cell::RefCell;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::time::Duration;
use super::{ValueDecl, ValueFactory, Value};
use crate::values::DynamicValue;

pub mod fader;
pub mod sequence;
pub mod random;
pub mod button;


impl Value<f64> {
//    pub fn new_fixed(value: f64) -> ValueFactory<f64> {
//        // FIXME: Check bounds - having some kind of scene error?
//        Box::new(move |_| Ok(Value::Fixed(value)))
//    }
//
//    fn new_dynamic(name: String, value: impl DynamicValue<f64> + 'static) -> Self {
//        Value::Dynamic {
//            name,
//            value: Box::new(value),
//        }
//    }
//
//    pub fn new_fader(name: Option<String>,
//                     min: Option<f64>,
//                     max: Option<f64>) -> ValueFactory<f64> {
//        Box::new(move |decl: ValueDecl<f64>| {
//            let name = name.unwrap_or_else(|| decl.name.to_owned());
//
//            let min = utils::combine_opts(decl.min, min, f64::max);
//            let max = utils::combine_opts(decl.max, max, f64::min);
//
//            return Ok(Self::new_dynamic(name, fader::Fader::new(
//                min,
//                max,
//            )));
//        })
//    }
//
//    pub fn new_button(name: Option<String>,
//                      value_released: Option<f64>,
//                      value_pressed: Option<f64>,
//                      hold_time: Duration,
//                      auto_trigger: Option<Duration>) -> ValueFactory<f64> {
//        Box::new(move |decl: ValueDecl<f64>| {
//            let name = name.unwrap_or_else(|| decl.name.to_owned());
//
//            let value_released = value_released.map(|v| math::clamp_opt(v, (decl.min, decl.max))).or(decl.min);
//            let value_pressed = value_pressed.map(|v| math::clamp_opt(v, (decl.min, decl.max))).or(decl.max);
//
//            return Ok(Self::new_dynamic(name, button::Button::new(
//                value_released.ok_or("value_released is required".to_string())?,
//                value_pressed.ok_or("value_pressed is required".to_string())?,
//                hold_time,
//                auto_trigger,
//            )));
//        })
//    }
//
//    pub fn new_sequence(name: Option<String>,
//                        values: Vec<f64>,
//                        auto_trigger: Option<Duration>) -> ValueFactory<f64> {
//        Box::new(move |decl: ValueDecl<f64>| {
//            let name = name.unwrap_or_else(|| decl.name.to_owned());
//
//            let values = values.iter()
//                               .map(|v| math::clamp_opt(*v, (decl.min, decl.max)))
//                               .collect();
//
//            return Ok(Self::new_dynamic(name, sequence::Sequence::new(
//                values,
//                auto_trigger,
//            )));
//        })
//    }
//
//    pub fn new_random(name: Option<String>,
//                      min: Option<f64>,
//                      max: Option<f64>,
//                      auto_trigger: Option<Duration>) -> ValueFactory<f64> {
//        Box::new(move |decl: ValueDecl<f64>| {
//            let name = name.unwrap_or_else(|| decl.name.to_owned());
//
//            let min = utils::combine_opts(decl.min, min, f64::max);
//            let max = utils::combine_opts(decl.max, max, f64::min);
//
//            return Ok(Self::new_dynamic(name, random::Random::new(
//                min.ok_or("min is required".to_string())?,
//                max.ok_or("max is required".to_string())?,
//                auto_trigger,
//            )));
//        })
//    }
}

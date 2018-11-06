use crate::values::float::Value;
use crate::color::RGBColor;
pub use crate::inspection::{ValueRef, Inspection, NodeRef};
use crate::math::{self, Lerp};
use std::ops::{Deref, DerefMut};
use std::time::Duration;

pub type MainColor = RGBColor;

pub trait Dynamic {
    fn update(&mut self, duration: &Duration);
}

pub trait Renderer {
    fn get(&self, index: usize) -> MainColor;
}

pub trait Source {
    fn render<'a>(&'a self) -> Box<Renderer + 'a>;
}

pub trait Node: Dynamic + Source + Inspection {
    fn class(&self) -> &'static str;
}

pub trait Output {
    fn render(&mut self, renderer: &Renderer);
}

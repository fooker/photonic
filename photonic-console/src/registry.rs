#![cfg(feature = "dyn")]

use photonic_core::boxed::{BoxedNodeDecl, BoxedOutputDecl};
use photonic_core::element::RGBColor;
use photonic_dyn::builder::OutputBuilder;
use photonic_dyn::registry::{self, Factory, OutputRegistry};

use crate::ConsoleOutputDecl;

pub struct Registry;

impl OutputRegistry for Registry {
    fn manufacture<Builder: OutputBuilder>(
        kind: &str,
    ) -> Option<Box<dyn Factory<BoxedOutputDecl<BoxedNodeDecl<RGBColor>>, Builder>>> {
        return Some(match kind {
            "console" => registry::output::<Builder, ConsoleOutputDecl>(),
            _ => return None,
        });
    }
}

#![feature(min_specialization)]

pub use serde;

pub use photonic_dynamic_registry::{Factory, Registry};

pub use crate::boxed::{BoxedBoundAttrDecl, BoxedFreeAttrDecl, BoxedNodeDecl, BoxedOutputDecl};
pub use crate::builder::{AttrBuilder, Builder, NodeBuilder, OutputBuilder};

pub mod boxed;
pub mod builder;
pub mod config;

pub type NodeFactory<B> = dyn Factory<BoxedNodeDecl, B>;
pub type FreeAttrFactory<V, B> = dyn Factory<BoxedFreeAttrDecl<V>, B>;
pub type BoundAttrFactory<V, B> = dyn Factory<BoxedBoundAttrDecl<V>, B>;
pub type OutputFactory<B> = dyn Factory<BoxedOutputDecl, B>;

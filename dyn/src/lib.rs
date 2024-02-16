pub use serde;

use photonic::NodeDecl;
pub use photonic_app_derive::DynamicNode;

use crate::builder::NodeBuilder;
use crate::registry::NodeFactory;

pub mod boxed;
pub mod builder;
pub mod config;
pub mod registry;

pub mod dynamic;

pub trait DynamicNode: NodeDecl {
    const KIND: &'static str;

    fn factory<Builder>() -> NodeFactory<Builder>
    where Builder: NodeBuilder;
}

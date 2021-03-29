#![feature(never_type)]
#![feature(unsize)]
#![feature(trait_alias)]

pub mod scene;
pub mod node;
pub mod output;
pub mod buffer;
pub mod color;
pub mod math;
pub mod utils;
pub mod attr;
pub mod animation;
pub mod input;
pub mod interface;
pub mod timer;

#[cfg(feature = "boxed")]
pub mod boxed;

pub use scene::{Scene,Loop,NodeHandle,InputHandle};
pub use node::{Node, NodeDecl};
pub use output::{Output, OutputDecl};
pub use buffer::Buffer;
pub use attr::{UnboundAttrDecl, BoundAttrDecl, Attr};
pub use input::Input;
pub use interface::Registry;
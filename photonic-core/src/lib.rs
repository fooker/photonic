#![allow(clippy::needless_return)]

pub mod animation;
pub mod attr;
pub mod buffer;
pub mod color;
pub mod input;
pub mod interface;
pub mod math;
pub mod node;
pub mod output;
pub mod scene;
pub mod timer;
pub mod utils;

#[cfg(feature = "boxed")]
pub mod boxed;

pub use attr::{Attr, BoundAttrDecl, UnboundAttrDecl};
pub use buffer::Buffer;
pub use input::Input;
pub use interface::Introspection;
pub use node::{Node, NodeDecl};
pub use output::{Output, OutputDecl};
pub use scene::{InputHandle, Loop, NodeHandle, Scene};

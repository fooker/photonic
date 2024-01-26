#![feature(unsize)]
#![feature(coerce_unsized)]
extern crate anyhow;

pub use attr::{Attr, AttrValue};
pub use buffer::{Buffer, BufferReader};
pub use color::rgbw::{Rgbw, WhiteMode, WithWhite};
pub use decl::{BoundAttrDecl, FreeAttrDecl, NodeDecl, OutputDecl};
pub use interface::{AttrInfo, InputInfo, NodeInfo};
pub use node::Node;
pub use output::Output;
pub use random::Random;
pub use scene::{AttrBuilder, Context, Loop, NodeBuilder, NodeHandle, NodeRef, Scene, SceneBuilder};

pub mod node;
pub mod output;
pub mod scene;
pub mod buffer;
pub mod interface;
pub mod decl;
pub mod utils;
pub mod attr;
pub mod input;
mod arena;

pub mod math;

pub mod random;

pub mod color;


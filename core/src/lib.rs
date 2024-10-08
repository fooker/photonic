#![feature(unsize)]
#![feature(coerce_unsized)]
#![feature(impl_trait_in_assoc_type)]
#![feature(map_try_insert)]
#![feature(trivial_bounds)]
#![feature(never_type)]

pub use attr::{Attr, AttrValue};
pub use buffer::{Buffer, BufferReader};
pub use color::rgbw::{Rgbw, WhiteMode, WithWhite};
pub use decl::{BoundAttrDecl, FreeAttrDecl, NodeDecl, OutputDecl};
pub use interface::{AttrInfo, InputInfo, NodeInfo};
pub use node::Node;
pub use output::Output;
pub use random::Random;
pub use scene::{AttrBuilder, Loop, NodeBuilder, NodeHandle, NodeRef, RenderContext, Scene, SceneBuilder};

mod arena;
pub mod attr;
pub mod buffer;
pub mod color;
pub mod decl;
pub mod input;
pub mod interface;
pub mod math;
pub mod node;
pub mod output;
pub mod random;
pub mod scene;
pub mod utils;

#[cfg(feature = "boxed")]
pub mod boxed;

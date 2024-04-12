#![feature(unsize)]
#![feature(coerce_unsized)]
#![feature(impl_trait_in_assoc_type)]
#![feature(map_try_insert)]
#![deny(
    bad_style,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    trivial_casts,
    trivial_numeric_casts,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_extern_crates,
    unused_import_braces,
    unused_parens,
    unused_qualifications,
    unused_results,
    while_true
)]

pub use attr::{Attr, AttrValue};
pub use buffer::{Buffer, BufferReader};
pub use color::rgbw::{Rgbw, WhiteMode, WithWhite};
pub use decl::{BoundAttrDecl, FreeAttrDecl, NodeDecl, OutputDecl};
pub use interface::{AttrInfo, InputInfo, NodeInfo};
pub use node::Node;
pub use output::Output;
pub use random::Random;
pub use scene::{AttrBuilder, Context, Loop, NodeBuilder, NodeHandle, NodeRef, Scene, SceneBuilder};

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

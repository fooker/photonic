#![feature(never_type)]

pub use attr::{BoxedBoundAttrDecl, BoxedFreeAttrDecl};
pub use node::BoxedNodeDecl;
pub use output::BoxedOutputDecl;

mod attr;
mod node;
mod output;

pub use self::attr::{DynBoundAttrDecl, DynFreeAttrDecl};
pub use self::node::DynNodeDecl;
pub use self::output::DynOutputDecl;

pub trait Boxed<T: ?Sized> {
    fn boxed(self) -> Box<T>;
}

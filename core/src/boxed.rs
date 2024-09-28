pub use self::attr::{BoxedAttr, BoxedBoundAttrDecl, BoxedFreeAttrDecl, DynBoundAttrDecl, DynFreeAttrDecl};
pub use self::node::{BoxedNode, BoxedNodeDecl, DynNodeDecl};
pub use self::output::{BoxedOutput, BoxedOutputDecl, DynOutputDecl};

mod attr;
mod node;
mod output;

pub trait Boxed<T: ?Sized> {
    fn boxed(self) -> Box<T>;
}

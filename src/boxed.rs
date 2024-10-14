pub use self::attr::{BoxedAttr, BoxedBoundAttrDecl, BoxedFreeAttrDecl, DynAttr, DynBoundAttrDecl, DynFreeAttrDecl};
pub use self::node::{BoxedNode, BoxedNodeDecl, DynNode, DynNodeDecl};
pub use self::output::{BoxedOutput, BoxedOutputDecl, DynOutput, DynOutputDecl};

mod attr;
mod node;
mod output;

pub trait Boxed<T: ?Sized> {
    fn boxed(self) -> Box<T>;
}

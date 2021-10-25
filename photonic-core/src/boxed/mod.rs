pub mod attr;
pub mod node;
pub mod output;

pub use attr::{BoxedAttr, BoxedBoundAttrDecl, BoxedUnboundAttrDecl};
pub use node::{BoxedNode, BoxedNodeDecl};
pub use output::{BoxedOutput, BoxedOutputDecl};

pub trait Wrap<T> {
    fn wrap(t: T) -> Self;
}

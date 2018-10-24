use crate::attributes::Attribute;
use crate::core::Node;
use std::ops::{Deref, DerefMut};
use std::error::Error;

pub trait Inspection {
    fn children(&self) -> Vec<NodeRef>;
    fn attributes(&self) -> Vec<AttributeRef>;
}

pub fn recurse_attributes<F>(node: &Node, f: &mut F)
    where F: FnMut(&Attribute) {
    for attr in node.attributes() {
        f(&attr);
    }

    for node in node.children() {
        recurse_attributes(node.as_ref(), f);
    }
}

#[derive(Clone)]
pub struct AttributeRef<'n> {
    pub name: &'static str,
    pub ptr: &'n Attribute,
}

impl<'n> Deref for AttributeRef<'n> {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        self.ptr
    }
}

impl<'n> AsRef<Attribute> for AttributeRef<'n> {
    fn as_ref(&self) -> &Attribute {
        self.ptr
    }
}

#[derive(Clone)]
pub struct NodeRef<'n> {
    pub name: &'static str,
    pub ptr: &'n Node,
}

impl<'n> Deref for NodeRef<'n> {
    type Target = (Node + 'n);

    fn deref(&self) -> &Self::Target {
        self.ptr
    }
}

impl<'n> AsRef<Node + 'n> for NodeRef<'n> {
    fn as_ref(&self) -> &(Node + 'n) {
        self.ptr
    }
}

use crate::attributes::Attribute;
use crate::core::Node;
use std::ops::{Deref,DerefMut};

pub trait Inspection {
    fn class(&self) -> &'static str;

    fn children(&self) -> Vec<NodeRef>;
    fn attributes(&self) -> Vec<AttributeRef>;
}


#[derive(Clone)]
pub struct AttributeRef<'n> {
    pub name: &'static str,
    pub ptr: &'n Attribute,
}

impl <'n> Deref for AttributeRef<'n> {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        self.ptr
    }
}

impl <'n> AsRef<Attribute> for AttributeRef<'n> {
    fn as_ref(&self) -> &Attribute {
        self.ptr
    }
}

#[derive(Clone)]
pub struct NodeRef<'n> {
    pub name: &'static str,
    pub ptr: &'n Node,
}

impl <'n> Deref for NodeRef<'n> {
    type Target = (Node + 'n);

    fn deref(&self) -> &Self::Target {
        self.ptr
    }
}

impl <'n> AsRef<Node + 'n> for NodeRef<'n> {
    fn as_ref(&self) -> &(Node + 'n) {
        self.ptr
    }
}

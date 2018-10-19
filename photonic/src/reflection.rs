use crate::attributes::Attribute;
use crate::core::Node;
use std::ops::{Deref,DerefMut};

pub struct AttributeRef<'n> {
    pub ptr: &'n Attribute,
    pub name: &'n str,
}

impl <'n> Deref for AttributeRef<'n> {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        self.ptr
    }
}

pub struct NodeRef<'n> {
    pub ptr: &'n Node,
    pub name: &'n str,
}

impl <'n> Deref for NodeRef<'n> {
    type Target = (Node + 'n);

    fn deref(&self) -> &Self::Target {
        self.ptr
    }
}

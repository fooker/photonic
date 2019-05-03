use crate::values::{FloatValue,IntValue};
use crate::core::Node;
use std::ops::{Deref, DerefMut};
use std::error::Error;

pub trait Inspection {
    fn children(&self) -> Vec<NodeRef>;
    fn values(&self) -> Vec<ValueRef>;
}

/// Visits all values of the node and its children recursively
pub fn visit_values<F>(node: &Node, f: &mut F)
    where F: FnMut(&ValuePtr) {
    for value in node.values() {
        f(&value.ptr);
    }

    for node in node.children() {
        visit_values(node.as_ref(), f);
    }
}

#[derive(Clone)]
pub enum ValuePtr<'n> {
    Float(&'n FloatValue),
    Int(&'n IntValue),
}

#[derive(Clone)]
pub struct ValueRef<'n> {
    pub name: &'static str,
    pub ptr: ValuePtr<'n>,
}

//impl<'n> Deref for ValueRef<'n> {
//    type Target = Value;
//
//    fn deref(&self) -> &Self::Target {
//        self.ptr
//    }
//}
//
//impl<'n> AsRef<Value> for ValueRef<'n> {
//    fn as_ref(&self) -> &Value {
//        self.ptr
//    }
//}

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

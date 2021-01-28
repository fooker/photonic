use std::sync::Arc;

use crate::attr::{Attr, AttrValue, Bounded, Bounds, AttrType};
use crate::core::{Node, NodeDecl, NodeRef, NodeHandle, AttrPath};
use crate::input::{Input, InputValue, InputType};
use std::collections::HashMap;

pub struct InputInfo {
    name: String,
    kind: &'static str,

    value_type: InputType,
}

pub struct AttrInfo {
    pub name: String,
    pub kind: &'static str,

    pub value_type: AttrType,

    pub nested: Vec<Arc<AttrInfo>>,
    pub inputs: Vec<Arc<InputInfo>>,
}

pub struct NodeInfo {
    pub name: String,
    pub kind: &'static str,

    pub alias: String,

    pub attrs: Vec<Arc<AttrInfo>>,
}

pub struct Registry {
    nodes_by_alias: HashMap<String, Arc<NodeInfo>>,
}

impl Registry {
    pub fn from(nodes: Vec<Arc<NodeInfo>>) -> Self {
        return Self {
            nodes_by_alias: nodes.iter().cloned().map(|node| (node.alias.clone(), node)).collect(),
        };
    }
}

pub trait Interface {}
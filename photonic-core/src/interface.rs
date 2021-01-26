use std::sync::Arc;

use crate::attr::{Attr, AttrValue, Bounded, Bounds};
use crate::core::{Node, NodeHandle};
use crate::input::{Input, InputValue};

pub enum InputType {
    Trigger,
    Integer,
    Decimal,
}

pub struct InputInfo {
    name: String,
    kind: InputType,
}

pub enum ValueType {
    Integer,
    Decimal,
}

pub struct AttrInfo {
    name: String,
    kind: ValueType,

    nested: Vec<Arc<AttrInfo>>,
    inputs: Vec<Arc<InputInfo>>,
}

pub struct NodeInfo {
    name: String,
    kind: &'static str,

    attrs: Vec<Arc<AttrInfo>>,
}

pub struct Registry {
    nodes: Vec<Arc<NodeInfo>>,
}

impl Registry {
    pub fn new() -> Self {
        return Self {
            nodes: Vec::new(),
        };
    }

    pub fn register_node<Node>(&mut self, node: &NodeHandle<Node>)
        where Node: self::Node {
        self.nodes.push(Arc::new(NodeInfo {
            name: node.name.clone(),
            kind: Node::TYPE,
            attrs: Vec::new(),
        }));
    }

    pub fn register_attr<Attr, V>(&mut self, attr: &Attr, bounds: Option<Bounds<V>>)
        where Attr: self::Attr<V>,
              V: AttrValue {

    }

    // pub fn register_input<Input, V>(&mut self, input: &Input, bounds: Option<Bounds<V>>)
    //     where Input: self::Input<V>,
    //           V: InputValue {
    //
    // }
}

pub trait Interface {}
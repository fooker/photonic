use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use failure::Error;

use crate::attr::{Attr, AttrValue, AttrValueType, Bounded, Bounds};
use crate::core::{AttrPath, Node, NodeDecl, NodeHandle, NodeRef};
use crate::input::{Input, InputValue, InputValueType};
use crate::utils::TreeIterator;

pub struct InputInfo {
    pub name: String,
    pub kind: &'static str,

    pub value_type: InputValueType,
}

impl InputInfo {
    pub fn walk<V: Visitor>(&self, mut visitor: V) -> V {
        let visitor = visitor.input(self);
        return visitor;
    }
}

pub struct AttrInfo {
    pub kind: &'static str,

    pub value_type: AttrValueType,

    pub attrs: HashMap<String, Arc<AttrInfo>>,
    pub inputs: HashMap<String, Arc<InputInfo>>,
}

impl AttrInfo {
    pub fn walk<V: Visitor>(&self, mut visitor: V) -> V {
        let visitor = visitor.attr(self);
        let visitor = self.attrs.values().fold(visitor, |v, attr| attr.walk(v));
        let visitor = self.inputs.values().fold(visitor, |v, input| input.walk(v));
        return visitor;
    }

    pub fn iter<'s>(self: &'s Arc<Self>) -> impl Iterator<Item=&'s Arc<Self>> + 's {
        return TreeIterator::new(self, |node| node.attrs.values());
    }
}

pub struct NodeInfo {
    pub name: String,
    pub kind: &'static str,

    pub nodes: HashMap<String, Arc<NodeInfo>>,
    pub attrs: HashMap<String, Arc<AttrInfo>>,
}

impl NodeInfo {
    // pub fn walk<V: Visitor>(&self, mut visitor: V) -> V {
    //     let visitor = visitor.node(self);
    //     let visitor = self.nodes.values().fold(visitor, |v, node| node.walk(v));
    //     let visitor = self.attrs.values().fold(visitor, |v, attr| attr.walk(v));
    //     return visitor;
    // }

    pub fn iter<'s>(self: &'s Arc<Self>) -> impl Iterator<Item=&'s Arc<Self>> + 's {
        return TreeIterator::new(self, |node| node.nodes.values());
    }
}

pub trait Visitor {
    fn node(self, node: &NodeInfo) -> Self;
    fn attr(self, attr: &AttrInfo) -> Self;
    fn input(self, input: &InputInfo) -> Self;
}

pub struct Registry {
    pub root: Arc<NodeInfo>,

    pub nodes: HashMap<String, Arc<NodeInfo>>,
    pub inputs: HashMap<String, Arc<InputInfo>>,
}

impl Registry {
    pub fn from(root: Arc<NodeInfo>) -> Arc<Self> {
        let nodes = root.iter()
            .map(|node| (node.name.clone(), node.clone()))
            .collect();

        let inputs = root.iter()
            .flat_map(|node| node.attrs.values())
            .flat_map(|attr| attr.iter())
            .flat_map(|attr| attr.inputs.values())
            .map(|input| (input.name.clone(), input.clone()))
            .collect();

        return Arc::new(Self {
            root,
            nodes,
            inputs,
        });
    }

    pub fn serve<I>(self: Arc<Self>, iface: I) -> Result<(), Error>
        where I: Interface + 'static {
        tokio::spawn(iface.listen(self));
        return Ok(());
    }
}

#[async_trait]
pub trait Interface {
    async fn listen(self, registry: Arc<Registry>) -> Result<(), Error>;
}
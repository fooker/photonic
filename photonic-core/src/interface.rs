use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;


use failure::Error;

use crate::attr::{Attr, AttrValueType, AttrValue, Bounded, Bounds};
use crate::core::{AttrPath, Node, NodeDecl, NodeHandle, NodeRef};
use crate::input::{Input, InputValueType, InputValue};

pub struct InputInfo {
    pub name: String,
    pub kind: &'static str,

    pub alias: String,

    pub value_type: InputValueType,
}

pub struct AttrInfo {
    pub name: String,
    pub kind: &'static str,

    pub value_type: AttrValueType,

    pub nested: Vec<Arc<AttrInfo>>,
    pub inputs: Vec<Arc<InputInfo>>,
}

pub struct NodeInfo {
    pub name: String,
    pub kind: &'static str,

    pub alias: String,

    pub nested: HashMap<String, Arc<NodeInfo>>,
    pub attrs: Vec<Arc<AttrInfo>>,
}

pub struct Registry {
    nodes: HashMap<String, Arc<NodeInfo>>,
}

impl Registry {
    pub fn from(nodes: Vec<Arc<NodeInfo>>) -> Arc<Self> {
        let nodes = nodes.iter()
            .map(|node| (node.alias.clone(), node.clone()))
            .collect();

        return Arc::new(Self {
            nodes,
        });
    }

    pub fn serve<I>(self: Arc<Self>, iface: I) -> Result<(), Error>
        where I: Interface + 'static {
        std::thread::spawn(move || {
            iface.listen(self)
        });

        return Ok(());
    }
}

#[async_trait]
pub trait Interface: Send {
    async fn listen(self, registry: Arc<Registry>) -> Result<(), Error>;
}
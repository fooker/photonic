use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;

use crate::attr::AttrValueType;
use crate::input::{InputSender, InputValueType};
use crate::utils::TreeIterator;

#[derive(Debug)]
pub struct NodeInfo {
    pub name: String,
    pub kind: &'static str,

    pub nodes: HashMap<String, Arc<NodeInfo>>,
    pub attrs: HashMap<String, Arc<AttrInfo>>,
}

impl NodeInfo {
    #[allow(clippy::needless_lifetimes)]
    pub fn iter<'s>(self: &'s Arc<Self>) -> impl Iterator<Item = &'s Arc<Self>> + 's {
        return TreeIterator::new(self, |node| node.nodes.values());
    }
}

#[derive(Debug)]
pub struct AttrInfo {
    pub kind: &'static str,

    pub value_type: AttrValueType,

    pub attrs: HashMap<String, Arc<AttrInfo>>,
    pub inputs: HashMap<String, Arc<InputInfo>>,
}

impl AttrInfo {
    #[allow(clippy::needless_lifetimes)]
    pub fn iter<'s>(self: &'s Arc<Self>) -> impl Iterator<Item = &'s Arc<Self>> + 's {
        return TreeIterator::new(self, |node| node.attrs.values());
    }
}

#[derive(Debug)]
pub struct InputInfo {
    pub name: String,
    pub value_type: InputValueType,

    pub sender: InputSender,
}

pub struct Introspection {
    pub root: Arc<NodeInfo>,

    pub nodes: HashMap<String, Arc<NodeInfo>>,
    pub inputs: HashMap<String, Arc<InputInfo>>,
}

impl Introspection {
    pub fn with(root: Arc<NodeInfo>) -> Arc<Self> {
        let nodes = root.iter().map(|node| (node.name.clone(), node.clone())).collect();

        let inputs = root
            .iter()
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

    pub fn serve<I>(self: Arc<Self>, iface: I) -> Result<()>
    where
        I: Interface + 'static,
    {
        tokio::spawn(iface.listen(self));
        return Ok(());
    }
}

#[async_trait]
pub trait Interface {
    async fn listen(self, introspection: Arc<Introspection>) -> Result<()>;
}

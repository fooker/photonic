use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::fmt;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;
use parking_lot::Mutex;
use tonic::transport::Channel;

use photonic_interface_grpc_proto::interface_client::InterfaceClient;
use photonic_interface_grpc_proto::{AttrInfoRequest, AttrName, NodeInfoRequest, NodeInfoResponse};

use crate::attr::Attr;
use crate::AttrId;

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct NodeId(pub(crate) String);

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl fmt::Debug for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for NodeId {
    fn as_ref(&self) -> &str {
        return &self.0;
    }
}

impl FromStr for NodeId {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(Self(s.to_owned()));
    }
}

pub struct Node {
    client: Arc<Mutex<InterfaceClient<Channel>>>,

    name: NodeId,
    kind: String,

    nodes: HashMap<String, NodeId>,
    attrs: HashSet<AttrId>,
}

impl Node {
    pub(crate) fn from_node_info(client: Arc<Mutex<InterfaceClient<Channel>>>, info: NodeInfoResponse) -> Self {
        let name = NodeId(info.name);

        let nodes = info.nodes.into_iter().map(|(key, node)| (key, NodeId(node))).collect();

        let attrs = info
            .attrs
            .into_iter()
            .map(|attr| AttrId {
                node: name.clone(),
                path: vec![attr],
            })
            .collect();

        Self {
            client,
            name,
            kind: info.kind,
            nodes,
            attrs,
        }
    }

    pub fn name(&self) -> &NodeId {
        return &self.name;
    }

    pub fn kind(&self) -> &str {
        return &self.kind;
    }

    pub fn nodes(&self) -> &HashMap<String, NodeId> {
        return &self.nodes;
    }

    pub fn attrs(&self) -> &HashSet<AttrId> {
        return &self.attrs;
    }

    pub async fn node(&self, name: &str) -> Result<Option<Node>> {
        let mut client = self.client.lock_arc();

        let Some(node) = self.nodes.get(name) else {
            return Ok(None);
        };

        let response = client
            .node(NodeInfoRequest {
                name: node.0.clone(),
            })
            .await?
            .into_inner();

        return Ok(Some(Node::from_node_info(self.client.clone(), response)));
    }

    pub async fn attr(&self, name: &str) -> Result<Option<Attr>> {
        let mut client = self.client.lock_arc();

        let response = client
            .attr(AttrInfoRequest {
                name: Some(AttrName {
                    node: self.name.0.clone(),
                    path: vec![name.to_owned()],
                }),
            })
            .await?
            .into_inner();

        return Ok(Some(Attr::from_attr_info(self.client.clone(), response)));
    }
}

use std::collections::HashMap;
use std::convert::Infallible;
use std::fmt;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;
use parking_lot::Mutex;
use tonic::transport::Channel;

use photonic_interface_grpc_proto::interface_client::InterfaceClient;
use photonic_interface_grpc_proto::{NodeInfoRequest, NodeInfoResponse};

use crate::attr::{Attribute, AttributeId, AttributeRef};
use crate::{Identified, Ref};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct NodeId(pub(crate) String);

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for NodeId {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(Self(s.to_owned()));
    }
}

pub struct Node {
    #[allow(dead_code)]
    client: Arc<Mutex<InterfaceClient<Channel>>>,

    name: NodeId,
    kind: String,

    nodes: HashMap<String, NodeRef>,
    attrs: HashMap<String, AttributeRef>,
}

impl Identified for Node {
    type Id = NodeId;

    fn name(&self) -> &Self::Id {
        return &self.name;
    }
}

impl Node {
    pub(crate) fn from_node_info(client: Arc<Mutex<InterfaceClient<Channel>>>, info: NodeInfoResponse) -> Self {
        let name = NodeId(info.name);

        let nodes = info
            .nodes
            .into_iter()
            .map(|(k, v)| {
                (k, NodeRef {
                    client: client.clone(),
                    name: NodeId(v),
                })
            })
            .collect();

        let attrs = info
            .attrs
            .into_iter()
            .map(|k| {
                (k.clone(), AttributeRef {
                    client: client.clone(),
                    name: AttributeId {
                        node: name.clone(),
                        path: vec![k],
                    },
                })
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

    pub fn kind(&self) -> &str {
        return &self.kind;
    }

    pub fn nodes(&self) -> &HashMap<String, impl Ref<Node>> {
        return &self.nodes;
    }

    pub fn attributes(&self) -> &HashMap<String, impl Ref<Attribute>> {
        return &self.attrs;
    }
}

#[derive(Clone)]
pub(crate) struct NodeRef {
    pub(crate) client: Arc<Mutex<InterfaceClient<Channel>>>,

    pub(crate) name: NodeId,
}

impl Ref<Node> for NodeRef {
    fn name(&self) -> &NodeId {
        return &self.name;
    }

    async fn resolve(&self) -> Result<Node> {
        let info = self
            .client
            .lock()
            .node(NodeInfoRequest {
                name: self.name.0.clone(),
            })
            .await?;

        return Ok(Node::from_node_info(self.client.clone(), info.into_inner()));
    }
}

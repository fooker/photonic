use std::collections::HashSet;
use std::sync::Arc;

use anyhow::Result;
use parking_lot::Mutex;
use tonic::transport::{Channel, Endpoint, Uri};

use photonic_interface_grpc_proto::interface_client::InterfaceClient;
use photonic_interface_grpc_proto::{AttrInfoRequest, AttrName, InputInfoRequest, NodeInfoRequest};

pub use crate::attr::{Attr, AttrId};
pub use crate::input::{Input, InputId};
pub use crate::node::{Node, NodeId};

pub mod attr;
pub mod input;
pub mod node;
pub mod values;

#[cfg(feature = "python")]
mod python;

pub struct Client {
    client: Arc<Mutex<InterfaceClient<Channel>>>,
}

impl Client {
    pub async fn connect(uri: Uri) -> Result<Self> {
        let channel = Endpoint::from(uri).connect_lazy();

        let client = InterfaceClient::new(channel);
        let client = Arc::new(Mutex::new(client));

        return Ok(Self {
            client,
        });
    }

    pub async fn nodes(&self) -> Result<HashSet<NodeId>> {
        let mut client = self.client.lock_arc();

        let nodes = client.nodes(()).await?.into_inner().nodes.into_iter().map(NodeId).collect();

        return Ok(nodes);
    }

    pub async fn inputs(&self) -> Result<HashSet<InputId>> {
        let mut client = self.client.lock_arc();

        let inputs = client.inputs(()).await?.into_inner().inputs.into_iter().map(InputId).collect();

        return Ok(inputs);
    }

    pub async fn root(&self) -> Result<Node> {
        let mut client = self.client.lock_arc();

        let info = client.root(()).await?.into_inner();

        return Ok(Node::from_node_info(self.client.clone(), info));
    }

    pub async fn node(&self, name: &NodeId) -> Result<Node> {
        let mut client = self.client.lock_arc();

        let info = client
            .node(NodeInfoRequest {
                name: name.0.clone(),
            })
            .await?
            .into_inner();

        return Ok(Node::from_node_info(self.client.clone(), info));
    }

    pub async fn attr(&self, name: &AttrId) -> Result<Attr> {
        let mut client = self.client.lock_arc();

        let info = client
            .attr(AttrInfoRequest {
                name: Some(AttrName {
                    node: name.node.0.clone(),
                    path: name.path.clone(),
                }),
            })
            .await?
            .into_inner();

        return Ok(Attr::from_attr_info(self.client.clone(), info));
    }

    pub async fn input(&self, name: &InputId) -> Result<Input> {
        let mut client = self.client.lock_arc();

        let info = client
            .input(InputInfoRequest {
                name: name.0.clone(),
            })
            .await?
            .into_inner();

        return Ok(Input::from_input_info(self.client.clone(), info));
    }
}

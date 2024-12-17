use std::future::Future;
use std::hash::Hash;
use std::sync::Arc;

use anyhow::Result;
use parking_lot::Mutex;
use tonic::transport::{Channel, Endpoint, Uri};

use photonic_interface_grpc_proto::interface_client::InterfaceClient;
use photonic_interface_grpc_proto::{AttrInfoRequest, AttrRef, InputInfoRequest, NodeInfoRequest};

pub use crate::attr::{Attribute, AttributeId};
use crate::input::InputRef;
pub use crate::input::{Input, InputId};
use crate::node::NodeRef;
pub use crate::node::{Node, NodeId};

pub mod attr;
pub mod input;
pub mod node;
pub mod values;

pub trait Identified {
    type Id: Clone + Eq + Hash;

    fn name(&self) -> &Self::Id;
}

pub trait Ref<V: Identified>: Clone {
    fn name(&self) -> &V::Id;

    fn resolve(&self) -> impl Future<Output = Result<V>>;
}

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

    pub async fn root(&self) -> Result<Node> {
        let info = self.client.lock().root(()).await?.into_inner();

        return Ok(Node::from_node_info(self.client.clone(), info));
    }

    pub async fn node(&self, name: &NodeId) -> Result<Node> {
        let info = self
            .client
            .lock()
            .node(NodeInfoRequest {
                name: name.0.clone(),
            })
            .await?
            .into_inner();

        return Ok(Node::from_node_info(self.client.clone(), info));
    }

    pub async fn attribute(&self, node: &NodeId, path: Vec<String>) -> Result<Attribute> {
        let info = self
            .client
            .lock()
            .attr(AttrInfoRequest {
                attr: Some(AttrRef {
                    node: node.0.clone(),
                    path,
                }),
            })
            .await?
            .into_inner();

        return Ok(Attribute::from_attr_info(self.client.clone(), info));
    }

    pub async fn input(&self, name: &InputId) -> Result<Input> {
        let info = self
            .client
            .lock()
            .input(InputInfoRequest {
                name: name.0.clone(),
            })
            .await?
            .into_inner();

        return Ok(Input::from_input_info(self.client.clone(), info));
    }

    pub async fn nodes(&self) -> Result<Vec<impl Ref<Node>>> {
        let nodes = self
            .client
            .lock()
            .nodes(())
            .await?
            .into_inner()
            .nodes
            .into_iter()
            .map(NodeId)
            .map(|name| NodeRef {
                client: self.client.clone(),
                name,
            })
            .collect();

        return Ok(nodes);
    }

    pub async fn inputs(&self) -> Result<Vec<impl Ref<Input>>> {
        let inputs = self
            .client
            .lock()
            .inputs(())
            .await?
            .into_inner()
            .inputs
            .into_iter()
            .map(InputId)
            .map(|name| InputRef {
                client: self.client.clone(),
                name,
            })
            .collect();

        return Ok(inputs);
    }
}

// struct ClientNodes<'c> {
//     client: &'c Client,
//     nodes: HashSet<NodeId>,
// }
//
// impl Map<Node> for ClientNodes<'_> {
//     type Key = NodeId;
//     type Error = anyhow::Error;
//
//     fn keys<'s>(&'s self) -> impl Iterator<Item=&'s NodeId> where NodeId: 's {
//         return self.nodes.iter();
//     }
//
//     fn get<'s>(&'s self, key: &'s NodeId) -> Option<&'s NodeId> {
//         if !self.nodes.contains(key) {
//             return None;
//         }
//
//         return Some(key);
//     }
//
//     async fn resolve(&self, key: &NodeId) -> Result<Option<Node>, Self::Error> {
//         if !self.nodes.contains(key) {
//             return Ok(None);
//         }
//
//         let info = self.client.client.lock()
//             .node(NodeInfoRequest {
//                 name: key.0.clone(),
//             }).await?;
//
//         return Ok(Some(Node::from_node_info(self.client.client.clone(), info.into_inner())));
//     }
// }
//
// struct ClientInputs<'c> {
//     client: &'c Client,
//     inputs: HashSet<InputId>,
// }
//
// impl Map<Input> for ClientInputs<'_> {
//     type Key = InputId;
//     type Error = anyhow::Error;
//
//     fn keys<'s>(&'s self) -> impl Iterator<Item=&'s InputId> where InputId: 's {
//         return self.inputs.iter();
//     }
//
//     fn get<'s>(&'s self, key: &'s InputId) -> Option<&'s InputId> {
//         if !self.inputs.contains(key) {
//             return None;
//         }
//
//         return Some(key);
//     }
//
//     async fn resolve(&self, key: &InputId) -> Result<Option<Input>, Self::Error> {
//         if !self.inputs.contains(key) {
//             return Ok(None);
//         }
//
//         let info = self.client.client.lock()
//             .input(InputInfoRequest {
//                 name: key.0.clone(),
//             }).await?;
//
//         return Ok(Some(Input::from_input_info(self.client.client.clone(), info.into_inner())));
//     }
// }

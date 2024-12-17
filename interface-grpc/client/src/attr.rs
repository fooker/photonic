use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use anyhow::Result;
use parking_lot::Mutex;
use tonic::transport::Channel;

use photonic_interface_grpc_proto::interface_client::InterfaceClient;
use photonic_interface_grpc_proto::{AttrInfoRequest, AttrInfoResponse, AttrRef};

use crate::input::{Input, InputId, InputRef};
use crate::node::NodeId;
use crate::{Identified, Ref};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct AttributeId {
    pub(crate) node: NodeId,
    pub(crate) path: Vec<String>,
}

impl fmt::Display for AttributeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}@{}", &self.node, &self.path.join("/"));
    }
}

impl AttributeId {
    pub(crate) fn extend(self, attr: String) -> Self {
        let mut path = self.path;
        path.push(attr);

        return Self {
            path,
            ..self
        };
    }

    pub fn node(&self) -> &NodeId {
        return &self.node;
    }

    pub fn path(&self) -> &[String] {
        return self.path.as_slice();
    }
}

pub struct Attribute {
    #[allow(dead_code)]
    client: Arc<Mutex<InterfaceClient<Channel>>>,

    name: AttributeId,

    kind: String,

    value_type: String,

    attrs: HashMap<String, AttributeRef>,
    inputs: HashMap<String, InputRef>,
}

impl Identified for Attribute {
    type Id = AttributeId;

    fn name(&self) -> &Self::Id {
        return &self.name;
    }
}

impl Attribute {
    pub(crate) fn from_attr_info(client: Arc<Mutex<InterfaceClient<Channel>>>, info: AttrInfoResponse) -> Self {
        let name = info.attr.expect("name must exist");
        let name = AttributeId {
            node: NodeId(name.node),
            path: name.path,
        };

        let value_type = info.value_type;

        let attrs = info
            .attrs
            .into_iter()
            .map(|k| {
                (k.clone(), AttributeRef {
                    client: client.clone(),
                    name: name.clone().extend(k),
                })
            })
            .collect();

        let inputs = info
            .inputs
            .into_iter()
            .map(|(k, v)| {
                (k, InputRef {
                    client: client.clone(),
                    name: InputId(v),
                })
            })
            .collect();

        Self {
            client,
            name,
            kind: info.kind,
            value_type,
            attrs,
            inputs,
        }
    }

    pub fn kind(&self) -> &str {
        return &self.kind;
    }

    pub fn value_type(&self) -> &str {
        return &self.value_type;
    }

    pub fn attrs(&self) -> &HashMap<String, impl Ref<Attribute>> {
        return &self.attrs;
    }

    pub fn inputs(&self) -> &HashMap<String, impl Ref<Input>> {
        return &self.inputs;
    }
}

#[derive(Clone)]
pub(crate) struct AttributeRef {
    pub(crate) client: Arc<Mutex<InterfaceClient<Channel>>>,

    pub(crate) name: AttributeId,
}

impl Ref<Attribute> for AttributeRef {
    fn name(&self) -> &AttributeId {
        return &self.name;
    }

    async fn resolve(&self) -> Result<Attribute> {
        let info = self
            .client
            .lock()
            .attr(AttrInfoRequest {
                attr: Some(AttrRef {
                    node: self.name.node.0.clone(),
                    path: self.name.path.clone(),
                }),
            })
            .await?;

        return Ok(Attribute::from_attr_info(self.client.clone(), info.into_inner()));
    }
}

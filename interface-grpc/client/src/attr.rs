use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::Arc;

use anyhow::Result;
use parking_lot::Mutex;
use tonic::transport::Channel;

use photonic_interface_grpc_proto::interface_client::InterfaceClient;
use photonic_interface_grpc_proto::{AttrInfoRequest, AttrInfoResponse, AttrName, InputInfoRequest};

use crate::node::NodeId;
use crate::{Input, InputId};

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct AttrId {
    pub(crate) node: NodeId,
    pub(crate) path: Vec<String>,
}

impl fmt::Display for AttrId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}@{}", &self.node.0, &self.path.join("/"));
    }
}

impl fmt::Debug for AttrId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}@{}", &self.node.0, &self.path.join("/"));
    }
}

impl AttrId {
    pub fn new(node: NodeId, path: impl IntoIterator<Item = String>) -> Self {
        let path = path.into_iter().collect();

        return Self {
            node,
            path,
        };
    }

    pub fn extend(self, attr: String) -> Self {
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

pub struct Attr {
    client: Arc<Mutex<InterfaceClient<Channel>>>,

    name: AttrId,

    kind: String,

    value_type: String,

    attrs: HashSet<AttrId>,
    inputs: HashMap<String, InputId>,
}

impl Attr {
    pub(crate) fn from_attr_info(client: Arc<Mutex<InterfaceClient<Channel>>>, info: AttrInfoResponse) -> Self {
        let name = info.attr.expect("name must exist");
        let name = AttrId {
            node: NodeId(name.node),
            path: name.path,
        };

        let value_type = info.value_type;

        let attrs = info.attrs.into_iter().map(|attr| name.clone().extend(attr)).collect();

        let inputs = info.inputs.into_iter().map(|(key, input)| (key, InputId(input))).collect();

        Self {
            client,
            name,
            kind: info.kind,
            value_type,
            attrs,
            inputs,
        }
    }

    pub fn name(&self) -> &AttrId {
        return &self.name;
    }

    pub fn kind(&self) -> &str {
        return &self.kind;
    }

    pub fn value_type(&self) -> &str {
        return &self.value_type;
    }

    pub fn attrs(&self) -> &HashSet<AttrId> {
        return &self.attrs;
    }

    pub fn inputs(&self) -> &HashMap<String, InputId> {
        return &self.inputs;
    }

    pub async fn attr(&self, name: &str) -> Result<Option<Attr>> {
        let mut client = self.client.lock_arc();

        let attr = self.name.clone().extend(name.to_owned());
        if !self.attrs.contains(&attr) {
            return Ok(None);
        }

        let response = client
            .attr(AttrInfoRequest {
                name: Some(AttrName {
                    node: attr.node.0,
                    path: attr.path,
                }),
            })
            .await?
            .into_inner();

        return Ok(Some(Attr::from_attr_info(self.client.clone(), response)));
    }

    pub async fn input(&self, name: &str) -> Result<Option<Input>> {
        let mut client = self.client.lock_arc();

        let Some(input) = self.inputs.get(name) else {
            return Ok(None);
        };

        let response = client
            .input(InputInfoRequest {
                name: input.0.clone(),
            })
            .await?
            .into_inner();

        return Ok(Some(Input::from_input_info(self.client.clone(), response)));
    }
}

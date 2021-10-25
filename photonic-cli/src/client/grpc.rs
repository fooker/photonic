use anyhow::Error;
use async_trait::async_trait;

use photonic_grpc_proto as proto;

use crate::client::{AttrInfo, AttrValueType, NodeInfo};
use crate::SendValue;

pub struct GrpcClient {
    client: proto::interface_client::InterfaceClient<tonic::transport::Channel>,
}

impl From<proto::NodeInfo> for NodeInfo {
    fn from(proto: proto::NodeInfo) -> Self {
        let attrs = proto.attrs.into_iter().map(|(key, attr)| (key, attr.into())).collect();

        return NodeInfo {
            name: proto.name,
            kind: proto.kind,
            nodes: proto.nodes,
            attrs,
        };
    }
}

impl From<proto::AttrInfo> for AttrInfo {
    fn from(proto: proto::AttrInfo) -> Self {
        let value_type = proto.value_type().into();

        let attrs = proto.attrs.into_iter().map(|(key, attr)| (key, attr.into())).collect();

        return AttrInfo {
            kind: proto.kind,
            value_type,
            attrs,
            inputs: proto.inputs,
        };
    }
}

impl From<proto::attr_info::ValueType> for AttrValueType {
    fn from(proto: proto::attr_info::ValueType) -> Self {
        return match proto {
            proto::attr_info::ValueType::Boolean => AttrValueType::Boolean,
            proto::attr_info::ValueType::Integer => AttrValueType::Integer,
            proto::attr_info::ValueType::Decimal => AttrValueType::Decimal,
            proto::attr_info::ValueType::Color => AttrValueType::Color,
            proto::attr_info::ValueType::Range => AttrValueType::Range(&AttrValueType::Color), // TODO: This is fake
        };
    }
}

#[async_trait]
impl super::Client for GrpcClient {
    async fn connect(cfg: String) -> Result<Self, Error> {
        let client = proto::interface_client::InterfaceClient::connect(cfg).await?;
        return Ok(Self {
            client,
        });
    }

    async fn nodes(&mut self) -> Result<Vec<String>, Error> {
        let response = self.client.node_list(proto::NodeListRequest {}).await?;

        let list = response.into_inner();

        return Ok(list.names);
    }

    async fn node(&mut self, name: Option<String>) -> Result<Option<NodeInfo>, Error> {
        let response = self
            .client
            .node_info(proto::NodeInfoRequest {
                name: name.clone(),
            })
            .await?;

        let node = response.into_inner().node;

        return Ok(node.map(Into::into));
    }

    async fn send(&mut self, name: String, value: SendValue) -> Result<(), Error> {
        let value = match value {
            SendValue::Trigger => proto::input_send_request::Value::Trigger(proto::TriggerValue {}),
            SendValue::Boolean {
                value,
            } => proto::input_send_request::Value::Boolean(proto::BooleanValue {
                value,
            }),
            SendValue::Integer {
                value,
            } => proto::input_send_request::Value::Integer(proto::IntegerValue {
                value,
            }),
            SendValue::Decimal {
                value,
            } => proto::input_send_request::Value::Decimal(proto::DecimalValue {
                value,
            }),
        };

        self.client
            .input_send(proto::InputSendRequest {
                name,
                value: Some(value),
            })
            .await?;

        return Ok(());
    }
}

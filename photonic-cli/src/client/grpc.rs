use async_trait::async_trait;
use anyhow::Error;

use photonic_grpc_proto as proto;
use photonic_grpc_proto::attr_info::ValueType;

use crate::client::{AttrInfo, AttrValueType, NodeInfo};
use crate::SendValue;

pub struct GrpcClient {
    client: proto::interface_client::InterfaceClient<tonic::transport::Channel>,
}

impl Into<NodeInfo> for proto::NodeInfo {
    fn into(self) -> NodeInfo {
        let attrs = self.attrs.into_iter()
            .map(|(key, attr)| (key, attr.into()))
            .collect();

        return NodeInfo {
            name: self.name,
            kind: self.kind,
            nodes: self.nodes,
            attrs,
        };
    }
}

impl Into<AttrInfo> for proto::AttrInfo {
    fn into(self) -> AttrInfo {
        let value_type = self.value_type().into();

        let attrs = self.attrs.into_iter()
            .map(|(key, attr)| (key, attr.into()))
            .collect();

        return AttrInfo {
            kind: self.kind,
            value_type,
            attrs,
            inputs: self.inputs,
        };
    }
}

impl Into<AttrValueType> for proto::attr_info::ValueType {
    fn into(self) -> AttrValueType {
        return match self {
            Self::Boolean => AttrValueType::Boolean,
            Self::Integer => AttrValueType::Integer,
            Self::Decimal => AttrValueType::Decimal,
            Self::Color => AttrValueType::Color,
            Self::Range => AttrValueType::Range(&AttrValueType::Color), // TODO: This is fake
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
        let response = self.client.node_info(proto::NodeInfoRequest {
            name: name.clone(),
        }).await?;

        let node = response.into_inner().node;

        return Ok(node.map(Into::into));
    }

    async fn send(&mut self, name: String, value: SendValue) -> Result<(), Error> {
        let value = match value {
            SendValue::Trigger => proto::input_send_request::Value::Trigger(proto::TriggerValue {}),
            SendValue::Boolean { value } => proto::input_send_request::Value::Boolean(proto::BooleanValue { value }),
            SendValue::Integer { value } => proto::input_send_request::Value::Integer(proto::IntegerValue { value }),
            SendValue::Decimal { value } => proto::input_send_request::Value::Decimal(proto::DecimalValue { value }),
        };

        self.client.input_send(proto::InputSendRequest {
            name,
            value: Some(value),
        }).await?;

        return Ok(());
    }
}
